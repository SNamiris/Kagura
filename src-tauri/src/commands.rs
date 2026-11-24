use std::{path::Path, sync::Mutex};

use kagura_core::{
    check::{check_library, CheckReport},
    index::{AssetDetail, AssetSummary, IndexChanges, SearchPage},
    txn::{EditEntry, EditPatch, EditResult},
};
use serde::{Deserialize, Serialize};
use tauri::{
    ipc::{CommandArg, CommandItem, InvokeError},
    AppHandle, Manager, Runtime,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

use crate::desktop::{display, ApiError, Desktop, LibraryStats, Reply, Settings, SortSpec, UiFilters};

/// Keep IPC deserialization failures in the same error format as command failures.
pub(crate) struct Input<T>(T);

impl<'de, R: Runtime, T: Deserialize<'de>> CommandArg<'de, R> for Input<T> {
    fn from_command(command: CommandItem<'de, R>) -> Result<Self, InvokeError> {
        let key = command.key;
        T::deserialize(command)
            .map(Self)
            .map_err(|error| ApiError::new("invalid_argument", format!("参数 {key} 无效：{error}")).into())
    }
}

#[derive(Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct SongChildren(Vec<AssetSummary>);

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct FolderOpened(());

#[derive(Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct RecentEdits(Vec<EditEntry>);

#[derive(Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct TagSuggestions(Vec<String>);

async fn background<T: Send + 'static>(f: impl FnOnce() -> Reply<T> + Send + 'static) -> Reply<T> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| ApiError::new("worker_error", e))?
}

async fn with_desktop<R: Runtime, T: Send + 'static>(
    app: AppHandle<R>,
    job: impl FnOnce(&mut Desktop, &AppHandle<R>) -> Reply<T> + Send + 'static,
) -> Reply<T> {
    background(move || {
        let state = app.state::<Mutex<Desktop>>();
        let mut desktop = state
            .lock()
            .map_err(|_| ApiError::new("worker_error", "库状态不可用"))?;
        job(&mut desktop, &app)
    })
    .await
}

#[tauri::command]
pub(crate) async fn get_settings<R: Runtime>(app: AppHandle<R>) -> Reply<Settings> {
    with_desktop(app, |d, _| d.settings()).await
}

#[tauri::command]
pub(crate) async fn choose_library<R: Runtime>(window: tauri::Window<R>) -> Reply<Settings> {
    background(move || {
        let picker = window.dialog().file().set_title("选择包含 Library 的库根目录");
        #[cfg(any(windows, target_os = "macos"))]
        let picker = picker.set_parent(&window);
        let selected = picker.set_can_create_directories(false).blocking_pick_folder();
        let path = selected
            .map(|entry| entry.into_path())
            .transpose()
            .map_err(|e| ApiError::new("dialog_error", e))?;
        Ok(Settings {
            root: path.as_deref().map(display),
        })
    })
    .await
}

#[tauri::command]
pub(crate) async fn open_library<R: Runtime>(app: AppHandle<R>, root: Input<String>) -> Reply<LibraryStats> {
    with_desktop(app, move |desktop, app| {
        let (stats, restart) = desktop.open(Path::new(&root.0), &app.asset_protocol_scope())?;
        if restart {
            app.request_restart();
        } else {
            desktop.parse_background(app)?;
        }
        Ok(stats)
    })
    .await
}

#[tauri::command]
pub(crate) async fn rescan<R: Runtime>(app: AppHandle<R>) -> Reply<IndexChanges> {
    with_desktop(app, |desktop, app| {
        let library = desktop.current()?;
        let changes = library.index.update(&library.root)?;
        desktop.parse_background(app)?;
        Ok(changes)
    })
    .await
}

#[tauri::command]
pub(crate) async fn parse_status<R: Runtime>(app: AppHandle<R>) -> Reply<crate::parse_jobs::ParseStatus> {
    with_desktop(app, |desktop, _| desktop.parse_jobs.status()).await
}

