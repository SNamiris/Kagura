use kagura_core::schema::Schemas;

/// 占位命令：确认内嵌 schema 能编译。M3 第 7 步换成扫描、列表等命令。
#[tauri::command]
fn core_status() -> Result<String, String> {
    Schemas::load()
        .map(|_| format!("kagura-core ok, format = {}", kagura_core::consts::FORMAT))
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![core_status])
        .run(tauri::generate_context!())
        .expect("error while running Kagura");
}
