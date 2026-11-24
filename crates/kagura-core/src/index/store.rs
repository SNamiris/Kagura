use std::{
    collections::{HashMap, HashSet},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use serde_json::Value;

use super::{IndexChanges, IndexError};
use crate::{
    check::check_references,
    consts::MANIFEST,
    layout,
    scan::{scan_for_update, FileStamp, Record, ScanReport, SkipReason},
};

type Result<T> = std::result::Result<T, IndexError>;

struct CachedAsset {
    id: String,
    stamp: FileStamp,
}

fn validate_report(report: &ScanReport) -> Result<()> {
    if let Some(first) = report.errors.first() {
        return Err(IndexError::Invalid(format!(
            "{} scan error(s), first at {}: {}",
            report.errors.len(),
            first.path,
            first.message
        )));
    }
    if report.pruned.iter().any(|item| item.path == "Library") {
        return Err(IndexError::Invalid("Library is a reparse point".into()));
    }
    Ok(())
}

pub(super) fn rebuild(conn: &mut Connection, root: &Path, scan: &ScanReport) -> Result<IndexChanges> {
    validate_report(scan)?;
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    clear(&tx)?;
    // The public, manifest-only scan prunes e.g. _audio. A check scan already
    // inventories those trees. Reuse validated documents in both cases.
    let extra;
    let stamps = if scan.inventory_reserved || !scan.pruned.iter().any(|s| s.reason == SkipReason::Reserved) {
        &scan.stamps
    } else {
        let known = scan
            .records
            .iter()
            .map(|record| manifest_path(&record.dir))
            .chain(skipped_manifests(scan).map(str::to_owned))
            .map(|path| {
                let stamp = *scan
                    .stamps
                    .get(&path)
                    .ok_or_else(|| IndexError::Invalid(path.clone()))?;
                Ok((path, stamp))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        extra = scan_for_update(root, &known);
        validate_report(&extra)?;
        if !extra.records.is_empty()
            || known
                .iter()
                .any(|(path, stamp)| extra.stamps.get(path) != Some(stamp))
        {
            return Err(IndexError::Invalid(
                "library manifests changed during rebuild; rescan the library".into(),
            ));
        }
        &extra.stamps
    };
    let changes = synchronize(&tx, root, &HashMap::new(), &scan.records, stamps)?;
    cache_skipped(&tx, scan)?;
    tx.commit()?;
    Ok(changes)
}

pub(super) fn update(conn: &mut Connection, root: &Path) -> Result<IndexChanges> {
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let previous: Option<String> = tx
        .query_row("SELECT root FROM meta WHERE id=1", [], |row| row.get(0))
        .optional()?;
    if previous.as_deref().is_some_and(|p| Path::new(p) != root) {
        clear(&tx)?;
    }
    let cache = tx
        .prepare("SELECT rel_dir,id,manifest_mtime,manifest_size FROM assets")?
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                CachedAsset {
                    id: row.get(1)?,
                    stamp: (row.get(2)?, row.get(3)?),
                },
            ))
        })?
        .collect::<rusqlite::Result<HashMap<_, _>>>()?;
    let ignored = tx
        .prepare("SELECT path,mtime,size FROM skipped_manifests")?
        .query_map([], |r| Ok((r.get::<_, String>(0)?, (r.get(1)?, r.get(2)?))))?
        .collect::<rusqlite::Result<HashMap<_, FileStamp>>>()?;
    let known = cache
        .iter()
        .map(|(dir, asset)| (manifest_path(dir), asset.stamp))
        .chain(ignored.iter().map(|(path, stamp)| (path.clone(), *stamp)))
        .collect();
    let scan = scan_for_update(root, &known);
    validate_report(&scan)?;
    let changes = synchronize(&tx, root, &cache, &scan.records, &scan.stamps)?;
    for (path, stamp) in &ignored {
        if scan.stamps.get(path) != Some(stamp) {
            tx.prepare_cached("DELETE FROM skipped_manifests WHERE path=?")?
                .execute([path])?;
        }
    }
    cache_skipped(&tx, &scan)?;
    tx.commit()?;
    Ok(changes)
}