#[tauri::command]
pub(crate) async fn search<R: Runtime>(
    app: AppHandle<R>,
    query: Input<String>,
    filters: Input<UiFilters>,
    sort: Input<SortSpec>,
    limit: Input<usize>,
    offset: Input<usize>,
) -> Reply<SearchPage> {
    with_desktop(app, move |desktop, app| {
        let library = desktop.current()?;
        let mut page = library
            .index
            .search(&query.0, &filters.0.into_core(sort.0), limit.0, offset.0)?;
        library.authorize_previews(&app.asset_protocol_scope(), &mut page.items)?;
        Ok(page)
    })
    .await
}

#[tauri::command]
pub(crate) async fn get_asset<R: Runtime>(app: AppHandle<R>, id: Input<String>) -> Reply<AssetDetail> {
    with_desktop(app, move |desktop, app| {
        let lib = desktop.current()?;
        let detail = lib
            .index
            .asset_detail(&lib.root, &id.0)?
            .ok_or_else(ApiError::missing)?;
        lib.grant_preview(&app.asset_protocol_scope(), &detail.summary)?;
        Ok(detail)
    })
    .await
}

#[tauri::command]
pub(crate) async fn song_children<R: Runtime>(
    app: AppHandle<R>,
    song_id: Input<String>,
) -> Reply<SongChildren> {
    with_desktop(app, move |desktop, app| {
        let library = desktop.current()?;
        let mut children = library.index.children(&song_id.0)?;
        library.authorize_previews(&app.asset_protocol_scope(), &mut children)?;
        Ok(SongChildren(children))
    })
    .await
}

#[tauri::command]
pub(crate) async fn stats<R: Runtime>(app: AppHandle<R>) -> Reply<LibraryStats> {
    with_desktop(app, |desktop, _| desktop.current()?.stats()).await
}

#[tauri::command]
pub(crate) async fn open_folder<R: Runtime>(app: AppHandle<R>, id: Input<String>) -> Reply<FolderOpened> {
    with_desktop(app, move |desktop, app| {
        let library = desktop.current()?;
        let dir = library
            .index
            .asset_directory(&library.root, &id.0)?
            .ok_or_else(ApiError::missing)?;
        app.opener()
            .open_path(display(&dir), None::<&str>)
            .map_err(|e| ApiError::new("open_folder", e))?;
        Ok(FolderOpened(()))
    })
    .await
}

#[tauri::command]
pub(crate) async fn run_check<R: Runtime>(app: AppHandle<R>) -> Reply<CheckReport> {
    let root = with_desktop(app, |desktop, _| Ok(desktop.current()?.root.clone())).await?;
    background(move || Ok(check_library(&root))).await
}

#[tauri::command]
pub(crate) async fn edit_asset<R: Runtime>(
    app: AppHandle<R>,
    id: Input<String>,
    patch: Input<EditPatch>,
    base_hash: Input<String>,
) -> Reply<EditResult> {
    with_desktop(app, move |desktop, _| {
        let lib = desktop.current()?;
        Ok(lib.index.edit_manifest(&lib.root, &id.0, patch.0, &base_hash.0)?)
    })
    .await
}

#[tauri::command]
pub(crate) async fn undo_edit<R: Runtime>(app: AppHandle<R>, entry_id: Input<String>) -> Reply<EditResult> {
    with_desktop(app, move |desktop, _| {
        let lib = desktop.current()?;
        Ok(lib.index.undo_edit(&lib.root, &entry_id.0)?)
    })
    .await
}

#[tauri::command]
pub(crate) async fn list_edits<R: Runtime>(app: AppHandle<R>, limit: Input<usize>) -> Reply<RecentEdits> {
    with_desktop(app, move |desktop, _| {
        let lib = desktop.current()?;
        Ok(RecentEdits(lib.index.list_edits(&lib.root, limit.0)?))
    })
    .await
}

#[tauri::command]
pub(crate) async fn suggest_tags<R: Runtime>(
    app: AppHandle<R>,
    query: Input<String>,
    limit: Input<usize>,
) -> Reply<TagSuggestions> {
    with_desktop(app, move |desktop, _| {
        Ok(TagSuggestions(
            desktop.current()?.index.suggest_tags(&query.0, limit.0)?,
        ))
    })
    .await
}
