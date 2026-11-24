use std::{
    collections::HashMap,
    fs::{self, File, Metadata},
    path::Path,
};

use rusqlite::{params, Connection, TransactionBehavior};

use super::{cache, CachedParseError, ParseProgress};
use crate::{
    index::{
        detail::active_root,
        paths::{checked_entry, open_input},
        Index, IndexError,
    },
    naming::check_rel,
    parse::{parse_model_file, parse_motion_file, ModelInfo, MotionInfo, ParseError},
    scan::{is_reparse, mtime_ns},
};

pub(super) struct Candidate {
    pub id: String,
    dir: String,
    pub rel: String,
    pub format: String,
    mtime: i64,
    size: i64,
}

pub(super) struct Texture {
    pub rel: Option<String>,
    pub present: bool,
}

pub(super) enum Parsed {
    Model(ModelInfo, Vec<Texture>),
    Motion(MotionInfo),
}

type Outcome = Result<Parsed, CachedParseError>;

impl From<ParseError> for CachedParseError {
    fn from(e: ParseError) -> Self {
        Self {
            section: e.section.into(),
            offset: e.offset,
            message: e.message,
        }
    }
}

fn io_error(e: impl std::fmt::Display) -> CachedParseError {
    CachedParseError {
        section: "file".into(),
        offset: 0,
        message: e.to_string(),
    }
}

/// All formats use Windows-style texture paths even on other hosts. A parent component can
/// reach a shared texture inside Library. Absolute, device and escaping paths are unavailable.
fn texture_target(dir: &str, file: &str, name: &str) -> Option<String> {
    let name = name.replace('\\', "/");
    if name.is_empty() || name.starts_with('/') || name.ends_with('/') {
        return None;
    }
    let mut components: Vec<_> = dir.strip_prefix("Library/")?.split('/').collect();
    if let Some((parent, _)) = file.rsplit_once('/') {
        components.extend(parent.split('/'));
    }
    for s in name.split('/') {
        match s {
            "" | "." => (),
            ".." => {
                components.pop()?;
            }
            _ => {
                check_rel(s).ok()?;
                components.push(s);
            }
        }
    }
    (!components.is_empty()).then(|| components.join("/"))
}

fn texture_present(root: &Path, rel: &str, memo: &mut HashMap<String, bool>) -> bool {
    if let Some(&present) = memo.get(rel) {
        return present;
    }
    let present = checked_entry(root, "Library", Some(rel)).is_ok();
    memo.insert(rel.to_owned(), present);
    present
}

impl Candidate {
    fn matches(&self, meta: &Metadata) -> bool {
        meta.is_file()
            && !is_reparse(meta)
            && u64::try_from(self.size) == Ok(meta.len())
            && mtime_ns(meta).ok() == Some(self.mtime)
    }

    fn read(&self, root: &Path, textures: &mut HashMap<String, bool>) -> Outcome {
        let (path, _) = checked_entry(root, &self.dir, Some(&self.rel)).map_err(io_error)?;
        let input: File = open_input(&path).map_err(io_error)?;
        if !self.matches(&input.metadata().map_err(io_error)?) {
            return Err(io_error("文件与清单的大小或修改时间不符，请重新扫描"));
        }
        let decoded = if self.format == "vmd" {
            parse_motion_file(input).map(Parsed::Motion)
        } else {
            parse_model_file(input, &self.format).map(|model| {
                let states = model
                    .textures
                    .iter()
                    .map(|name| {
                        let rel = texture_target(&self.dir, &self.rel, name);
                        let present = rel
                            .as_deref()
                            .is_some_and(|rel| texture_present(root, rel, textures));
                        Texture { rel, present }
                    })
                    .collect();
                Parsed::Model(model, states)
            })
        };
        if !self.matches(&fs::symlink_metadata(path).map_err(io_error)?) {
            return Err(io_error("文件在解析期间发生变化，请重新扫描"));
        }
        Ok(decoded?)
    }
}

fn assert_revision(db: &Connection, root: &Path, rev: i64) -> Result<(), IndexError> {
    let mut statement = db.prepare_cached(
        "SELECT EXISTS(SELECT 1 FROM meta,parse_state WHERE meta.id=1 AND meta.root=? AND inventory_revision=?)",
    )?;
    let valid: bool = statement.query_row(params![root.to_str(), rev], |row| row.get(0))?;
    if !valid {
        return Err(IndexError::Invalid("库根或文件清单已变化，请重新解析".into()));
    }
    Ok(())
}

