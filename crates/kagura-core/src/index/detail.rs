use std::{
    io::Read,
    path::{Path, PathBuf},
};

use rusqlite::{params, Connection, OptionalExtension, Statement, Transaction, TransactionBehavior};
use serde::Serialize;
use serde_json::Value;

use super::{
    paths::{checked_entry, display_path, open_input},
    query::SUMMARY_COLUMNS,
    store, AssetSummary, Index, IndexError,
};
use crate::{
    consts::MANIFEST,
    layout::expected_type,
    manifest::{self, MetadataRole},
    naming::check_rel,
    scan::{is_reparse, mtime_ns, Record},
    schema::Schemas,
};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct LibraryMeta {
    pub root: String,
    /// Unix seconds.
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub last_scan: i64,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct AssetRef {
    pub id: String,
    pub field: String,
    pub name: Option<String>,
    pub asset_type: Option<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct AssetDetail {
    pub summary: AssetSummary,
    pub manifest: Value,
    pub base_hash: String,
    pub absolute_path: String,
    pub file_count: usize,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub total_bytes: u64,
    pub outgoing: Vec<AssetRef>,
    pub incoming: Vec<AssetRef>,
    /// Paths relative to the song directory, including files not described in manifest.audio.
    pub audio_files: Vec<String>,
    pub model_info: Vec<super::ModelFileInfo>,
    pub motion_info: Vec<super::MotionFileInfo>,
}

pub(super) fn active_root(conn: &Connection, root: &Path) -> Result<PathBuf, IndexError> {
    let root = root.canonicalize()?;
    let indexed: Option<String> = conn
        .query_row("SELECT root FROM meta WHERE id=1", [], |row| row.get(0))
        .optional()?;
    if indexed.as_deref().map(Path::new) != Some(root.as_path()) {
        return Err(IndexError::Invalid("索引与当前库不一致，请重新扫描".into()));
    }
    Ok(root)
}

fn validate_asset_dir(dir: &str, asset_type: &str) -> Result<(), IndexError> {
    if check_rel(dir).is_err() || expected_type(dir) != Some(asset_type) {
        return Err(IndexError::Invalid(format!("无效的资产目录：{dir}")));
    }
    Ok(())
}

struct Snapshot {
    value: Value,
    base_hash: String,
    directory: PathBuf,
}

enum IndexedEntry<'a> {
    Directory,
    Manifest,
    Payload(&'a str),
}

/// Read one file under the current index identity. Sync its metadata in this same
/// transaction so the detail, search fields and optimistic base describe the same bytes.
fn current_manifest(tx: &Transaction<'_>, root: &Path, id: &str) -> Result<Option<Snapshot>, IndexError> {
    let cached = tx
        .query_row(
            "SELECT rel_dir,type,manifest_json,manifest_mtime,manifest_size FROM assets WHERE id=?",
            [id],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    (r.get::<_, i64>(3)?, r.get::<_, i64>(4)?),
                ))
            },
        )
        .optional()?;
    let Some((dir, ty, json, cached_stamp)) = cached else {
        return Ok(None);
    };
    validate_asset_dir(&dir, &ty)?;
    let (mut path, _) = checked_entry(root, &dir, Some(MANIFEST))?;
    let mut file = open_input(&path)?;
    let before = file.metadata()?;
    if is_reparse(&before) || !before.is_file() {
        return Err(IndexError::Invalid(format!(
            "manifest 不是普通文件：{}",
            path.display()
        )));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let after = file.metadata()?;
    if before.modified()? != after.modified()?
        || bytes.len() as u64 != before.len()
        || before.len() != after.len()
    {
        return Err(IndexError::Invalid(format!(
            "读取期间 manifest 已改变：{}",
            path.display()
        )));
    }
    let stamp = (
        mtime_ns(&before)?,
        i64::try_from(bytes.len()).map_err(|e| IndexError::Invalid(e.to_string()))?,
    );
    let doc = manifest::parse_document(&bytes)?;
    if doc.value["id"].as_str() != Some(id)
        || doc.value["type"] != ty
        || manifest::metadata_role(&doc.value) != MetadataRole::Asset
    {
        return Err(IndexError::Invalid(format!(
            "磁盘上的资产标识或类型已改变，请重新扫描：{dir}"
        )));
    }
    let record = Record {
        dir,
        value: doc.value,
        format: doc.format,
    };
    if serde_json::to_string(&record.value)? != json {
        let schemas = Schemas::load().map_err(|e| IndexError::Invalid(e.to_string()))?;
        let issues = schemas
            .validate_asset(&record.value, 1)
            .map_err(|e| IndexError::Invalid(e.to_string()))?;
        if let Some(issue) = issues.first() {
            return Err(IndexError::Invalid(format!(
                "{}: {} {}",
                record.dir, issue.path, issue.message
            )));
        }
        store::clear_asset_metadata(tx, id)?;
        store::write_asset(tx, &record, id, stamp)?;
    } else if stamp != cached_stamp {
        tx.execute(
            "UPDATE assets SET manifest_mtime=?, manifest_size=? WHERE id=?",
            params![stamp.0, stamp.1, id],
        )?;
    }
    path.pop();
    Ok(Some(Snapshot {
        value: record.value,
        base_hash: blake3::hash(&bytes).to_hex().to_string(),
        directory: path,
    }))
}

