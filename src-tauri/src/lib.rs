mod commands;
mod desktop;
mod duplicates;
mod hash_jobs;
mod parse_jobs;

use std::sync::Mutex;

use desktop::Desktop;
use kagura_core::index::Index;
use tauri::Manager;

fn handlers<R: tauri::Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        commands::get_settings,
        commands::choose_library,
        commands::open_library,
        commands::rescan,
        commands::parse_status,
        duplicates::hash_status,
        duplicates::start_hash,
        duplicates::pause_hash,
        duplicates::cancel_hash,
        duplicates::dedup_page,
        duplicates::export_duplicates,
        commands::search,
        commands::get_asset,
        commands::song_children,
        commands::stats,
        commands::open_folder,
        commands::open_copy_folder,
        commands::run_check,
        commands::edit_asset,
        commands::undo_edit,
        commands::list_edits,
        commands::suggest_tags
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(Desktop::new(Index::default_path()?)));
            Ok(())
        })
        .invoke_handler(handlers())
        .run(tauri::generate_context!())
        .expect("error while running Kagura");
}

#[cfg(test)]
mod tests;