fn clear(tx: &Transaction<'_>) -> Result<()> {
    tx.execute_batch(
        "DELETE FROM assets_fts; DELETE FROM assets; DELETE FROM meta; DELETE FROM skipped_manifests;",
    )?;
    Ok(())
}

fn skipped_manifests(scan: &ScanReport) -> impl Iterator<Item = &str> {
    scan.skipped
        .iter()
        .filter(|s| {
            matches!(
                s.reason,
                SkipReason::SourceJson | SkipReason::RuntimeConfig | SkipReason::ForeignManifest
            )
        })
        .map(|s| s.path.as_str())
}

fn cache_skipped(tx: &Transaction<'_>, scan: &ScanReport) -> Result<()> {
    let mut insert = tx.prepare_cached("INSERT INTO skipped_manifests(path,mtime,size) VALUES(?,?,?)")?;
    for path in skipped_manifests(scan) {
        let stamp = scan
            .stamps
            .get(path)
            .ok_or_else(|| IndexError::Invalid(format!("missing stamp: {path}")))?;
        insert.execute(params![path, stamp.0, stamp.1])?;
    }
    Ok(())
}

fn manifest_path(dir: &str) -> String {
    format!("{dir}/{MANIFEST}")
}

fn record_id(record: &Record) -> Result<&str> {
    record.value["id"]
        .as_str()
        .ok_or_else(|| IndexError::Invalid(format!("missing id in {}", record.dir)))
}

fn synchronize(
    tx: &Transaction<'_>,
    root: &Path,
    cache: &HashMap<String, CachedAsset>,
    records: &[Record],
    stamps: &HashMap<String, FileStamp>,
) -> Result<IndexChanges> {
    let mut owners: HashMap<&str, &str> = cache
        .iter()
        .filter(|(dir, asset)| stamps.get(&manifest_path(dir)) == Some(&asset.stamp))
        .map(|(dir, asset)| (dir.as_str(), asset.id.as_str()))
        .collect();
    for record in records {
        owners.insert(&record.dir, record_id(record)?);
    }
    let mut ids = HashSet::with_capacity(owners.len());
    for (dir, id) in &owners {
        if !ids.insert(*id) {
            return Err(IndexError::Invalid(format!("duplicate asset id {id} at {dir}")));
        }
    }
    let mut changes = IndexChanges::default();
    // Remove obsolete identities/paths first, so renames and ID swaps work in one transaction.
    for (dir, old) in cache {
        if owners.get(dir.as_str()).copied() != Some(old.id.as_str()) {
            let deleted: i64 = tx
                .prepare_cached("SELECT count(*) FROM files WHERE asset_id=?")?
                .query_row([&old.id], |row| row.get(0))?;
            changes.files_deleted +=
                usize::try_from(deleted).map_err(|e| IndexError::Invalid(e.to_string()))?;
            remove_search_row(tx, &old.id)?;
            tx.prepare_cached("DELETE FROM assets WHERE id=?")?
                .execute([&old.id])?;
            changes.deleted += 1;
        }
    }
    for record in records {
        let id = record_id(record)?;
        if cache.get(&record.dir).is_some_and(|asset| asset.id == id) {
            clear_asset_metadata(tx, id)?;
            changes.updated += 1;
        } else {
            changes.inserted += 1;
        }
        let path = manifest_path(&record.dir);
        let stamp = stamps
            .get(&path)
            .ok_or_else(|| IndexError::Invalid(format!("missing scanned manifest stamp: {path}")))?;
        write_asset(tx, record, id, *stamp)?;
    }
    write_files(tx, stamps, &owners, &mut changes)?;
    // Textures may be shared with an unowned sibling directory. Include scanned regular files
    // outside the assets table when deciding whether cached texture existence needs a new check.
    let mut paths: Vec<_> = stamps.keys().collect();
    paths.sort_unstable();
    let mut digest = blake3::Hasher::new();
    for path in paths {
        digest.update(path.as_bytes()).update(&[0]);
    }
    tx.execute(
        "UPDATE parse_state SET inventory_revision=inventory_revision+1,path_digest=?1
         WHERE path_digest!=?1 OR ?2",
        params![
            digest.finalize().to_hex().as_str(),
            changes.files_changed != 0 || changes.files_deleted != 0
        ],
    )?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| IndexError::Invalid(e.to_string()))?;
    let root = root
        .to_str()
        .ok_or_else(|| IndexError::Invalid("non-Unicode library root".into()))?;
    tx.execute(
        "INSERT OR REPLACE INTO meta(id,root,last_scan,asset_count,file_count)
         VALUES(1,?,?,(SELECT count(*) FROM assets),(SELECT count(*) FROM files))",
        params![
            root,
            i64::try_from(now.as_secs()).map_err(|e| IndexError::Invalid(e.to_string()))?
        ],
    )?;
    Ok(changes)
}