// Look only at indexed payloads owned by this asset. Explicit previews retain manifest order.
const PREVIEWS: &str = "
    SELECT f.rel AS rel, 0 AS priority, CAST(p.key AS INTEGER) AS ordinal
    FROM assets a JOIN json_each(a.manifest_json, '$.preview') p
      JOIN files f ON f.asset_id=a.id AND f.rel=p.value COLLATE NOCASE
    WHERE a.id=?1 AND p.type='text'
    UNION ALL
    SELECT rel, 1, CASE lower(rel) WHEN 'preview.png' THEN 0 ELSE 1 END
    FROM files WHERE asset_id=?1 AND rel COLLATE NOCASE IN ('preview.png','preview.jpg')
    ORDER BY priority, ordinal, rel";

fn set_preview(
    root: &Path,
    item: &mut AssetSummary,
    statement: &mut Statement<'_>,
) -> Result<(), IndexError> {
    validate_asset_dir(&item.rel_dir, &item.asset_type)?;
    item.preview_path = None;
    let mut rows = statement.query([&item.id])?;
    while let Some(row) = rows.next()? {
        let rel: String = row.get(0)?;
        let extension = Path::new(&rel).extension().and_then(|s| s.to_str()).unwrap_or("");
        if !["png", "jpg", "jpeg", "webp", "gif", "bmp", "avif"]
            .iter()
            .any(|ext| ext.eq_ignore_ascii_case(extension))
            || check_rel(&rel).is_err()
        {
            continue;
        }
        // Stale, missing and reparse-point images fall back to the next candidate or placeholder.
        if let Ok((image, _)) = checked_entry(root, &item.rel_dir, Some(&rel)) {
            item.preview_path = Some(display_path(&image));
            break;
        }
    }
    Ok(())
}