fn flush(
    db: &mut Connection,
    root: &Path,
    rev: i64,
    batch: &mut Vec<(&Candidate, Outcome)>,
) -> Result<(), IndexError> {
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    assert_revision(&tx, root, rev)?;
    for (file, parsed) in batch.drain(..) {
        // A parallel scan may remove, reparent or change the file, and another parser
        // may already have filled this exact revision while we were reading outside SQLite.
        let current: bool = tx
            .prepare_cached(
                "SELECT EXISTS(SELECT 1 FROM files f JOIN assets a ON f.asset_id=a.id WHERE f.asset_id=?1
             AND f.rel=?2 AND f.mtime=?3 AND f.size=?4 AND a.rel_dir=?5
             AND NOT EXISTS(SELECT 1 FROM model_info m WHERE m.asset_id=?1 AND m.file=?2)
             AND NOT EXISTS(SELECT 1 FROM motion_info v WHERE v.asset_id=?1 AND v.file=?2))",
            )?
            .query_row(params![file.id, file.rel, file.mtime, file.size, file.dir], |r| {
                r.get(0)
            })?;
        if current {
            cache::save(&tx, file, &parsed)?;
        }
    }
    Ok(tx.commit()?)
}

// Recheck unique texture targets when the inventory changes. A texture can be restored or
// removed without changing the model binary. No model is decoded for that operation.
fn refresh_textures(
    db: &mut Connection,
    root: &Path,
    rev: i64,
    memo: &mut HashMap<String, bool>,
    mut keep: impl FnMut() -> bool,
) -> Result<bool, IndexError> {
    let targets: Vec<(String, bool, bool)> = db
        .prepare(
            "SELECT resolved_path,min(present),max(present) FROM model_textures
         WHERE resolved_path IS NOT NULL GROUP BY resolved_path",
        )?
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .collect::<rusqlite::Result<_>>()?;
    for chunk in targets.chunks(256) {
        if !keep() {
            return Ok(false);
        }
        let changed: Vec<_> = chunk
            .iter()
            .filter_map(|(rel, min, max)| {
                let actual = texture_present(root, rel, memo);
                (*min != actual || *max != actual).then_some((rel, actual))
            })
            .collect();
        if changed.is_empty() {
            continue;
        }
        let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
        assert_revision(&tx, root, rev)?;
        {
            let mut update =
                tx.prepare_cached("UPDATE model_textures SET present=? WHERE resolved_path=?")?;
            for (rel, present) in changed {
                update.execute(params![present, rel])?;
            }
        }
        tx.commit()?;
    }
    let tx = db.transaction_with_behavior(TransactionBehavior::Immediate)?;
    assert_revision(&tx, root, rev)?;
    tx.execute("UPDATE parse_state SET texture_revision=?", [rev])?;
    tx.commit()?;
    Ok(true)
}

impl Index {
    /// Decode the currently indexed PMX/PMD/VMD payloads, including ancillary files. All input
    /// reads happen outside write transactions. Returning false from progress cancels between
    /// batches; the next call resumes from committed files, including cached format errors.
    pub fn parse_files(
        &mut self,
        root: &Path,
        mut progress: impl FnMut(ParseProgress) -> bool,
    ) -> Result<ParseProgress, IndexError> {
        let tx = self.conn.transaction()?;
        let root = active_root(&tx, root)?;
        let (rev, verified): (i64, i64) = tx.query_row(
            "SELECT inventory_revision,texture_revision FROM parse_state",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        let mut state = ParseProgress::default();
        let mut queue = Vec::new();
        {
            let mut files = tx.prepare(
                "SELECT a.id,a.rel_dir,f.rel,f.mtime,f.size,lower(substr(f.rel,-3)),
                 EXISTS(SELECT 1 FROM model_info m WHERE m.asset_id=f.asset_id AND m.file=f.rel)
                 OR EXISTS(SELECT 1 FROM motion_info v WHERE v.asset_id=f.asset_id AND v.file=f.rel)
                 FROM files f JOIN assets a ON f.asset_id=a.id WHERE lower(substr(f.rel,-4)) IN ('.pmx','.pmd','.vmd')
                 ORDER BY a.rel_dir,f.rel",
            )?;
            let mut rows = files.query([])?;
            while let Some(r) = rows.next()? {
                state.total += 1;
                if r.get(6)? {
                    state.cached += 1;
                    continue;
                }
                queue.push(Candidate {
                    id: r.get(0)?,
                    dir: r.get(1)?,
                    rel: r.get(2)?,
                    mtime: r.get(3)?,
                    size: r.get(4)?,
                    format: r.get(5)?,
                });
            }
        }
        tx.commit()?;
        if !progress(state) {
            return Ok(state);
        }
        let mut textures = HashMap::new();
        if rev != verified
            && !refresh_textures(&mut self.conn, &root, rev, &mut textures, || progress(state))?
        {
            return Ok(state);
        }
        for chunk in queue.chunks(16) {
            let mut batch = Vec::with_capacity(chunk.len());
            for file in chunk {
                let result = file.read(&root, &mut textures);
                state.failed += usize::from(result.is_err());
                batch.push((file, result));
            }
            flush(&mut self.conn, &root, rev, &mut batch)?;
            state.processed += chunk.len();
            if !progress(state) {
                break;
            }
        }
        Ok(state)
    }
}