fn remove_search_row(tx: &Transaction<'_>, id: &str) -> Result<()> {
    tx.prepare_cached("DELETE FROM assets_fts WHERE rowid=(SELECT rowid FROM assets WHERE id=?)")?
        .execute([id])?;
    Ok(())
}

pub(super) fn clear_asset_metadata(tx: &Transaction<'_>, id: &str) -> Result<()> {
    remove_search_row(tx, id)?;
    tx.prepare_cached("DELETE FROM refs WHERE from_id=?")?
        .execute([id])?;
    tx.prepare_cached("DELETE FROM tags WHERE id=?")?.execute([id])?;
    tx.prepare_cached("DELETE FROM model_variants WHERE asset_id=?")?
        .execute([id])?;
    Ok(())
}

fn strings(value: &Value) -> impl Iterator<Item = &str> {
    value.as_array().into_iter().flatten().filter_map(Value::as_str)
}

pub(super) fn write_asset(tx: &Transaction<'_>, record: &Record, id: &str, stamp: FileStamp) -> Result<()> {
    let v = &record.value;
    let key = &v["key"];
    let author = key["author"].as_str();
    let name = v["name"].as_str().or(key["name"].as_str());
    let license = ["commercial", "modify", "redistribute"]
        .iter()
        .any(|field| v["license"][field].is_boolean());
    let rowid: i64 = tx.prepare_cached(
        "INSERT INTO assets(id,type,kind,rel_dir,author,name,name_original,title,composer,edition,part,suffix,
          song_id,character,series,rating,added,updated,license_explicit,manifest_mtime,manifest_size,manifest_json)
         VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
         ON CONFLICT(id) DO UPDATE SET type=excluded.type,kind=excluded.kind,rel_dir=excluded.rel_dir,
          author=excluded.author,name=excluded.name,name_original=excluded.name_original,title=excluded.title,
          composer=excluded.composer,edition=excluded.edition,part=excluded.part,suffix=excluded.suffix,
          song_id=excluded.song_id,character=excluded.character,series=excluded.series,rating=excluded.rating,
          added=excluded.added,updated=excluded.updated,license_explicit=excluded.license_explicit,
          manifest_mtime=excluded.manifest_mtime,manifest_size=excluded.manifest_size,manifest_json=excluded.manifest_json
         RETURNING rowid",
    )?.query_row(params![
        id, v["type"].as_str(), v["kind"].as_str(), &record.dir, author, name, v["name_original"].as_str(),
        key["title"].as_str(), key["composer"].as_str(), key["edition"].as_str(), key["part"].as_str(),
        key["suffix"].as_str(), v["song"].as_str(), v["character"].as_str(), v["series"].as_str(),
        v["rating"].as_i64(), v["added"].as_str(), v["updated"].as_str(), license, stamp.0, stamp.1,
        serde_json::to_string(v)?,
    ], |row| row.get(0))?;
    let tags = strings(&v["tags"]).collect::<Vec<_>>();
    if v["type"] == "model" {
        let mut insert =
            tx.prepare_cached("INSERT OR IGNORE INTO model_variants(asset_id,file_key) VALUES(?,?)")?;
        for file in v["variants"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|variant| variant["file"].as_str())
        {
            insert.execute(params![id, super::formats::file_key(file)])?;
        }
    }
    let names = name
        .into_iter()
        .chain(key["name"].as_str().filter(|&other| Some(other) != name))
        .collect::<Vec<_>>()
        .join("\n");
    let authors = author
        .into_iter()
        .chain(strings(&v["author"]).filter(|&other| Some(other) != author))
        .collect::<Vec<_>>()
        .join("\n");
    tx.prepare_cached(
        "INSERT INTO assets_fts(rowid,name,name_original,author,character,series,title,title_alias,tags,note)
         VALUES(?,?,?,?,?,?,?,?,?,?)",
    )?
    .execute(params![
        rowid,
        names,
        v["name_original"].as_str(),
        authors,
        v["character"].as_str(),
        v["series"].as_str(),
        key["title"].as_str(),
        strings(&v["title_alias"]).collect::<Vec<_>>().join("\n"),
        tags.join("\n"),
        v["note"].as_str()
    ])?;
    let mut add_tag = tx.prepare_cached("INSERT INTO tags(id,tag) VALUES(?,?)")?;
    for tag in tags {
        add_tag.execute(params![id, tag])?;
    }
    let mut references = Vec::new();
    check_references(record, |field, target| {
        references.push((field.to_owned(), target.to_owned()))
    });
    let mut add_ref = tx.prepare_cached("INSERT INTO refs(from_id,to_id,field) VALUES(?,?,?)")?;
    for (field, target) in references {
        add_ref.execute(params![id, target, field])?;
    }
    Ok(())
}

