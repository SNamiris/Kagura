use std::env;
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;

use kagura_core::check::check_library;

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let Some(root) = args.next().filter(|_| args.next().is_none()) else {
        eprintln!("Usage: check_library <MMD root>");
        return ExitCode::FAILURE;
    };
    let start = Instant::now();
    let report = check_library(Path::new(&root));
    let records = report.counts.values().sum::<usize>();
    println!(
        "{records} records, {} findings, {:.3} s",
        report.findings.len(),
        start.elapsed().as_secs_f64()
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("serializable check report")
    );
    if report.findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
