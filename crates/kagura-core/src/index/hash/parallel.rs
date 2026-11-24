use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        mpsc::{self, RecvTimeoutError},
    },
    thread,
    time::{Duration, Instant},
};

use rusqlite::Connection;
use serde::Serialize;

use super::{
    checked_path, invalid, read_digest, stamp, store_digests, DigestUpdate, Index, IndexError, BUFFER_SIZE,
};

pub const DEFAULT_HASH_THREADS: usize = 4;
const MAX_THREADS: usize = 4;
const TICK: Duration = Duration::from_millis(200);
const COMMIT_FILES: usize = 64;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct HashJobProgress {
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub total_bytes: u64,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub completed_bytes: u64,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub cached_bytes: u64,
    pub total_files: usize,
    pub completed_files: usize,
    pub cached_files: usize,
    pub bytes_per_second: f64,
    pub eta_seconds: Option<f64>,
    pub stopped: bool,
}

struct Job {
    id: String,
    dir: String,
    rel: String,
    stamp: (i64, i64),
    cached: bool,
}

impl Job {
    fn read(
        &self,
        root: &Path,
        buffer: &mut [u8],
        keep: &impl Fn() -> bool,
        bytes: &AtomicU64,
    ) -> Result<Option<DigestUpdate<'_>>, IndexError> {
        let (path, meta) = checked_path(root, &self.dir, &self.rel)?;
        let current = stamp(&meta)?;
        if self.cached && self.stamp == current {
            return Ok(None);
        }
        let digest = read_digest(&path, current, buffer, keep, |n| {
            bytes.fetch_add(n, Ordering::Relaxed);
        })?;
        Ok(digest.map(|digest| DigestUpdate {
            id: &self.id,
            dir: &self.dir,
            rel: &self.rel,
            old_stamp: self.stamp,
            new_stamp: current,
            digest,
        }))
    }
}

fn coverage(db: &Connection) -> Result<HashJobProgress, IndexError> {
    Ok(db.query_row(
        "SELECT count(*), coalesce(sum(size), 0), count(blake3),
                coalesce(sum(CASE WHEN blake3 IS NOT NULL THEN size END), 0) FROM files",
        [],
        |r| {
            let files = r.get::<_, i64>(2)? as usize;
            let bytes = r.get::<_, i64>(3)? as u64;
            Ok(HashJobProgress {
                total_files: r.get::<_, i64>(0)? as usize,
                total_bytes: r.get::<_, i64>(1)? as u64,
                completed_files: files,
                cached_files: files,
                completed_bytes: bytes,
                cached_bytes: bytes,
                ..Default::default()
            })
        },
    )?)
}

struct Batch<'a> {
    updates: Vec<DigestUpdate<'a>>,
    previously_cached: usize,
    previously_cached_bytes: u64,
}

impl<'a> Batch<'a> {
    fn new() -> Self {
        Self {
            updates: Vec::with_capacity(COMMIT_FILES),
            previously_cached: 0,
            previously_cached_bytes: 0,
        }
    }
    fn push(&mut self, update: DigestUpdate<'a>, cached: bool) {
        if cached {
            self.previously_cached += 1;
            self.previously_cached_bytes += update.old_stamp.0 as u64;
        }
        self.updates.push(update);
    }
    fn commit(
        &mut self,
        conn: &mut Connection,
        root: &str,
        state: &mut HashJobProgress,
    ) -> Result<(), IndexError> {
        store_digests(conn, root, &self.updates)?;
        state.cached_files -= self.previously_cached;
        state.cached_bytes -= self.previously_cached_bytes;
        state.completed_files += self.updates.len() - self.previously_cached;
        state.completed_bytes -= self.previously_cached_bytes;
        for u in self.updates.drain(..) {
            state.total_bytes = state.total_bytes - u.old_stamp.0 as u64 + u.new_stamp.0 as u64;
            state.completed_bytes += u.new_stamp.0 as u64;
        }
        self.previously_cached = 0;
        self.previously_cached_bytes = 0;
        Ok(())
    }
}

fn metrics(
    mut state: HashJobProgress,
    bytes_read: u64,
    start: Instant,
    final_state: bool,
) -> HashJobProgress {
    state.bytes_per_second = bytes_read as f64 / start.elapsed().as_secs_f64().max(f64::EPSILON);
    if !final_state {
        state.completed_bytes = (state.cached_bytes + bytes_read).min(state.total_bytes);
    }
    if state.bytes_per_second > 0.0 && !state.stopped {
        state.eta_seconds = Some((state.total_bytes - state.completed_bytes) as f64 / state.bytes_per_second);
    }
    state
}

