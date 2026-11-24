use std::{
    collections::BTreeMap,
    fmt::Display,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use kagura_core::index::{AssetSummary, Index, IndexError, Missing, SearchFilters, Sort};
use serde::{Deserialize, Serialize};
use tauri::scope::fs::Scope;

pub(crate) type Reply<T> = Result<T, ApiError>;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct ApiError {
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn new(code: &str, message: impl Display) -> Self {
        Self {
            code: code.to_owned(),
            message: message.to_string(),
        }
    }

    pub fn missing() -> Self {
        Self::new("asset_not_found", "当前库中没有此资产")
    }
}

impl From<IndexError> for ApiError {
    fn from(error: IndexError) -> Self {
        Self::new("index_error", error)
    }
}

impl From<kagura_core::txn::TxnError> for ApiError {
    fn from(error: kagura_core::txn::TxnError) -> Self {
        Self::new(error.code(), error)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct Settings {
    pub root: Option<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct LibraryStats {
    pub root: String,
    pub total: usize,
    #[cfg_attr(test, ts(type = "number"))]
    pub last_scan: i64,
    pub counts: BTreeMap<String, usize>,
    pub unknown_author: BTreeMap<String, usize>,
    pub unknown_composer: usize,
    pub explicit_license: BTreeMap<String, usize>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(default)]
pub(crate) struct UiFilters {
    pub types: Vec<String>,
    pub kinds: Vec<String>,
    pub author: Option<String>,
    pub tag: Option<String>,
    pub missing: Vec<Missing>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct SortSpec {
    pub field: Sort,
    pub descending: bool,
}

impl UiFilters {
    pub fn into_core(self, sort: SortSpec) -> SearchFilters {
        SearchFilters {
            types: self.types,
            kinds: self.kinds,
            author_contains: self.author.filter(|s| !s.is_empty()),
            tag: self.tag.filter(|s| !s.is_empty()),
            missing_any: self.missing,
            sort: sort.field,
            descending: Some(sort.descending),
            ..Default::default()
        }
    }
}

pub(crate) fn display(path: &Path) -> String {
    dunce::simplified(path).to_string_lossy().into_owned()
}

pub(crate) struct Library {
    pub root: PathBuf,
    pub index: Index,
    granted_previews: bool,
}

impl Library {
    pub fn stats(&self) -> Reply<LibraryStats> {
        let stats = self.index.stats()?;
        let meta = self
            .index
            .library_meta()?
            .ok_or_else(|| ApiError::new("library_closed", "请先打开资产库"))?;
        Ok(LibraryStats {
            root: display(&self.root),
            total: stats.counts.values().sum(),
            last_scan: meta.last_scan,
            counts: stats.counts,
            unknown_author: stats.unknown_author,
            unknown_composer: stats.unknown_composer,
            explicit_license: stats.explicit_license,
        })
    }

    pub fn authorize_previews(&mut self, scope: &Scope, items: &mut [AssetSummary]) -> Reply<()> {
        self.index.load_previews(&self.root, items)?;
        for item in items {
            self.grant_preview(scope, item)?;
        }
        Ok(())
    }

    pub fn grant_preview(&mut self, scope: &Scope, item: &AssetSummary) -> Reply<()> {
        if let Some(path) = &item.preview_path {
            if !scope.is_allowed(path) {
                scope
                    .allow_file(path)
                    .map_err(|e| ApiError::new("preview_scope", e))?;
            }
            self.granted_previews = true;
        }
        Ok(())
    }
}

pub(crate) struct Desktop {
    database: PathBuf,
    settings: PathBuf,
    active: Option<Library>,
}

impl Desktop {
    pub fn new(database: PathBuf) -> Self {
        Self {
            settings: database.with_file_name("settings.json"),
            database,
            active: None,
        }
    }

    fn settings_bytes(&self) -> Reply<Option<Vec<u8>>> {
        match fs::read(&self.settings) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(ApiError::new("settings_read", e)),
        }
    }

    pub fn settings(&self) -> Reply<Settings> {
        self.settings_bytes()?
            .map(|bytes| serde_json::from_slice(&bytes))
            .transpose()
            .map(Option::unwrap_or_default)
            .map_err(|e| ApiError::new("settings_invalid", e))
    }

    fn save_settings(&self, bytes: &[u8]) -> Reply<()> {
        let save = || -> io::Result<()> {
            let dir = self.settings.parent().expect("settings has a parent");
            fs::create_dir_all(dir)?;
            let mut temp = tempfile::NamedTempFile::new_in(dir)?;
            temp.write_all(bytes)?;
            temp.as_file().sync_all()?;
            temp.persist(&self.settings)?;
            Ok(())
        };
        save().map_err(|e| ApiError::new("settings_write", e))
    }

    pub fn current(&mut self) -> Reply<&mut Library> {
        self.active
            .as_mut()
            .ok_or_else(|| ApiError::new("library_closed", "请先打开资产库"))
    }

    /// Returns true when reopening a library whose previous preview permissions were revoked.
    pub fn open(&mut self, root: &Path, scope: &Scope) -> Reply<(LibraryStats, bool)> {
        let root = root
            .canonicalize()
            .map_err(|e| ApiError::new("invalid_root", e))?;
        if !root.join("Library").is_dir() {
            return Err(ApiError::new(
                "invalid_root",
                "请选择包含 Library 子目录的库根目录",
            ));
        }
        let mut index = Index::open_for_library(&self.database, &root)?;
        index.set_edits_dir(self.settings.with_file_name("edits"));
        let previous = self.settings_bytes()?;
        let selection = Settings {
            root: Some(display(&root)),
        };
        // Persist the choice before committing the new cache. A settings error leaves the
        // active connection unchanged; a failed scan rolls back its SQLite transaction.
        let changed = previous
            .as_deref()
            .and_then(|bytes| serde_json::from_slice(bytes).ok())
            != Some(selection.clone());
        if changed {
            let mut encoded =
                serde_json::to_vec_pretty(&selection).map_err(|e| ApiError::new("settings_write", e))?;
            encoded.push(b'\n');
            self.save_settings(&encoded)?;
        }
        let ready = index.update(&root).map_err(ApiError::from).and_then(|_| {
            if let Err(error) = index.recover_edits(&root) {
                // A failed switch must keep the UI's previous root paired with its own cache.
                if let Some(old) = self.active.as_ref().filter(|old| old.root != root) {
                    index.update(&old.root)?;
                }
                return Err(error.into());
            }
            Ok(())
        });
        if let Err(error) = ready {
            if changed {
                if let Some(previous) = previous {
                    self.save_settings(&previous)?;
                } else {
                    fs::remove_file(&self.settings).map_err(|e| ApiError::new("settings_write", e))?;
                }
            }
            return Err(error);
        }
        let mut library = Library {
            root,
            index,
            granted_previews: false,
        };
        if let Some(old) = &self.active {
            if old.root == library.root {
                library.granted_previews = old.granted_previews;
            } else if old.granted_previews {
                // Tauri gives forbidden patterns precedence for the lifetime of this process.
                scope
                    .forbid_directory(old.root.join("Library"), true)
                    .map_err(|error| ApiError::new("preview_scope", error))?;
            }
        }
        let restart = scope.is_forbidden(library.root.join("Library"));
        let result = library.stats()?;
        self.active = Some(library);
        Ok((result, restart))
    }
}
