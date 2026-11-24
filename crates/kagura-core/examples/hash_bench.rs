//! Balanced file-read benchmark. The nine runs use disjoint inventory rows: no file is read twice.
use std::{
    env,
    error::Error,
    fs,
    io::{self, Read},
    path::{Component, Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
    thread,
    time::Instant,
};

use kagura_core::{index::Index, scan::is_reparse};
use rusqlite::{Connection, OpenFlags};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const MIB: u64 = 1024 * 1024;

// Windows unbuffered reads require sector-aligned addresses and request lengths.
#[repr(align(4096))]
struct ReadBuffer([u8; 256 * 1024]);

struct FileRow {
    rel: String,
    size: u64,
}

fn sample(root: &Path, database: &Path, bytes_per_run: u64) -> Result<[Vec<FileRow>; 9]> {
    let db = Connection::open_with_flags(database, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let stored: String = db.query_row("SELECT root FROM meta WHERE id=1", [], |r| r.get(0))?;
    if Path::new(&stored).canonicalize()? != root {
        return Err("the cache belongs to a different library".into());
    }
    let mut query = db.prepare(
        "SELECT a.rel_dir||'/'||f.rel, f.size FROM files f JOIN assets a ON a.id=f.asset_id
        WHERE f.size BETWEEN 1 AND 67108864 ORDER BY f.size,a.rel_dir,f.rel",
    )?;
    let rows = query
        .query_map([], |r| {
            Ok(FileRow {
                rel: r.get(0)?,
                size: r.get::<_, i64>(1)? as u64,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let eligible: u64 = rows.iter().map(|r| r.size).sum();
    // Adjacent rows have nearly the same size. Sample regularly across size strata and
    // rotate the nine positions so slightly larger members do not favor a thread count.
    let groups = rows.len() / 9;
    if groups == 0 {
        return Err("benchmark needs at least nine eligible files".into());
    }
    let selected =
        ((bytes_per_run as f64 * 9. / eligible as f64 * groups as f64).round() as usize).clamp(1, groups);
    let mut bins: [Vec<FileRow>; 9] = std::array::from_fn(|_| Vec::new());
    let mut rows = rows.into_iter();
    let mut bucket = 0;
    for group in 0..groups {
        let pick = bucket < selected && group == ((2 * bucket + 1) * groups) / (2 * selected);
        for slot in 0..9 {
            let row = rows.next().expect("complete group of nine");
            if pick {
                bins[(slot + bucket) % 9].push(row);
            }
        }
        if pick {
            bucket += 1;
        }
    }
    for bin in &mut bins {
        bin.sort_by(|a, b| a.rel.cmp(&b.rel));
    }
    Ok(bins)
}

fn read_one(root: &Path, row: &FileRow, buf: &mut ReadBuffer, uncached: bool) -> io::Result<()> {
    let mut path = root.to_owned();
    if !row.rel.starts_with("Library/") {
        return Err(io::Error::other("path outside Library"));
    }
    for component in Path::new(&row.rel).components() {
        let Component::Normal(c) = component else {
            return Err(io::Error::other("invalid inventory path"));
        };
        path.push(c);
        if is_reparse(&fs::symlink_metadata(&path)?) {
            return Err(io::Error::other("inventory path became a reparse point"));
        }
    }
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    if uncached {
        use std::os::windows::fs::OpenOptionsExt;
        options.custom_flags(0x2000_0000); // FILE_FLAG_NO_BUFFERING
    }
    #[cfg(not(windows))]
    if uncached {
        return Err(io::Error::other("--uncached requires Windows"));
    }
    let mut file = options.open(&path)?;
    let mut digest = blake3::Hasher::new();
    // A final sector may return a short read up to logical EOF. Do not request an
    // additional sector from the resulting non-aligned offset.
    while digest.count() < row.size {
        let n = file.read(&mut buf.0)?;
        if n == 0 {
            break;
        }
        digest.update(&buf.0[..n]);
    }
    if digest.count() != row.size {
        return Err(io::Error::other(format!("inventory changed: {}", path.display())));
    }
    std::hint::black_box(digest.finalize());
    Ok(())
}

fn timed_read(root: &Path, rows: &[FileRow], threads: usize, uncached: bool) -> Result<f64> {
    let next = AtomicUsize::new(0);
    let error = Mutex::new(None);
    let start = Instant::now();
    thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                let mut buf = ReadBuffer([0; 256 * 1024]);
                while let Some(row) = rows.get(next.fetch_add(1, Ordering::Relaxed)) {
                    if let Err(e) = read_one(root, row, &mut buf, uncached) {
                        *error.lock().expect("benchmark mutex") = Some(e);
                        break;
                    }
                }
            });
        }
    });
    if let Some(e) = error.into_inner()? {
        return Err(e.into());
    }
    Ok(start.elapsed().as_secs_f64())
}

fn main() -> Result<()> {
    let mut args = env::args_os().skip(1).peekable();
    let uncached = args.next_if(|a| a == "--uncached").is_some();
    let root = PathBuf::from(
        args.next()
            .ok_or("usage: hash_bench [--uncached] <root> [index.sqlite] [MiB per run]")?,
    )
    .canonicalize()?;
    let db = args
        .next()
        .map(PathBuf::from)
        .map(Ok)
        .unwrap_or_else(Index::default_path)?;
    let mib = args
        .next()
        .map(|s| s.to_string_lossy().parse::<u64>())
        .transpose()?
        .unwrap_or(768);
    if args.next().is_some() || mib == 0 || mib > 65536 {
        return Err("invalid benchmark parameters".into());
    }
    let bins = sample(&root, &db, mib * MIB)?;
    println!("uncached={uncached} MiB_per_trial={mib}");
    let mut totals = [(0u64, 0f64); 3];
    for (i, rows) in bins.iter().enumerate() {
        let lane = (i / 3 + i % 3) % 3;
        let threads = 1 << lane;
        let bytes: u64 = rows.iter().map(|r| r.size).sum();
        let seconds = timed_read(&root, rows, threads, uncached)?;
        println!(
            "round={} threads={threads} files={} bytes={bytes} seconds={seconds:.3} MiB/s={:.2}",
            i / 3 + 1,
            rows.len(),
            bytes as f64 / MIB as f64 / seconds
        );
        totals[lane].0 += bytes;
        totals[lane].1 += seconds;
    }
    for (i, (bytes, seconds)) in totals.iter().enumerate() {
        println!(
            "total threads={} bytes={bytes} seconds={seconds:.3} MiB/s={:.2}",
            1 << i,
            *bytes as f64 / MIB as f64 / seconds
        );
    }
    Ok(())
}
