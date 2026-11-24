use std::{fs, path::Path, sync::Mutex};

use kagura_core::{
    check::CheckReport,
    index::{AssetDetail, AssetSummary, IndexChanges, SearchPage},
    txn::{EditPatch, EditResult},
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
            crate::parse_jobs::ParseStatus,
            IndexChanges,
            UiFilters,
            SortSpec,
            SearchPage,
            AssetDetail,
            AssetSummary,
            CheckReport,
            SongChildren,
            FolderOpened,
            EditPatch,
            EditResult,
            RecentEdits,
            TagSuggestions
        );
    }
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../ui/src/generated");
    let write = std::env::var("KAGURA_EXPORT_TYPES").as_deref() == Ok("1");
    let temp = tempfile::tempdir().unwrap();
    export(temp.path());
    sync_types(temp.path(), &target, write);
}

fn invoke_on(
    window: &tauri::WebviewWindow<tauri::test::MockRuntime>,
    cmd: &str,
    body: Value,
) -> Result<Value, Value> {
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
    tauri::test::get_ipc_response(window, request).map(|r| r.deserialize::<Value>().unwrap())
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
    let invoke = |cmd, body| invoke_on(&window, cmd, body);
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
fn editing_commands_roundtrip_through_ipc() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("mmd");
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../crates/kagura-core/tests/fixtures/mini_library");
    for rel in kagura_core::scan::scan_library_for_check(&fixture).files {
        let target = root.join(&rel);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(fixture.join(rel), target).unwrap();
    }
    let app = mock_builder()
        .manage(Mutex::new(Desktop::new(temp.path().join("appdata/index.sqlite"))))
        .invoke_handler(crate::handlers())
        .build(mock_context(noop_assets()))
        .unwrap();
    let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();
    let invoke = |cmd, body| invoke_on(&window, cmd, body);
    invoke("open_library", json!({"root":root})).unwrap();
    let id = "00000000000000000000000001";
    let detail = invoke("get_asset", json!({"id":id})).unwrap();
    let base = &detail["base_hash"];
    let original_path = Path::new(detail["absolute_path"].as_str().unwrap()).join("_asset.manifest.json");
    let original = fs::read(&original_path).unwrap();
    assert_eq!(base.as_str().unwrap().len(), 64);
    assert!(invoke("edit_asset", json!({"id":id,"patch":{"key":{}},"baseHash":base})).is_err());
    let patch = json!({"tags":["ipc-test"],"rating":4,"note":"一行\n二行","source":{"url":"https://example.org/asset"}});
    let result = invoke("edit_asset", json!({"id":id,"patch":patch,"baseHash":base})).unwrap();
    let saved = invoke("get_asset", json!({"id":id})).unwrap();
    assert_eq!(saved["base_hash"], result["new_hash"]);
    assert_eq!(saved["manifest"]["source"]["url"], patch["source"]["url"]);
    assert_eq!(saved["summary"]["rating"], 4);
    assert_eq!(
        invoke("suggest_tags", json!({"query":"ipc","limit":10})).unwrap(),
        json!(["ipc-test"])
    );
    let conflict = invoke(
        "edit_asset",
        json!({"id":id,"patch":{"note":"stale"},"baseHash":base}),
    )
    .unwrap_err();
    assert_eq!(conflict["code"], "conflict");
    let history = invoke("list_edits", json!({"limit":5})).unwrap();
    assert_eq!(history[0]["entry_id"], result["entry_id"]);
    let undone = invoke("undo_edit", json!({"entryId":result["entry_id"]})).unwrap();
    assert_eq!(&undone["new_hash"], base);
    assert_eq!(fs::read(original_path).unwrap(), original);
    assert_eq!(
        invoke("get_asset", json!({"id":id})).unwrap()["manifest"],
        detail["manifest"]
    );
    assert_eq!(
        invoke("suggest_tags", json!({"query":"ipc","limit":10})).unwrap(),
        json!([])
    );
    assert_eq!(
        invoke("list_edits", json!({"limit":1})).unwrap()[0]["undone"],
        true
    );
}

#[test]
fn background_parse_reports_generations_and_file_errors() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("one");
    let model = root.join("Library/Models/Props/Builder/Box");
    fs::create_dir_all(&model).unwrap();
    fs::write(model.join("_asset.manifest.json"),
        include_bytes!("../../crates/kagura-core/tests/fixtures/mini_library/Library/Models/Props/Builder/Box/_asset.manifest.json")).unwrap();
    fs::write(model.join("damaged.pmx"), b"PMX ").unwrap();
    let mut motion = b"Vocaloid Motion Data 0002".to_vec();
    motion.resize(74, 0);
    fs::write(model.join("sample.vmd"), motion).unwrap();
    let app = mock_builder()
        .manage(Mutex::new(Desktop::new(temp.path().join("cache/index.sqlite"))))
        .invoke_handler(crate::handlers())
        .build(mock_context(noop_assets()))
        .unwrap();
    let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();
    let call = |cmd, body| invoke_on(&window, cmd, body).unwrap();
    let until_complete = |generation: u32| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
        loop {
            let state = call("parse_status", json!({}));
            assert_ne!(state["phase"], "failed", "{state}");
            if state["phase"] == "complete" && state["generation"] == generation {
                break state;
            }
            assert!(std::time::Instant::now() < deadline, "{state}");
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    };
    assert_eq!(call("parse_status", json!({}))["phase"], "idle");
    call("open_library", json!({"root":root}));
    let browse = call(
        "search",
        json!({"query":"Box","filters":{}, "sort":{"field":"name","descending":false},"limit":5,"offset":0}),
    );
    assert_eq!(browse["total"], 1);
    let state = until_complete(1);
    assert_eq!(
        (state["total"].as_u64(), state["failed"].as_u64()),
        (Some(2), Some(1))
    );
    let detail = call("get_asset", json!({"id":browse["items"][0]["id"]}));
    assert_eq!(detail["model_info"][0]["parse_error"]["section"], "header");
    assert_eq!(detail["motion_info"][0]["max_frame"], 0);
    call("rescan", json!({}));
    assert_eq!(until_complete(2)["cached"], 2);
    let other = temp.path().join("two");
    fs::create_dir_all(other.join("Library")).unwrap();
    let switched = call("open_library", json!({"root":other}));
    let empty = until_complete(3);
    assert_eq!(
        (empty["root"].as_str(), empty["total"].as_u64()),
        (switched["root"].as_str(), Some(0))
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
