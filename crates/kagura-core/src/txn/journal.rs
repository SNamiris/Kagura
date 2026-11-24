use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

use super::{EditEntry, TxnError};
use crate::scan::is_reparse;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Record {
    #[serde(flatten)]
    pub entry: EditEntry,
    pub root: String,
    pub undo_of: Option<String>,
    pub old_bytes: String,
}

#[derive(Deserialize)]
struct Header {
    #[serde(flatten)]
    entry: EditEntry,
    root: String,
    undo_of: Option<String>,
}

pub(super) struct Journal {
    dir: PathBuf,
    /// The OS releases this interprocess lock even if the process dies mid-edit.
    _guard: File,
    pending: PathBuf,
}

fn reject_reparse(path: &Path) -> Result<(), TxnError> {
    match fs::symlink_metadata(path) {
        Ok(meta) if is_reparse(&meta) => Err(TxnError::Invalid(format!(
            "编辑日志不能使用重解析点：{}",
            path.display()
        ))),
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub(super) fn sync_dir(dir: &Path) -> io::Result<()> {
    #[cfg(unix)]
    File::open(dir)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = dir;
    Ok(())
}

impl Journal {
    pub fn open(dir: &Path, root: &Path) -> Result<Self, TxnError> {
        reject_reparse(dir)?;
        let dir = crate::index::resolve_output(dir)?;
        if dunce::simplified(&dir).starts_with(root) {
            return Err(TxnError::Invalid("编辑日志必须位于库根之外".into()));
        }
        fs::create_dir_all(&dir)?;
        let lock = dir.join(".lock");
        reject_reparse(&lock)?;
        let guard = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(lock)?;
        guard.lock()?;
        let root_hash = blake3::hash(root.to_string_lossy().as_bytes());
        let pending = dir.join(format!(".pending-{}.json", root_hash.to_hex()));
        reject_reparse(&pending)?;
        Ok(Self {
            dir,
            _guard: guard,
            pending,
        })
    }

    pub fn pending(&self) -> Result<Option<Record>, TxnError> {
        match File::open(&self.pending) {
            Ok(file) => Ok(Some(serde_json::from_reader(BufReader::new(file))?)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn prepare(&self, record: &Record) -> Result<(), TxnError> {
        let mut temp = NamedTempFile::new_in(&self.dir)?;
        serde_json::to_writer(&mut temp, record)?;
        temp.as_file().sync_all()?;
        temp.persist_noclobber(&self.pending).map_err(|e| e.error)?;
        sync_dir(&self.dir)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), TxnError> {
        fs::remove_file(&self.pending)?;
        sync_dir(&self.dir)?;
        Ok(())
    }

    pub fn archive_conflict(&self) -> Result<PathBuf, TxnError> {
        let path = self.dir.join(format!(".conflict-{}.json", uuid::Uuid::new_v4()));
        fs::rename(&self.pending, &path)?;
        sync_dir(&self.dir)?;
        Ok(path)
    }

    /// Copy-on-write publication keeps every monthly JSONL complete even across interrupted writes.
    pub fn publish(&self, record: &Record) -> Result<(), TxnError> {
        let month = record
            .entry
            .timestamp
            .get(..7)
            .filter(|m| valid_month(m))
            .ok_or_else(|| TxnError::Invalid("日志时间无效".into()))?;
        let path = self.dir.join(format!("{month}.jsonl"));
        reject_reparse(&path)?;
        let mut temp = NamedTempFile::new_in(&self.dir)?;
        match File::open(&path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut line = String::new();
                loop {
                    line.clear();
                    if reader.read_line(&mut line)? == 0 {
                        break;
                    }
                    let header: Header = serde_json::from_str(&line)?;
                    if header.entry.entry_id == record.entry.entry_id {
                        return Ok(());
                    }
                    temp.write_all(line.as_bytes())?;
                    if !line.ends_with('\n') {
                        temp.write_all(b"\n")?;
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
        serde_json::to_writer(&mut temp, record)?;
        temp.write_all(b"\n")?;
        temp.as_file().sync_all()?;
        temp.persist(&path).map_err(|e| e.error)?;
        sync_dir(&self.dir)?;
        Ok(())
    }

    fn months(&self) -> Result<Vec<PathBuf>, TxnError> {
        let mut paths = Vec::new();
        for item in fs::read_dir(&self.dir)? {
            let path = item?.path();
            if path.extension().is_some_and(|ext| ext == "jsonl")
                && path.file_stem().and_then(|s| s.to_str()).is_some_and(valid_month)
            {
                reject_reparse(&path)?;
                paths.push(path);
            }
        }
        paths.sort_unstable_by(|a, b| b.cmp(a));
        Ok(paths)
    }

    pub fn recent(&self, root: &str, limit: usize) -> Result<Vec<EditEntry>, TxnError> {
        let mut result = Vec::new();
        let mut undone = HashSet::new();
        for path in self.months()? {
            if result.len() >= limit {
                break;
            }
            let mut month = Vec::new();
            for line in BufReader::new(File::open(path)?).lines() {
                let row: Header = serde_json::from_str(&line?)?;
                if row.root == root {
                    month.push(row);
                }
            }
            for row in month.into_iter().rev() {
                if let Some(id) = row.undo_of {
                    undone.insert(id);
                } else if result.len() < limit {
                    let mut entry = row.entry;
                    entry.undone = undone.contains(&entry.entry_id);
                    result.push(entry);
                }
            }
        }
        Ok(result)
    }

    pub fn undo_target(&self, root: &str, entry_id: &str) -> Result<Record, TxnError> {
        let mut found = None;
        for path in self.months()? {
            for line in BufReader::new(File::open(path)?).lines() {
                let line = line?;
                let header: Header = serde_json::from_str(&line)?;
                if header.root != root {
                    continue;
                }
                if header.undo_of.as_deref() == Some(entry_id) {
                    return Err(TxnError::Conflict("这次修改已撤回".into()));
                }
                if header.entry.entry_id == entry_id && header.undo_of.is_none() {
                    found = Some(serde_json::from_str(&line)?);
                }
            }
            if found.is_some() {
                break;
            }
        }
        found.ok_or_else(|| TxnError::Invalid("当前库中没有这条编辑记录".into()))
    }
}

fn valid_month(s: &str) -> bool {
    s.len() == 7
        && s.bytes()
            .enumerate()
            .all(|(i, b)| if i == 4 { b == b'-' } else { b.is_ascii_digit() })
}