impl Index {
    /// Indexed coverage, including reusable results left by earlier runs.
    pub fn hash_coverage(&self) -> Result<HashJobProgress, IndexError> {
        coverage(&self.conn)
    }

    /// Hash indexed payloads on up to four independent file readers. Call update after inventory changes.
    /// Returning false from keep_running pauses/cancels this run. Finished files are durable; call again
    /// with the same library to resume. Incomplete files restart at their first byte.
    pub fn hash_all_parallel(
        &mut self,
        root: &Path,
        threads: usize,
        keep_running: impl Fn() -> bool + Sync,
        mut progress: impl FnMut(HashJobProgress),
    ) -> Result<HashJobProgress, IndexError> {
        if !(1..=MAX_THREADS).contains(&threads) {
            return Err(invalid("哈希线程数应为 1～4"));
        }
        let tx = self.conn.transaction()?;
        let root = super::super::detail::active_root(&tx, root)?;
        let root_text = root.to_str().ok_or_else(|| invalid("库根路径不是有效 Unicode"))?;
        let jobs = tx
            .prepare(
                "SELECT a.id,a.rel_dir,f.rel,f.size,f.mtime,f.blake3 IS NOT NULL
             FROM files f JOIN assets a ON a.id=f.asset_id ORDER BY a.rel_dir,f.rel",
            )?
            .query_map([], |r| {
                Ok(Job {
                    id: r.get(0)?,
                    dir: r.get(1)?,
                    rel: r.get(2)?,
                    stamp: (r.get(3)?, r.get(4)?),
                    cached: r.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let mut state = coverage(&tx)?;
        tx.commit()?;
        progress(state);
        let bytes = AtomicU64::new(0);
        let next = AtomicUsize::new(0);
        let stop = AtomicBool::new(false);
        let start = Instant::now();
        let keep = || !stop.load(Ordering::Relaxed) && keep_running();
        thread::scope(|scope| -> Result<(), IndexError> {
            let (sender, receiver) = mpsc::sync_channel(COMMIT_FILES);
            for _ in 0..threads.min(jobs.len()) {
                let sender = sender.clone();
                let jobs = &jobs;
                let root = &root;
                let next = &next;
                let bytes = &bytes;
                let keep = &keep;
                scope.spawn(move || {
                    let mut buffer = vec![0; BUFFER_SIZE];
                    while keep() {
                        let Some(job) = jobs.get(next.fetch_add(1, Ordering::Relaxed)) else {
                            break;
                        };
                        match job.read(root, &mut buffer, keep, bytes) {
                            Ok(Some(d)) => {
                                if sender.send(Ok((d, job.cached))).is_err() {
                                    break;
                                }
                            }
                            Ok(None) => (),
                            Err(e) => {
                                let _ = sender.send(Err(invalid(format!("{}/{}: {e}", job.dir, job.rel))));
                                break;
                            }
                        }
                    }
                });
            }
            drop(sender);
            let mut batch = Batch::new();
            let mut last = Instant::now();
            let mut failure = None;
            loop {
                match receiver.recv_timeout(TICK) {
                    Ok(Ok((update, cached))) => batch.push(update, cached),
                    Ok(Err(e)) => {
                        stop.store(true, Ordering::Relaxed);
                        failure.get_or_insert(e);
                    }
                    Err(RecvTimeoutError::Disconnected) => break,
                    Err(RecvTimeoutError::Timeout) => (),
                }
                if batch.updates.len() >= COMMIT_FILES || last.elapsed() >= TICK {
                    if let Err(e) = batch.commit(&mut self.conn, root_text, &mut state) {
                        stop.store(true, Ordering::Relaxed);
                        return Err(e);
                    }
                    progress(metrics(state, bytes.load(Ordering::Relaxed), start, false));
                    last = Instant::now();
                }
            }
            batch.commit(&mut self.conn, root_text, &mut state)?;
            if let Some(e) = failure {
                return Err(e);
            }
            Ok(())
        })?;
        let tx = self.conn.transaction()?;
        super::super::detail::active_root(&tx, &root)?;
        let actual = coverage(&tx)?;
        tx.commit()?;
        state.stopped = !keep_running();
        if !state.stopped && actual.completed_files != actual.total_files {
            return Err(invalid("哈希运行期间文件清单已更新，请继续计算剩余文件"));
        }
        state.total_files = actual.total_files;
        state.total_bytes = actual.total_bytes;
        state.completed_files = actual.completed_files;
        state.completed_bytes = actual.completed_bytes;
        state = metrics(state, bytes.load(Ordering::Relaxed), start, true);
        progress(state);
        Ok(state)
    }
}