fn file_owner<'a, 'b>(path: &'a str, owners: &HashMap<&str, &'b str>) -> Option<(&'b str, &'a str)> {
    let (mut dir, _) = path.rsplit_once('/')?;
    loop {
        if let Some(&id) = owners.get(dir) {
            let rel = &path[dir.len() + 1..];
            if rel == MANIFEST
                || (rel.contains('/')
                    && layout::expected_type(dir) == Some("song")
                    && !layout::is_song_reserved(path))
            {
                return None;
            }
            return Some((id, rel));
        }
        dir = dir.rsplit_once('/')?.0;
    }
}

fn write_files(
    tx: &Transaction<'_>,
    stamps: &HashMap<String, FileStamp>,
    owners: &HashMap<&str, &str>,
    changes: &mut IndexChanges,
) -> Result<()> {
    let mut old: HashMap<String, HashMap<String, FileStamp>> = HashMap::new();
    let mut previous = tx.prepare("SELECT asset_id,rel,mtime,size FROM files")?;
    let mut rows = previous.query([])?;
    while let Some(row) = rows.next()? {
        old.entry(row.get(0)?)
            .or_default()
            .insert(row.get(1)?, (row.get(2)?, row.get(3)?));
    }
    let mut save = tx.prepare(
        "INSERT INTO files(asset_id,rel,mtime,size) VALUES(?,?,?,?)
         ON CONFLICT(asset_id,rel) DO UPDATE SET mtime=excluded.mtime,size=excluded.size,blake3=NULL",
    )?;
    for (path, stamp) in stamps {
        if let Some((id, rel)) = file_owner(path, owners) {
            if old.get_mut(id).and_then(|files| files.remove(rel)).as_ref() != Some(stamp) {
                save.execute(params![id, rel, stamp.0, stamp.1])?;
                changes.files_changed += 1;
            }
        }
    }
    let mut remove = tx.prepare("DELETE FROM files WHERE asset_id=? AND rel=?")?;
    for (id, files) in old {
        for rel in files.keys() {
            remove.execute(params![id, rel])?;
            changes.files_deleted += 1;
        }
    }
    Ok(())
}
