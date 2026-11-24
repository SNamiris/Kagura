mod commands;
mod desktop;

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
        commands::search,
        commands::get_asset,
        commands::song_children,
        commands::stats,
        commands::open_folder,
        commands::run_check
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
