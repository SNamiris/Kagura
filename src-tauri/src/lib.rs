use kagura_core::schema::Schemas;

#[derive(serde::Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
struct CoreStatus(String);

/// 占位命令：确认内嵌 schema 能编译。M3 第 7 步换成扫描、列表等命令。
#[tauri::command]
fn core_status() -> Result<CoreStatus, String> {
    Schemas::load()
        .map(|_| {
            CoreStatus(format!(
                "kagura-core ok, format = {}",
                kagura_core::consts::FORMAT
            ))
        })
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![core_status])
        .run(tauri::generate_context!())
        .expect("error while running Kagura");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use ts_rs::{Config, TS};

    #[test]
    fn core_status_typescript() {
        let config =
            Config::new().with_out_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../ui/src/generated"));
        if std::env::var("KAGURA_EXPORT_TYPES").as_deref() == Ok("1") {
            CoreStatus::export(&config).unwrap();
        }
        let path = config.out_dir().join("CoreStatus.ts");
        let checked_in = std::fs::read_to_string(path).expect("在 ui 目录运行 npm run types:generate");
        assert_eq!(
            checked_in,
            CoreStatus::export_to_string(&config).unwrap(),
            "请在 ui 目录运行 npm run types:generate 更新绑定"
        );
        let status = core_status().unwrap();
        assert_eq!(
            serde_json::to_value(&status).unwrap(),
            serde_json::Value::String(status.0)
        );
    }
}
