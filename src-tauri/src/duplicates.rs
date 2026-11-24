use kagura_core::index::dedup::{DedupOptions, DedupPage, DedupSection, DuplicateReport};
use serde::Serialize;
use std::{
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_dialog::DialogExt;

use crate::{
    commands::{background, with_desktop, Input},
    desktop::{display, ApiError, Reply},
    hash_jobs::HashStatus,
};

pub(crate) struct SavedReport {
    rev: (i64, u64),
    options: DedupOptions,
    report: Arc<DuplicateReport>,
}

enum Snapshot {
    Ready(Arc<DuplicateReport>),
    Build {
        root: PathBuf,
        database: PathBuf,
        revision: (i64, u64),
    },
}

async fn snapshot<R: Runtime>(app: AppHandle<R>, options: DedupOptions) -> Reply<Arc<DuplicateReport>> {
    let request = with_desktop(app.clone(), move |d, _| {
        let database = d.database().to_owned();
        let lib = d.current()?;
        let revision = lib.index.dedup_revision()?;
        if let Some(saved) = &lib.report {
            if saved.rev == revision && saved.options == options {
                return Ok(Snapshot::Ready(saved.report.clone()));
            }
        }
        Ok(Snapshot::Build {
            root: lib.root.clone(),
            database,
            revision,
        })
    })
    .await?;
    let (root, database, revision) = match request {
        Snapshot::Ready(report) => return Ok(report),
        Snapshot::Build {
            root,
            database,
            revision,
        } => (root, database, revision),
    };
    let report = background(move || {
        let report = DuplicateReport::read_cache(&database, options)?;
        if report.root() != root {
            return Err(ApiError::new("library_changed", "资产库已切换，请重新打开报告"));
        }
        Ok(Arc::new(report))
    })
    .await?;
    with_desktop(app, move |d, _| {
        let lib = d.current()?;
        if lib.root != report.root() {
            return Err(ApiError::new("library_changed", "资产库已切换，请重新打开报告"));
        }
        if lib.index.dedup_revision()? == revision {
            lib.report = Some(SavedReport {
                rev: revision,
                options,
                report: report.clone(),
            });
        }
        Ok(report)
    })
    .await
}

#[tauri::command]
pub(crate) async fn dedup_page<R: Runtime>(
    app: AppHandle<R>,
    options: Input<DedupOptions>,
    section: Input<DedupSection>,
    offset: Input<usize>,
    limit: Input<usize>,
) -> Reply<DedupPage> {
    let report = snapshot(app, options.0).await?;
    background(move || Ok(report.page(section.0, offset.0, limit.0.min(200)))).await
}

#[tauri::command]
pub(crate) async fn hash_status<R: Runtime>(app: AppHandle<R>) -> Reply<HashStatus> {
    with_desktop(app, |d, _| d.hash_jobs.status()).await
}

#[tauri::command]
pub(crate) async fn start_hash<R: Runtime>(app: AppHandle<R>, threads: Input<usize>) -> Reply<HashStatus> {
    with_desktop(app, move |d, app| {
        let root = d.current()?.root.clone();
        d.hash_jobs
            .start(app.clone(), root, d.database().to_owned(), threads.0)
    })
    .await
}

#[tauri::command]
pub(crate) async fn pause_hash<R: Runtime>(app: AppHandle<R>) -> Reply<HashStatus> {
    with_desktop(app, |d, app| d.hash_jobs.stop(app, false)).await
}

#[tauri::command]
pub(crate) async fn cancel_hash<R: Runtime>(app: AppHandle<R>) -> Reply<HashStatus> {
    with_desktop(app, |d, app| d.hash_jobs.stop(app, true)).await
}

#[derive(Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct ExportedReport {
    pub path: Option<String>,
}

fn output_path(root: &Path, path: &Path) -> Reply<PathBuf> {
    let denied = || ApiError::new("export_location", "请选择资产库之外的 JSON 文件");
    if !path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("json"))
    {
        return Err(denied());
    }
    let parent = path
        .parent()
        .ok_or_else(denied)?
        .canonicalize()
        .map_err(|e| ApiError::new("export_location", e))?;
    let library_root = root
        .canonicalize()
        .map_err(|e| ApiError::new("export_location", e))?;
    // Check the resolved directory, including junctions selected through the native dialog.
    if parent.starts_with(library_root) || parent.ancestors().any(|p| p.join("Library").is_dir()) {
        return Err(denied());
    }
    let name = path.file_name().and_then(|n| n.to_str()).ok_or_else(denied)?;
    kagura_core::naming::check_rel(name).map_err(|_| denied())?;
    let out = parent.join(name);
    if fs::symlink_metadata(&out).is_ok_and(|m| !m.is_file() || kagura_core::scan::is_reparse(&m)) {
        return Err(denied());
    }
    Ok(out)
}

pub(crate) fn write_export(report: &DuplicateReport, path: &Path) -> Reply<PathBuf> {
    let out = output_path(report.root(), path)?;
    let io_error = |e| ApiError::new("export_io", e);
    let mut temp =
        tempfile::NamedTempFile::new_in(out.parent().expect("absolute output path")).map_err(io_error)?;
    {
        let mut writer = BufWriter::new(&mut temp);
        report.write_json(&mut writer)?;
        writer.write_all(b"\n").map_err(io_error)?;
        writer.flush().map_err(io_error)?;
    }
    temp.as_file().sync_all().map_err(io_error)?;
    temp.persist(&out).map_err(|e| io_error(e.error))?;
    Ok(out)
}

#[tauri::command]
pub(crate) async fn export_duplicates<R: Runtime>(
    app: AppHandle<R>,
    window: tauri::Window<R>,
    options: Input<DedupOptions>,
) -> Reply<ExportedReport> {
    let report = snapshot(app, options.0).await?;
    background(move || {
        let picker = window
            .dialog()
            .file()
            .set_title("导出重复报告")
            .add_filter("JSON", &["json"])
            .set_file_name("kagura-duplicates.json")
            .set_can_create_directories(false);
        let folder = window
            .path()
            .document_dir()
            .ok()
            .into_iter()
            .chain([std::env::temp_dir()])
            .find(|dir| output_path(report.root(), &dir.join("kagura-duplicates.json")).is_ok());
        let picker = match folder {
            Some(dir) => picker.set_directory(dir),
            None => picker,
        };
        #[cfg(any(windows, target_os = "macos"))]
        let picker = picker.set_parent(&window);
        let Some(file) = picker.blocking_save_file() else {
            return Ok(ExportedReport { path: None });
        };
        let path = file
            .into_path()
            .map_err(|e| ApiError::new("export_location", e))?;
        Ok(ExportedReport {
            path: Some(display(&write_export(&report, &path)?)),
        })
    })
    .await
}
