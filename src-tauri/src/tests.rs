use std::{fs, path::Path, sync::Mutex};

use kagura_core::{
    check::CheckReport,
    index::{AssetDetail, AssetSummary, IndexChanges, SearchPage},
};
use serde_json::{json, Value};
use tauri::{
    test::{mock_builder, mock_context, noop_assets},
    Manager,
};
use ts_rs::{Config, TS};

use crate::{
    commands::*,
    desktop::{ApiError, Desktop, LibraryStats, Settings, SortSpec, UiFilters},
};

fn sync_types(actual: &Path, checked_in: &Path, write: bool) {
    if write {
        fs::create_dir_all(checked_in).unwrap();
    }
    for file in fs::read_dir(actual).unwrap() {
        let entry = file.unwrap();
        let target = checked_in.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            sync_types(&entry.path(), &target, write);
        } else {
            // ts-rs can leave a space before a field's multiline documentation.
            let raw = fs::read_to_string(entry.path()).unwrap();
            let mut expected = String::with_capacity(raw.len());
            for line in raw.lines() {
                expected.push_str(line.trim_end());
                expected.push('\n');
            }
            if write {
                fs::write(&target, &expected).unwrap();
            }
            assert_eq!(
                expected,
                fs::read_to_string(&target).unwrap_or_else(|error| panic!(
                    "{}: {error}; run npm run types:generate",
                    target.display()
                )),
                "{} is stale; run npm run types:generate",
                target.display()
            );
        }
    }
}

#[test]
fn command_typescript() {
    fn export(dir: &Path) {
        let config = Config::new().with_out_dir(dir);
        macro_rules! export {
            ($($ty:ty),+) => { $(<$ty>::export_all(&config).unwrap();)+ };
        }
        export!(
            Settings,
            ApiError,
            LibraryStats,
            IndexChanges,
            UiFilters,
            SortSpec,
            SearchPage,
            AssetDetail,
            AssetSummary,
            CheckReport,
            SongChildren,
            FolderOpened
        );
    }
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../ui/src/generated");
    let write = std::env::var("KAGURA_EXPORT_TYPES").as_deref() == Ok("1");
    let temp = tempfile::tempdir().unwrap();
    export(temp.path());
    sync_types(temp.path(), &target, write);
}

#[test]
fn library_commands_use_the_mini_library_and_reject_external_paths() {
    let temp = tempfile::tempdir().unwrap();
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../crates/kagura-core/tests/fixtures/mini_library");
    let database = temp.path().join("appdata/index.sqlite");
    let app = mock_builder()
        .manage(Mutex::new(Desktop::new(database.clone())))
        .invoke_handler(crate::handlers())
        .build(mock_context(noop_assets()))
        .unwrap();
    let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();
    let invoke = |cmd: &str, body: Value| {
        let request = tauri::webview::InvokeRequest {
            cmd: cmd.into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: if cfg!(windows) {
                "http://tauri.localhost"
            } else {
                "tauri://localhost"
            }
            .parse()
            .unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_owned(),
        };
        tauri::test::get_ipc_response(&window, request).map(|r| r.deserialize::<Value>().unwrap())
    };
    assert_eq!(invoke("get_settings", json!({})).unwrap(), json!({"root": null}));
    assert_eq!(invoke("stats", json!({})).unwrap_err()["code"], "library_closed");
    let library = invoke("open_library", json!({"root": root})).unwrap();
    assert_eq!(library["total"], 5);
    assert!(database.is_file());
    assert_eq!(
        invoke("get_settings", json!({})).unwrap()["root"],
        library["root"]
    );
    let page = invoke(
        "search",
        json!({
            "query":"Box", "filters":{}, "sort":{"field":"name","descending":false}, "limit":200, "offset":0
        }),
    )
    .unwrap();
    assert_eq!(page["total"], 1);
    let asset = invoke("get_asset", json!({"id":page["items"][0]["id"]})).unwrap();
    assert_eq!(asset["manifest"]["key"]["name"], "Box");
    assert!(asset["file_count"].as_u64().unwrap() > 0);
    assert!(Path::new(asset["absolute_path"].as_str().unwrap())
        .join("_asset.manifest.json")
        .is_file());
    for bad in ["../escape", "C:\\Windows", "00000000000000000000000999"] {
        assert_eq!(
            invoke("open_folder", json!({"id":bad})).unwrap_err()["code"],
            "asset_not_found"
        );
    }
    for malformed in [json!({"path":"C:\\Windows"}), json!({"id":42})] {
        let error = invoke("open_folder", malformed).unwrap_err();
        assert_eq!(error["code"], "invalid_argument");
        assert!(error["message"].as_str().unwrap().contains("id"));
    }
    assert_eq!(
        invoke("rescan", json!({})).unwrap(),
        serde_json::to_value(IndexChanges::default()).unwrap()
    );
    let empty = temp.path().join("not-a-library");
    fs::create_dir(&empty).unwrap();
    assert_eq!(
        invoke("open_library", json!({"root":empty})).unwrap_err()["code"],
        "invalid_root"
    );
    let bad_stage = empty.join("Library/Stages/Artist/Stage");
    fs::create_dir_all(&bad_stage).unwrap();
    fs::write(bad_stage.join("_asset.manifest.json"), "{").unwrap();
    assert!(invoke("open_library", json!({"root":empty})).is_err());
    assert_eq!(invoke("stats", json!({})).unwrap()["root"], library["root"]);
    assert_eq!(
        invoke("get_settings", json!({})).unwrap()["root"],
        library["root"]
    );
    let settings = database.with_file_name("settings.json");
    fs::write(&settings, b"broken settings\n").unwrap();
    assert_eq!(
        invoke("get_settings", json!({})).unwrap_err()["code"],
        "settings_invalid"
    );
    assert!(invoke("open_library", json!({"root":empty})).is_err());
    assert_eq!(fs::read(&settings).unwrap(), b"broken settings\n");
    invoke("open_library", json!({"root":root})).unwrap();
    assert_eq!(
        invoke("get_settings", json!({})).unwrap()["root"],
        library["root"]
    );
}

#[test]
fn preview_permissions_apply_to_only_the_active_library() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("first");
    let dir = root.join("Library/Models/Props/Builder/Box");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("_asset.manifest.json"),
        include_bytes!("../../crates/kagura-core/tests/fixtures/mini_library/Library/Models/Props/Builder/Box/_asset.manifest.json")).unwrap();
    fs::write(dir.join("preview.png"), b"preview bytes").unwrap();
    fs::write(dir.join("private.txt"), b"private").unwrap();
    let other = temp.path().join("other");
    fs::create_dir_all(other.join("Library")).unwrap();
    let app = mock_builder().build(mock_context(noop_assets())).unwrap();
    let scope = app.asset_protocol_scope();
    let mut desktop = Desktop::new(temp.path().join("cache/index.sqlite"));
    assert!(!desktop.open(&root, &scope).unwrap().1);
    assert!(!scope.is_allowed(dir.join("preview.png")));
    let library = desktop.current().unwrap();
    let mut page = library.index.search("", &Default::default(), 200, 0).unwrap();
    library.authorize_previews(&scope, &mut page.items).unwrap();
    assert!(scope.is_allowed(dir.join("preview.png")));
    assert!(!scope.is_allowed(dir.join("private.txt")));
    assert!(!desktop.open(&other, &scope).unwrap().1);
    assert!(!scope.is_allowed(dir.join("preview.png")));
    assert!(desktop.open(&root, &scope).unwrap().1);
}
