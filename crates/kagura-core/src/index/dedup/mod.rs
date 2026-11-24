//! Duplicate file and asset relationships from one consistent SQLite snapshot.
mod assets;
mod groups;

use rusqlite::{Connection, OpenFlags};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use std::io::Write;

use super::{Index, IndexError};
use assets::AssetRelations;
use groups::FileGroups;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct DedupOptions {
    pub include_small: bool,
    pub include_toon: bool,
    pub include_accessories: bool,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub enum DedupSection {
    Cross,
    Within,
    Identical,
    Subsets,
}

#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct DuplicateCounts {
    pub groups: usize,
    /// All participating file paths, including the retained copy in each group.
    pub files: usize,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub reclaim_bytes: u64,
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct ExtensionDuplicates {
    /// Lowercase suffix without the dot, or the empty string for extensionless files.
    pub extension: String,
    pub groups: usize,
    pub files: usize,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub reclaim_bytes: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct DedupSummary {
    pub total_files: usize,
    pub hashed_files: usize,
    pub total_assets: usize,
    /// Non-empty assets for which every payload file has a hash.
    pub complete_assets: usize,
    pub identical_groups: usize,
    pub identical_assets: usize,
    /// Ordered (proper subset, superset) asset pairs.
    pub subset_pairs: usize,
    /// Hashed files hidden by the current file filters, including unique files.
    pub excluded_files: usize,
    pub all: DuplicateCounts,
    pub cross: DuplicateCounts,
    pub within: DuplicateCounts,
    pub extensions: Vec<ExtensionDuplicates>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct DuplicateAsset {
    pub id: String,
    pub name: String,
    pub rel_dir: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct DuplicateCopy {
    pub asset: DuplicateAsset,
    pub rel: String,
    pub exclusions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct DuplicateFileGroup {
    pub hash: String,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub size: u64,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub reclaim_bytes: u64,
    pub copies: Vec<DuplicateCopy>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct IdenticalAssetGroup {
    pub unique_files: usize,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub unique_bytes: u64,
    pub assets: Vec<DuplicateAsset>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct AssetSubset {
    pub smaller: DuplicateAsset,
    pub larger: DuplicateAsset,
    pub smaller_files: usize,
    pub larger_files: usize,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct DedupPage {
    pub summary: DedupSummary,
    pub total: usize,
    pub file_groups: Vec<DuplicateFileGroup>,
    pub asset_groups: Vec<IdenticalAssetGroup>,
    pub subsets: Vec<AssetSubset>,
}

pub struct DuplicateReport {
    pub summary: DedupSummary,
    root: String,
    options: DedupOptions,
    files: FileGroups,
    assets: AssetRelations,
}

struct InventoryAsset {
    asset: DuplicateAsset,
    is_stage: bool,
    payload: Vec<usize>,
    total: usize,
    hashed: usize,
}

impl Index {
    /// A cache key for repeated reads on this specific Index connection. SQLite data_version
    /// detects other WAL writers; total_changes covers commits made by this connection.
    pub fn dedup_revision(&self) -> Result<(i64, u64), IndexError> {
        Ok((
            self.conn.query_row("PRAGMA data_version", [], |r| r.get(0))?,
            self.conn.total_changes(),
        ))
    }

    pub fn duplicate_report(&self, options: DedupOptions) -> Result<DuplicateReport, IndexError> {
        DuplicateReport::read(&self.conn, options)
    }
}

impl DuplicateReport {
    pub fn root(&self) -> &std::path::Path {
        std::path::Path::new(&self.root)
    }

    /// Inspect an existing cache without creating or upgrading it.
    pub fn read_cache(path: &std::path::Path, options: DedupOptions) -> Result<Self, IndexError> {
        let db = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        if super::verify_cache(&db, path)? != super::APPLICATION_ID {
            return Err(IndexError::Invalid("not a Kagura cache".into()));
        }
        Self::read(&db, options)
    }

    fn read(db: &Connection, options: DedupOptions) -> Result<Self, IndexError> {
        let tx = db.unchecked_transaction()?;
        let root = tx.query_row("SELECT root FROM meta WHERE id=1", [], |r| r.get(0))?;
        let mut inventory = inventory(&tx)?;
        let mut summary = DedupSummary {
            total_assets: inventory.len(),
            ..Default::default()
        };
        let (files, sizes) = FileGroups::read(&tx, &mut inventory, options, &mut summary)?;
        tx.commit()?;
        let assets = AssetRelations::build(inventory, &sizes, &mut summary);
        Ok(DuplicateReport {
            summary,
            root,
            options,
            files,
            assets,
        })
    }
}

fn inventory(db: &Connection) -> Result<Vec<InventoryAsset>, IndexError> {
    let mut stmt = db.prepare(
        "SELECT id,rel_dir,type,coalesce(nullif(name,''),nullif(title,'')) FROM assets ORDER BY id",
    )?;
    let rows = stmt.query_map([], |r| {
        let rel_dir: String = r.get(1)?;
        let name: Option<String> = r.get(3)?;
        Ok(InventoryAsset {
            asset: DuplicateAsset {
                id: r.get(0)?,
                name: name.unwrap_or_else(|| rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned()),
                rel_dir,
            },
            is_stage: r.get_ref(2)?.as_str()? == "stage",
            payload: Vec::new(),
            total: 0,
            hashed: 0,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

impl DuplicateReport {
    pub fn page(&self, section: DedupSection, offset: usize, limit: usize) -> DedupPage {
        let mut result = DedupPage {
            summary: self.summary.clone(),
            total: 0,
            file_groups: Vec::new(),
            asset_groups: Vec::new(),
            subsets: Vec::new(),
        };
        match section {
            DedupSection::Cross | DedupSection::Within => {
                let groups = if matches!(section, DedupSection::Cross) {
                    &self.files.cross
                } else {
                    &self.files.within
                };
                result.total = groups.len();
                result.file_groups = groups.iter().skip(offset).take(limit).cloned().collect();
            }
            DedupSection::Identical => {
                result.total = self.summary.identical_groups;
                result.asset_groups = self
                    .assets
                    .identical
                    .iter()
                    .skip(offset)
                    .take(limit)
                    .cloned()
                    .collect();
            }
            DedupSection::Subsets => {
                result.total = self.summary.subset_pairs;
                result.subsets = self.assets.pairs_from(offset).take(limit).collect();
            }
        }
        result
    }

    /// Write complete arrays to the supplied writer; even large subset expansions are streamed.
    pub fn write_json(&self, out: impl Write) -> Result<(), IndexError> {
        let mut ser = serde_json::Serializer::new(out);
        let mut report = serde::Serializer::serialize_struct(&mut ser, "DuplicateReport", 7)?;
        report.serialize_field("root", &self.root)?;
        report.serialize_field("options", &self.options)?;
        report.serialize_field("summary", &self.summary)?;
        report.serialize_field("cross", &self.files.cross)?;
        report.serialize_field("within", &self.files.within)?;
        report.serialize_field("identical", &self.assets.identical)?;
        report.serialize_field("subsets", &self.assets)?;
        report.end()?;
        Ok(())
    }
}
