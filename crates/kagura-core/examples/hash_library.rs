//! Hash asset payloads into the current cache, resuming any earlier run.
use kagura_core::index::{Index, DEFAULT_HASH_THREADS};
use std::{env, error::Error, path::PathBuf, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let root = PathBuf::from(
        args.next()
            .ok_or("usage: hash_library <root> [threads] [index.sqlite]")?,
    );
    let threads = args
        .next()
        .map(|s| s.to_string_lossy().parse::<usize>())
        .transpose()?
        .unwrap_or(DEFAULT_HASH_THREADS);
    if !(1..=4).contains(&threads) {
        return Err("threads must be 1–4".into());
    }
    let db = args
        .next()
        .map(PathBuf::from)
        .map(Ok)
        .unwrap_or_else(Index::default_path)?;
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let start = Instant::now();
    let mut index = Index::open_for_library(&db, &root)?;
    index.update(&root)?;
    println!(
        "inventory_seconds={:.3} threads={threads} index={}",
        start.elapsed().as_secs_f64(),
        db.display()
    );
    let start = Instant::now();
    let mut next = 0.;
    let result = index.hash_all_parallel(
        &root,
        threads,
        || true,
        |p| {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed >= next {
                println!(
                    "elapsed={elapsed:.1}s bytes={}/{} files={}/{} MiB/s={:.2} eta_seconds={:.1}",
                    p.completed_bytes,
                    p.total_bytes,
                    p.completed_files,
                    p.total_files,
                    p.bytes_per_second / 1048576.,
                    p.eta_seconds.unwrap_or(0.)
                );
                next = elapsed + 5.;
            }
        },
    )?;
    println!("hash_seconds={:.3} total_files={} total_bytes={} completed_files={} completed_bytes={} cached_files={} cached_bytes={} MiB/s={:.2}",
        start.elapsed().as_secs_f64(),result.total_files,result.total_bytes,result.completed_files,result.completed_bytes,
        result.cached_files,result.cached_bytes,result.bytes_per_second/1048576.);
    Ok(())
}
