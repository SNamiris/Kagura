use super::{
    DedupOptions, DedupSummary, DuplicateCopy, DuplicateCounts, DuplicateFileGroup, ExtensionDuplicates,
    IndexError, InventoryAsset,
};
use rusqlite::Connection;
use std::collections::{HashMap, HashSet};
use std::path::Path;

const SMALL_FILE: u64 = 4 * 1024;
const CONTROLLER_MAX: u64 = 64 * 1024;
const CONTROLLER_WORDS: &[&str] = &["controller", "コントローラ", "控制器"];
const STANDARD_TOONS: &[&str] = &[
    "toon01.bmp",
    "toon02.bmp",
    "toon03.bmp",
    "toon04.bmp",
    "toon05.bmp",
    "toon06.bmp",
    "toon07.bmp",
    "toon08.bmp",
    "toon09.bmp",
    "toon10.bmp",
];

#[derive(Default)]
pub(super) struct FileGroups {
    pub cross: Vec<DuplicateFileGroup>,
    pub within: Vec<DuplicateFileGroup>,
}

#[derive(Default)]
struct Exclusions {
    small: bool,
    toon: bool,
    accessory: bool,
}

impl Exclusions {
    fn classify(rel: &str, size: u64, stage: bool) -> Self {
        let name = rel.rsplit('/').next().unwrap_or(rel).to_lowercase();
        let accessory = stage
            && size <= CONTROLLER_MAX
            && (name.ends_with(".pmx") || name.ends_with(".pmd"))
            && CONTROLLER_WORDS.iter().any(|word| name.contains(word));
        Self {
            small: size < SMALL_FILE,
            toon: STANDARD_TOONS.contains(&name.as_str()),
            accessory,
        }
    }
    fn hidden(&self, options: DedupOptions) -> bool {
        (self.small && !options.include_small)
            || (self.toon && !options.include_toon)
            || (self.accessory && !options.include_accessories)
    }
    fn labels(&self) -> Vec<String> {
        [
            ("small", self.small),
            ("toon", self.toon),
            ("accessory", self.accessory),
        ]
        .into_iter()
        .filter(|(_, yes)| *yes)
        .map(|(name, _)| name.into())
        .collect()
    }
}

struct CopyInfo {
    owner: usize,
    rel: String,
    exclusions: Exclusions,
}
struct Digest {
    size: u64,
    kept: Vec<CopyInfo>,
}

impl FileGroups {
    pub fn read(
        db: &Connection,
        assets: &mut [InventoryAsset],
        options: DedupOptions,
        summary: &mut DedupSummary,
    ) -> Result<(Self, Vec<u64>), IndexError> {
        let ids: HashMap<String, usize> = assets
            .iter()
            .enumerate()
            .map(|(i, a)| (a.asset.id.clone(), i))
            .collect();
        let mut by_hash: HashMap<String, usize> = HashMap::new();
        let mut data: Vec<Digest> = Vec::new();
        let mut stmt = db.prepare("SELECT asset_id,rel,size,blake3 FROM files ORDER BY asset_id,rel")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let owner = *ids
                .get(&id)
                .ok_or_else(|| IndexError::Invalid(format!("missing owner {id}")))?;
            let asset = &mut assets[owner];
            summary.total_files += 1;
            asset.total += 1;
            let Some(hash) = row.get::<_, Option<String>>(3)? else {
                continue;
            };
            let rel: String = row.get(1)?;
            let size: u64 = u64::try_from(row.get::<_, i64>(2)?)
                .map_err(|_| IndexError::Invalid("negative cached file size".into()))?;
            let key = *by_hash.entry(hash).or_insert_with(|| {
                data.push(Digest {
                    size,
                    kept: Vec::new(),
                });
                data.len() - 1
            });
            let digest = &mut data[key];
            if digest.size != size {
                return Err(IndexError::Invalid(
                    "one BLAKE3 has conflicting cached file sizes".into(),
                ));
            }
            summary.hashed_files += 1;
            asset.hashed += 1;
            asset.payload.push(key);
            let exclusions = Exclusions::classify(&rel, size, asset.is_stage);
            if exclusions.hidden(options) {
                summary.excluded_files += 1
            } else {
                digest.kept.push(CopyInfo {
                    owner,
                    rel,
                    exclusions,
                });
            }
        }
        let sizes = data.iter().map(|d| d.size).collect();
        let mut result = Self::default();
        let mut extensions: HashMap<String, DuplicateCounts> = HashMap::new();
        for (hash, key) in by_hash {
            let digest = &mut data[key];
            if digest.kept.len() < 2 {
                continue;
            }
            // Keep the lexicographically first (asset directory, relative path) for attribution.
            digest.kept.sort_unstable_by(|l, r| {
                (&assets[l.owner].asset.rel_dir, &l.rel).cmp(&(&assets[r.owner].asset.rel_dir, &r.rel))
            });
            let first = digest.kept[0].owner;
            let cross = digest.kept.iter().any(|copy| copy.owner != first);
            let count = digest.kept.len();
            let reclaim_bytes = (count - 1) as u64 * digest.size;
            for totals in [
                &mut summary.all,
                if cross {
                    &mut summary.cross
                } else {
                    &mut summary.within
                },
            ] {
                totals.groups += 1;
                totals.files += count;
                totals.reclaim_bytes += reclaim_bytes;
            }
            let mut seen = HashSet::new();
            let mut copies = Vec::with_capacity(count);
            for (i, copy) in digest.kept.drain(..).enumerate() {
                let ext = Path::new(&copy.rel)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or_default()
                    .to_lowercase();
                let is_first = seen.insert(ext.clone());
                let totals = extensions.entry(ext).or_default();
                totals.groups += usize::from(is_first);
                totals.files += 1;
                if i > 0 {
                    totals.reclaim_bytes += digest.size
                }
                copies.push(DuplicateCopy {
                    asset: assets[copy.owner].asset.clone(),
                    rel: copy.rel,
                    exclusions: copy.exclusions.labels(),
                });
            }
            let bucket = if cross {
                &mut result.cross
            } else {
                &mut result.within
            };
            bucket.push(DuplicateFileGroup {
                hash,
                size: digest.size,
                reclaim_bytes,
                copies,
            });
        }
        for groups in [&mut result.cross, &mut result.within] {
            groups.sort_unstable_by(|a, b| {
                b.reclaim_bytes
                    .cmp(&a.reclaim_bytes)
                    .then_with(|| a.hash.cmp(&b.hash))
            });
        }
        summary.extensions = extensions
            .into_iter()
            .map(|(extension, c)| ExtensionDuplicates {
                extension,
                groups: c.groups,
                files: c.files,
                reclaim_bytes: c.reclaim_bytes,
            })
            .collect();
        summary.extensions.sort_unstable_by(|a, b| {
            b.reclaim_bytes
                .cmp(&a.reclaim_bytes)
                .then_with(|| a.extension.cmp(&b.extension))
        });
        Ok((result, sizes))
    }
}
