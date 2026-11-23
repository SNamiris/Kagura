use std::{env, error::Error, fs, io, path::PathBuf, time::Instant};

use kagura_core::{index::Index, scan::scan_library_for_check};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let usage = || io::Error::other("usage: build_index <MMD root> [index.sqlite]");
    let root = PathBuf::from(args.next().ok_or_else(usage)?);
    let path = match args.next() {
        Some(path) => PathBuf::from(path),
        None => Index::default_path()?,
    };
    if args.next().is_some() {
        return Err(usage().into());
    }

    let start = Instant::now();
    let mut index = Index::open_for_library(&path, &root)?;
    let scan = scan_library_for_check(&root);
    let scan_time = start.elapsed();
    let changes = index.rebuild(&scan)?;
    let total = start.elapsed();
    let stats = index.stats()?;
    drop(index); // Closing the last connection checkpoints the WAL.
    println!("SQLite {} · {}", rusqlite::version(), path.display());
    println!(
        "rebuild {:.3}s (scan {:.3}s included)",
        total.as_secs_f64(),
        scan_time.as_secs_f64()
    );
    println!("assets {:?}, files {}", stats.counts, changes.files_changed);
    println!("database {} bytes", fs::metadata(&path)?.len());
    Ok(())
}
