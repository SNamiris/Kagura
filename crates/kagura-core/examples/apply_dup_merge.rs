//! Apply or undo the automatic candidates from a Kagura duplicate review.
#[path = "dup_merge/journal.rs"]
mod journal;
#[path = "dup_merge/plan.rs"]
mod plan;
#[cfg(test)]
#[path = "dup_merge/tests.rs"]
mod tests;

use kagura_core::index::Index;
use rusqlite::{Connection, OpenFlags, OptionalExtension};
use std::{collections::BTreeSet, env, fs, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const DATE: &str = "2026-11-26";
const ARCHIVE: &str = "_Duplicates/dup-assets-20261126";
const RUN: &str = "_design/migration_runs/dup-assets-20261126";

fn ensure(condition: bool, message: impl FnOnce() -> String) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message().into())
    }
}

fn desktop_index_guard(root: &std::path::Path) -> Result<Option<Connection>> {
    let Ok(path) = Index::default_path() else {
        return Ok(None);
    };
    if !path.is_file() {
        return Ok(None);
    }
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    conn.busy_timeout(std::time::Duration::ZERO)?;
    let library: Option<String> = conn
        .query_row("SELECT root FROM meta WHERE id=1", [], |r| r.get(0))
        .optional()?;
    if library
        .as_deref()
        .and_then(|p| dunce::canonicalize(p).ok())
        .as_deref()
        != Some(root)
    {
        return Ok(None);
    }
    #[cfg(windows)]
    {
        let out = std::process::Command::new("tasklist.exe")
            .args(["/FI", "IMAGENAME eq kagura.exe", "/FO", "CSV", "/NH"])
            .output()?;
        ensure(out.status.success(), || "无法确认 Kagura 桌面进程状态".into())?;
        ensure(
            !String::from_utf8_lossy(&out.stdout)
                .to_ascii_lowercase()
                .contains("\"kagura.exe\""),
            || "请先关闭 Kagura 桌面应用".into(),
        )?;
    }
    conn.execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| format!("无法锁定桌面索引的写入事务：{e}"))?;
    Ok(Some(conn))
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let mut root = PathBuf::from(r"G:\MMD");
    let mut report = None;
    let mut only = BTreeSet::new();
    let mut dry_run = false;
    let mut undo = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().ok_or("缺少库根路径")?.into(),
            "--dry-run" => dry_run = true,
            "--undo" => undo = true,
            "--only" => {
                for group in args.next().ok_or("缺少组号")?.split(',') {
                    only.insert(group.parse::<u32>()?);
                }
            }
            _ if arg.starts_with('-') || report.is_some() => return Err(format!("未知参数：{arg}").into()),
            _ => report = Some(PathBuf::from(arg)),
        }
    }
    let usage = "用法：apply_dup_merge <报告 JSON> [--root <库根>] [--dry-run] [--only <组号,...>]；撤回使用 --undo [--dry-run]";
    let report = if undo {
        ensure(only.is_empty(), || {
            "--undo 撤回整次迁移，不能与 --only 一起使用".into()
        })?;
        None
    } else {
        Some(serde_json::from_slice::<plan::Report>(&fs::read(
            report.ok_or(usage)?,
        )?)?)
    };
    let eligible = report.as_ref().map(|report| report.eligible(&only)).transpose()?;
    let root = dunce::canonicalize(root)?;
    let _desk = desktop_index_guard(&root)?;
    journal::check_run_paths(&root)?;
    let mut journal = if dry_run {
        journal::Journal::read(&root)?
    } else {
        journal::Journal::open(&root)?
    };
    let Some(eligible) = eligible else {
        return journal.undo(&root, dry_run);
    };
    let applied = journal.completed()?;
    let prepared = plan::prepare(&root, eligible, &applied)?;
    if !dry_run {
        for group in prepared {
            journal.apply(&root, group)?;
        }
    }
    Ok(())
}
