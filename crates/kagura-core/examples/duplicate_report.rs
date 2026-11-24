//! Summarize existing cached hashes without touching payload files or starting another hash job.
use kagura_core::index::dedup::{DedupOptions, DedupSection, DuplicateReport};
use serde_json::json;
use std::{
    env,
    error::Error,
    io::{self, Write},
    path::PathBuf,
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let path = PathBuf::from(
        args.next()
            .ok_or("usage: duplicate_report <index.sqlite> [--include-excluded]")?,
    );
    let options = match args.next() {
        Some(arg) if arg == "--include-excluded" => DedupOptions {
            include_small: true,
            include_toon: true,
            include_accessories: true,
        },
        None => DedupOptions::default(),
        _ => return Err("unexpected argument".into()),
    };
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let start = Instant::now();
    let report = DuplicateReport::read_cache(&path, options)?;
    let examples = report.page(DedupSection::Identical, 0, 10).asset_groups;
    let summary = &report.summary;
    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);
    serde_json::to_writer_pretty(
        &mut output,
        &json!({
            "seconds":start.elapsed().as_secs_f64(),"options":options,"summary":summary,
            "identical_examples":examples,"subset_examples":report.page(DedupSection::Subsets,0,3).subsets
        }),
    )?;
    writeln!(output)?;
    Ok(())
}
