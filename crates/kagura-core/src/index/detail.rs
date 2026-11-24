use std::path::{Path, PathBuf};

use rusqlite::{Connection, OptionalExtension, Statement};
use serde::Serialize;
use serde_json::Value;

use super::{
    paths::{checked_entry, display_path},
    query::SUMMARY_COLUMNS,
    AssetSummary, Index, IndexError,
};
use crate::{layout::expected_type, naming::check_rel};

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
    pub absolute_path: String,
    pub file_count: usize,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub total_bytes: u64,
    pub outgoing: Vec<AssetRef>,
    pub incoming: Vec<AssetRef>,
    /// Paths relative to the song directory, including files not described in manifest.audio.
    pub audio_files: Vec<String>,
}

fn active_root(conn: &Connection, root: &Path) -> Result<PathBuf, IndexError> {
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
        let tx = self.conn.unchecked_transaction()?;
        let root = active_root(&tx, root)?;
        let row: Option<(String, String)> = tx
            .query_row("SELECT rel_dir,type FROM assets WHERE id=?", [id], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .optional()?;
        let Some((dir, ty)) = row else {
            return Ok(None);
        };
        validate_asset_dir(&dir, &ty)?;
        let (path, _) = checked_entry(&root, &dir, None)?;
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
        let tx = self.conn.unchecked_transaction()?;
        let root = active_root(&tx, root)?;
        let row = tx
            .query_row(
                &format!("SELECT {SUMMARY_COLUMNS},a.manifest_json FROM assets a WHERE a.id=?"),
                [id],
                |row| {
                    Ok((
                        AssetSummary::from_row(row)?,
                        row.get::<_, String>("manifest_json")?,
                    ))
                },
            )
            .optional()?;
        let Some((mut summary, json)) = row else {
            return Ok(None);
        };
        validate_asset_dir(&summary.rel_dir, &summary.asset_type)?;
        let (path, _) = checked_entry(&root, &summary.rel_dir, None)?;
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
            manifest: serde_json::from_str(&json)?,
            absolute_path: display_path(&path),
            file_count: usize::try_from(file_count).map_err(|e| IndexError::Invalid(e.to_string()))?,
            total_bytes: u64::try_from(total_bytes).map_err(|e| IndexError::Invalid(e.to_string()))?,
            incoming: references(&tx, id, true)?,
            outgoing: references(&tx, id, false)?,
            audio_files,
        };
        tx.commit()?;
        Ok(Some(detail))
    }
}