fn references(conn: &Connection, id: &str, incoming: bool) -> Result<Vec<AssetRef>, IndexError> {
    let (owner, target) = if incoming {
        ("to_id", "from_id")
    } else {
        ("from_id", "to_id")
    };
    let mut query = conn.prepare_cached(&format!(
        "SELECT r.{target}, r.field, coalesce(nullif(a.name,''),a.title,a.rel_dir), a.type
         FROM refs r LEFT JOIN assets a ON a.id=r.{target} WHERE r.{owner}=? ORDER BY r.field,r.{target}"
    ))?;
    let rows = query.query_map([id], |row| {
        Ok(AssetRef {
            id: row.get(0)?,
            field: row.get(1)?,
            name: row.get(2)?,
            asset_type: row.get(3)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

impl Index {
    pub fn library_meta(&self) -> Result<Option<LibraryMeta>, IndexError> {
        Ok(self
            .conn
            .query_row("SELECT root,last_scan FROM meta WHERE id=1", [], |row| {
                Ok(LibraryMeta {
                    root: display_path(Path::new(&row.get::<_, String>(0)?)),
                    last_scan: row.get(1)?,
                })
            })
            .optional()?)
    }

    /// Resolve a directory from its cached asset ID, using the caller's active root as the boundary.
    pub fn asset_directory(&self, root: &Path, id: &str) -> Result<Option<PathBuf>, IndexError> {
        self.indexed_path(root, id, IndexedEntry::Directory)
    }

    pub fn manifest_path(&self, root: &Path, id: &str) -> Result<Option<PathBuf>, IndexError> {
        self.indexed_path(root, id, IndexedEntry::Manifest)
    }

    /// Parent directory of a payload owned by this asset in the active index.
    pub fn payload_directory(&self, root: &Path, id: &str, rel: &str) -> Result<Option<PathBuf>, IndexError> {
        Ok(self
            .indexed_path(root, id, IndexedEntry::Payload(rel))?
            .map(|mut file| {
                file.pop();
                file
            }))
    }

    fn indexed_path(
        &self,
        root: &Path,
        id: &str,
        entry: IndexedEntry<'_>,
    ) -> Result<Option<PathBuf>, IndexError> {
        let tx = self.conn.unchecked_transaction()?;
        let root = active_root(&tx, root)?;
        let (file, payload) = match entry {
            IndexedEntry::Directory => (None, None),
            IndexedEntry::Manifest => (Some(MANIFEST), None),
            IndexedEntry::Payload(rel) => {
                check_rel(rel).map_err(|e| IndexError::Invalid(e.to_string()))?;
                (Some(rel), Some(rel))
            }
        };
        let row: Option<(String, String)> = tx
            .query_row(
                "SELECT rel_dir,type FROM assets WHERE id=?1
                AND (?2 IS NULL OR EXISTS(SELECT 1 FROM files WHERE asset_id=?1 AND rel=?2))",
                params![id, payload],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        let Some((dir, ty)) = row else {
            return Ok(None);
        };
        validate_asset_dir(&dir, &ty)?;
        let (path, _) = checked_entry(&root, &dir, file)?;
        tx.commit()?;
        Ok(Some(PathBuf::from(display_path(&path))))
    }

    pub fn load_previews(&self, root: &Path, items: &mut [AssetSummary]) -> Result<(), IndexError> {
        let tx = self.conn.unchecked_transaction()?;
        let root = active_root(&tx, root)?;
        {
            let mut statement = tx.prepare_cached(PREVIEWS)?;
            for item in items {
                set_preview(&root, item, &mut statement)?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn asset_detail(&self, root: &Path, id: &str) -> Result<Option<AssetDetail>, IndexError> {
        let tx = Transaction::new_unchecked(&self.conn, TransactionBehavior::Immediate)?;
        let root = active_root(&tx, root)?;
        let Some(snapshot) = current_manifest(&tx, &root, id)? else {
            return Ok(None);
        };
        let mut summary = tx.query_row(
            &format!("SELECT {SUMMARY_COLUMNS} FROM assets a WHERE a.id=?"),
            [id],
            AssetSummary::from_row,
        )?;
        {
            let mut previews = tx.prepare_cached(PREVIEWS)?;
            set_preview(&root, &mut summary, &mut previews)?;
        }
        let (file_count, total_bytes): (i64, i64) = tx.query_row(
            "SELECT count(*), coalesce(sum(size),0) FROM files WHERE asset_id=?",
            [id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        let audio_files =
            if summary.asset_type == "song" {
                tx.prepare_cached(
                "SELECT rel FROM files WHERE asset_id=? AND lower(substr(rel,1,7))='_audio/' ORDER BY rel"
            )?.query_map([id], |row| row.get(0))?.collect::<rusqlite::Result<_>>()?
            } else {
                Vec::new()
            };
        let detail = AssetDetail {
            summary,
            manifest: snapshot.value,
            base_hash: snapshot.base_hash,
            absolute_path: display_path(&snapshot.directory),
            file_count: usize::try_from(file_count).map_err(|e| IndexError::Invalid(e.to_string()))?,
            total_bytes: u64::try_from(total_bytes).map_err(|e| IndexError::Invalid(e.to_string()))?,
            incoming: references(&tx, id, true)?,
            outgoing: references(&tx, id, false)?,
            audio_files,
            model_info: super::formats::models(&tx, id)?,
            motion_info: super::formats::motions(&tx, id)?,
        };
        tx.commit()?;
        Ok(Some(detail))
    }

    /// Synchronize one existing asset after an edit or recovery. Its payload files and
    /// their cached hashes belong to the asset directory and need no new enumeration.
    pub fn refresh_manifest(&mut self, root: &Path, id: &str) -> Result<(), IndexError> {
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let root = active_root(&tx, root)?;
        current_manifest(&tx, &root, id)?
            .ok_or_else(|| IndexError::Invalid(format!("索引中没有此资产：{id}")))?;
        Ok(tx.commit()?)
    }
}
