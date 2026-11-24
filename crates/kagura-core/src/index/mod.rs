//! Rebuildable SQLite search cache. Source library files are only ever opened for reading.

mod detail;
mod hash;
mod paths;
mod query;
mod store;

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use rusqlite::{Connection, TransactionBehavior};
use thiserror::Error;

use crate::scan::ScanReport;
pub use detail::{AssetDetail, AssetRef, LibraryMeta};
pub use hash::{HashProgress, HashReport};
pub use query::{AssetSummary, IndexStats, Missing, SearchFilters, SearchPage, Sort};

const SCHEMA_VERSION: i32 = 1;
const APPLICATION_ID: i32 = 0x4b475241; // KGRA

#[derive(Debug, Error)]
pub enum IndexError {
    #[error(transparent)]
    Sql(#[from] rusqlite::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Invalid(String),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct IndexChanges {
    pub inserted: usize,
    pub updated: usize,
    pub deleted: usize,
    pub files_changed: usize,
    pub files_deleted: usize,
}

pub struct Index {
    conn: Connection,
    path: PathBuf,
}

impl Index {
    pub fn default_path() -> Result<PathBuf, IndexError> {
        let appdata = env::var_os("APPDATA")
            .filter(|path| !path.is_empty())
            .ok_or_else(|| IndexError::Invalid("APPDATA is unset; specify an index path".into()))?;
        Ok(PathBuf::from(appdata).join("Kagura").join("index.sqlite"))
    }

    /// Resolve existing junctions before creating any directories, database or WAL files.
    pub fn open_for_library(path: impl AsRef<Path>, root: &Path) -> Result<Self, IndexError> {
        let target = resolve_output(path.as_ref())?;
        outside_library(&target, &root.canonicalize()?)?;
        Self::open(target)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, IndexError> {
        let path = resolve_output(path.as_ref())?;
        if let Some(root) = env::var_os("KAGURA_LIBRARY").and_then(|dir| Path::new(&dir).canonicalize().ok())
        {
            outside_library(&path, &root)?;
        }
        if let Some(root) = path.ancestors().skip(1).find(|dir| {
            let library = dir.join("Library");
            ["Models", "Stages", "Motions"]
                .iter()
                .any(|category| library.join(category).is_dir())
        }) {
            outside_library(&path, root)?;
        }
        for extension in ["-wal", "-shm", "-journal"] {
            let mut sidecar = path.as_os_str().to_owned();
            sidecar.push(extension);
            if fs::symlink_metadata(&sidecar).is_ok_and(|meta| crate::scan::is_reparse(&meta)) {
                return Err(IndexError::Invalid(format!(
                    "SQLite sidecar is a reparse point: {sidecar:?}"
                )));
            }
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut conn = Connection::open(&path)?;
        verify_cache(&conn, &path)?;
        let mode: String = conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))?;
        if !mode.eq_ignore_ascii_case("wal") {
            return Err(IndexError::Invalid(format!(
                "SQLite returned journal mode {mode}, expected WAL"
            )));
        }
        // Old schemas may contain foreign keys in any order, including cycles.
        conn.execute_batch("PRAGMA foreign_keys=OFF")?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let app_id = verify_cache(&tx, &path)?;
        let version: i32 = tx.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if version != SCHEMA_VERSION || app_id != APPLICATION_ID {
            drop_cache_schema(&tx)?;
            tx.execute_batch(include_str!("schema.sql"))?;
            tx.pragma_update(None, "application_id", APPLICATION_ID)?;
            tx.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        }
        tx.commit()?;
        conn.execute_batch("PRAGMA foreign_keys=ON")?;
        Ok(Self { conn, path })
    }

    pub fn rebuild(&mut self, report: &ScanReport) -> Result<IndexChanges, IndexError> {
        let root = report.root.canonicalize()?;
        outside_library(&self.path, &root)?;
        store::rebuild(&mut self.conn, &root, report)
    }

    pub fn update(&mut self, root: &Path) -> Result<IndexChanges, IndexError> {
        let root = root.canonicalize()?;
        outside_library(&self.path, &root)?;
        store::update(&mut self.conn, &root)
    }
}

fn drop_cache_schema(tx: &rusqlite::Transaction<'_>) -> Result<(), IndexError> {
    // Dropping the virtual table removes its own shadow tables. SQLite owns sqlite_* objects.
    let objects = tx
        .prepare(
            "SELECT name,type FROM pragma_table_list WHERE schema='main' AND name NOT GLOB 'sqlite_*'
         AND type IN ('view','virtual','table') ORDER BY type='table', type='virtual'",
        )?
        .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for (name, kind) in objects {
        let object = if kind == "view" { "VIEW" } else { "TABLE" };
        let escaped = name.replace('"', "\"\"");
        tx.execute_batch(&format!("DROP {object} \"{escaped}\""))?;
    }
    Ok(())
}

fn verify_cache(conn: &Connection, path: &Path) -> Result<i32, IndexError> {
    // Read the identifier and schema on a single SQLite snapshot, including on first open.
    let (app_id, has_tables): (i32, bool) = conn.query_row(
        "SELECT application_id, EXISTS(SELECT 1 FROM sqlite_schema WHERE name NOT GLOB 'sqlite_*')
         FROM pragma_application_id",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    if app_id != APPLICATION_ID && (app_id != 0 || has_tables) {
        return Err(IndexError::Invalid(format!(
            "existing database is not a Kagura cache: {}",
            path.display()
        )));
    }
    Ok(app_id)
}

fn resolve_output(path: &Path) -> io::Result<PathBuf> {
    let absolute = std::path::absolute(path)?;
    let mut missing = Vec::new();
    let mut ancestor = absolute.as_path();
    loop {
        match ancestor.canonicalize() {
            Ok(mut resolved) => {
                for name in missing.into_iter().rev() {
                    resolved.push(name);
                }
                return Ok(resolved);
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                // A dangling symlink is not an available output name.
                if fs::symlink_metadata(ancestor).is_ok() {
                    return Err(error);
                }
                missing.push(
                    ancestor
                        .file_name()
                        .ok_or_else(|| io::Error::other("invalid output path"))?,
                );
                ancestor = ancestor.parent().ok_or(error)?;
            }
            Err(error) => return Err(error),
        }
    }
}

fn outside_library(database: &Path, root: &Path) -> Result<(), IndexError> {
    if database.starts_with(root) {
        return Err(IndexError::Invalid(format!(
            "index path must be outside the library root {}",
            root.display()
        )));
    }
    Ok(())
}
