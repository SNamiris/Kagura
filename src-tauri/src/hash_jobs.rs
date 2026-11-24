use crate::desktop::{display, ApiError, Reply};
use kagura_core::index::{HashJobProgress, Index, DEFAULT_HASH_THREADS};
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard,
    },
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, Runtime};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "lowercase")]
pub(crate) enum HashPhase {
    #[default]
    Idle,
    Running,
    Pausing,
    Paused,
    Cancelling,
    Cancelled,
    Complete,
    Failed,
}

impl HashPhase {
    fn active(self) -> bool {
        matches!(self, Self::Running | Self::Pausing | Self::Cancelling)
    }
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct HashStatus {
    pub root: Option<String>,
    pub generation: u32,
    pub phase: HashPhase,
    pub threads: usize,
    #[cfg_attr(test, ts(type = "number"))]
    pub total_bytes: u64,
    #[cfg_attr(test, ts(type = "number"))]
    pub completed_bytes: u64,
    #[cfg_attr(test, ts(type = "number"))]
    pub cached_bytes: u64,
    pub total_files: usize,
    pub completed_files: usize,
    pub cached_files: usize,
    pub bytes_per_second: f64,
    pub eta_seconds: Option<f64>,
    pub error: Option<String>,
}

impl Default for HashStatus {
    fn default() -> Self {
        Self {
            root: None,
            generation: 0,
            phase: HashPhase::Idle,
            threads: DEFAULT_HASH_THREADS,
            total_bytes: 0,
            completed_bytes: 0,
            cached_bytes: 0,
            total_files: 0,
            completed_files: 0,
            cached_files: 0,
            bytes_per_second: 0.,
            eta_seconds: None,
            error: None,
        }
    }
}
impl HashStatus {
    fn counts(&mut self, p: &HashJobProgress) {
        self.total_bytes = p.total_bytes;
        self.completed_bytes = p.completed_bytes;
        self.cached_bytes = p.cached_bytes;
        self.total_files = p.total_files;
        self.completed_files = p.completed_files;
        self.cached_files = p.cached_files;
        self.bytes_per_second = p.bytes_per_second;
        self.eta_seconds = p.eta_seconds;
    }
}

#[derive(Default)]
struct State {
    status: HashStatus,
    stop: Arc<AtomicBool>,
}
#[derive(Default, Clone)]
pub(crate) struct HashJobs(Arc<Mutex<State>>);

impl HashJobs {
    fn lock(&self) -> Reply<MutexGuard<'_, State>> {
        self.0
            .lock()
            .map_err(|_| ApiError::new("worker_error", "无法读取哈希任务状态"))
    }
    pub fn status(&self) -> Reply<HashStatus> {
        Ok(self.lock()?.status.clone())
    }

    /// Preserve an active job on a same-library rescan. A switch invalidates its token.
    pub fn inventory(&self, root: &Path, coverage: &HashJobProgress) -> Reply<()> {
        let root = display(root);
        let mut job = self.lock()?;
        if job.status.root.as_deref() != Some(&root) {
            job.stop.store(true, Ordering::Release);
            job.status = HashStatus {
                root: Some(root),
                generation: job.status.generation.wrapping_add(1),
                ..Default::default()
            };
        }
        if !job.status.phase.active() {
            job.status.counts(coverage);
            if coverage.completed_files == coverage.total_files && coverage.total_files > 0 {
                job.status.phase = HashPhase::Complete;
            } else if job.status.phase == HashPhase::Complete {
                job.status.phase = HashPhase::Idle;
            }
        }
        Ok(())
    }

    pub fn stop<R: Runtime>(&self, app: &AppHandle<R>, cancel: bool) -> Reply<HashStatus> {
        let mut job = self.lock()?;
        job.status.phase = match (job.status.phase, cancel) {
            (HashPhase::Running, false) => HashPhase::Pausing,
            (HashPhase::Running | HashPhase::Pausing, true) => HashPhase::Cancelling,
            (HashPhase::Paused, true) => HashPhase::Cancelled,
            (other, _) => other,
        };
        if matches!(job.status.phase, HashPhase::Pausing | HashPhase::Cancelling) {
            job.stop.store(true, Ordering::Release);
        }
        let _ = app.emit("hash-progress", &job.status);
        Ok(job.status.clone())
    }

    pub fn start<R: Runtime>(
        &self,
        app: AppHandle<R>,
        root: PathBuf,
        db: PathBuf,
        threads: usize,
    ) -> Reply<HashStatus> {
        if !(1..=4).contains(&threads) {
            return Err(ApiError::new("invalid_argument", "线程数须在 1～4 之间"));
        }
        let (initial, stop) = {
            let mut job = self.lock()?;
            if job.status.phase.active() {
                return Err(ApiError::new("hash_busy", "哈希任务正在运行或停止中"));
            }
            let stop = Arc::new(AtomicBool::new(false));
            job.stop = stop.clone();
            job.status.generation = job.status.generation.wrapping_add(1);
            job.status.phase = HashPhase::Running;
            job.status.threads = threads;
            job.status.error = None;
            let _ = app.emit("hash-progress", &job.status);
            (job.status.clone(), stop)
        };
        let jobs = self.clone();
        let generation = initial.generation;
        tauri::async_runtime::spawn_blocking(move || {
            let mut last = None::<Instant>;
            let result = Index::open_for_library(db, &root).and_then(|mut index| {
                index.hash_all_parallel(
                    &root,
                    threads,
                    || !stop.load(Ordering::Acquire),
                    |progress| {
                        if last.is_some_and(|t| t.elapsed() < Duration::from_millis(250)) {
                            return;
                        }
                        if let Ok(mut job) = jobs.lock() {
                            if job.status.generation == generation {
                                job.status.counts(&progress);
                                let _ = app.emit("hash-progress", &job.status);
                            }
                        }
                        last = Some(Instant::now());
                    },
                )
            });
            if let Ok(mut job) = jobs.lock() {
                if job.status.generation != generation {
                    return;
                }
                match result {
                    Ok(progress) => {
                        job.status.counts(&progress);
                        job.status.phase = if !progress.stopped {
                            HashPhase::Complete
                        } else if job.status.phase == HashPhase::Pausing {
                            HashPhase::Paused
                        } else {
                            HashPhase::Cancelled
                        };
                    }
                    Err(e) => {
                        job.status.phase = HashPhase::Failed;
                        job.status.error = Some(e.to_string());
                    }
                }
                job.status.bytes_per_second = 0.;
                job.status.eta_seconds = None;
                let _ = app.emit("hash-progress", &job.status);
            }
        });
        Ok(initial)
    }
}
