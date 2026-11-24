//! Optimistic manifest edits, a write-ahead recovery record and reversible monthly journals.
mod journal;
mod patch;

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use chrono::{FixedOffset, SecondsFormat, Utc};
use journal::{Journal, Record};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tempfile::NamedTempFile;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    index::{Index, IndexError},
    layout::{self, SongMap},
    manifest,
    scan::is_reparse,
    schema::Schemas,
};
pub use patch::{EditPatch, SourcePatch};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct EditEntry {
    pub entry_id: String,
    pub id: String,
    pub relative_path: String,
    pub timestamp: String,
    pub old_hash: String,
    pub new_hash: String,
    pub changed_fields: Vec<String>,
    pub undone: bool,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct EditResult {
    pub entry_id: String,
    pub id: String,
    pub new_hash: String,
}

#[derive(Debug, Error)]
pub enum TxnError {
    #[error("文件已改变，请重新加载。{0}")]
    Conflict(String),
    #[error("{0}")]
    Invalid(String),
    #[error("没有需要保存的修改")]
    NoChanges,
    #[error("Schema 校验失败：{0}")]
    Schema(String),
    #[error("文件已保存，索引或日志尚待恢复。请重新打开库：{0}")]
    Recovery(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Index(#[from] IndexError),
}

impl TxnError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Conflict(_) => "conflict",
            Self::Schema(_) => "schema_invalid",
            Self::Invalid(_) | Self::Json(_) => "invalid_edit",
            Self::NoChanges => "no_changes",
            Self::Recovery(_) => "recovery_required",
            Self::Io(_) => "edit_io",
            Self::Index(_) => "index_error",
        }
    }
}

fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn now() -> chrono::DateTime<FixedOffset> {
    // Contemporary Asia/Shanghai dates use UTC+08:00, independent of the host's time zone.
    Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).expect("UTC+08"))
}

fn relative(root: &Path, path: &Path) -> Result<String, TxnError> {
    dunce::canonicalize(path)?
        .strip_prefix(root)
        .ok()
        .and_then(Path::to_str)
        .map(|s| s.replace('\\', "/"))
        .ok_or_else(|| TxnError::Invalid("资产路径必须位于库根内且为有效 Unicode".into()))
}

fn read_current(path: &Path) -> Result<(File, Vec<u8>), TxnError> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // Other readers and atomic renames are allowed. Ordinary in-place writers are excluded.
        options.share_mode(1 | 4).custom_flags(0x0020_0000); // FILE_FLAG_OPEN_REPARSE_POINT
    }
    let mut file = options.open(path).map_err(|e| match e.raw_os_error() {
        #[cfg(windows)]
        Some(32 | 33) => TxnError::Conflict("其他程序正在写入该文件".into()),
        _ if e.kind() == io::ErrorKind::NotFound => TxnError::Conflict("资产文件已移除".into()),
        _ => TxnError::Io(e),
    })?;
    let meta = file.metadata()?;
    if !meta.is_file() || is_reparse(&meta) {
        return Err(TxnError::Invalid("manifest 必须是普通文件".into()));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok((file, bytes))
}

impl Index {
    /// Override the shared per-user journal (e.g. for an isolated fixture).
    pub fn set_edits_dir(&mut self, path: impl Into<PathBuf>) {
        self.edits_dir = Some(path.into());
    }

    fn edit_journal(&self, root: &Path) -> Result<Journal, TxnError> {
        match &self.edits_dir {
            Some(dir) => Journal::open(dir, root),
            None => Journal::open(&Self::default_path()?.with_file_name("edits"), root),
        }
    }

    fn edit_path(&self, root: &Path, id: &str) -> Result<PathBuf, TxnError> {
        match self.manifest_path(root, id) {
            Ok(Some(path)) => Ok(path),
            Ok(None) => Err(TxnError::Conflict("资产文件已移除或改名".into())),
            Err(IndexError::Io(e)) if e.kind() == io::ErrorKind::NotFound => {
                Err(TxnError::Conflict("资产文件已移除或改名".into()))
            }
            Err(e) => Err(e.into()),
        }
    }

    fn validate_edit(
        &self,
        root: &Path,
        id: &str,
        path: &Path,
        bytes: &[u8],
        schemas: &Schemas,
    ) -> Result<Value, TxnError> {
        let value = manifest::parse(bytes)?;
        if value["id"].as_str() != Some(id) {
            return Err(TxnError::Conflict("目录中的资产 ID 已改变".into()));
        }
        if manifest::metadata_role(&value) != manifest::MetadataRole::Asset {
            return Err(TxnError::Invalid("文件不属于 Kagura Asset".into()));
        }
        let issues = schemas
            .validate_asset(&value, 4)
            .map_err(|e| TxnError::Schema(e.to_string()))?;
        if !issues.is_empty() {
            return Err(TxnError::Schema(
                issues
                    .iter()
                    .map(|i| format!("{} {}", i.path, i.message))
                    .collect::<Vec<_>>()
                    .join("；"),
            ));
        }
        let mut songs = SongMap::new();
        if let Some(song) = value["song"].as_str() {
            let dir = self
                .asset_directory(root, song)?
                .ok_or_else(|| TxnError::Invalid("所属歌曲不存在".into()))?;
            songs.insert(song.to_owned(), relative(root, &dir)?);
        }
        let expected = layout::destination(&value, &songs).map_err(|e| TxnError::Invalid(e.to_string()))?;
        if relative(
            root,
            path.parent()
                .ok_or_else(|| TxnError::Invalid("资产目录无效".into()))?,
        )? != expected
        {
            return Err(TxnError::Invalid("资产 key 与目录不一致，请检查资产库".into()));
        }
        Ok(value)
    }

    /// Reconcile bookkeeping after interruption. Recovery never writes to a library manifest.
    pub fn recover_edits(&mut self, root: &Path) -> Result<(), TxnError> {
        let root = dunce::canonicalize(root)?;
        let journal = self.edit_journal(&root)?;
        self.recover_locked(&root, &journal)
    }

    fn recover_locked(&mut self, root: &Path, journal: &Journal) -> Result<(), TxnError> {
        let Some(record) = journal.pending()? else {
            return Ok(());
        };
        let archive = || -> Result<TxnError, TxnError> {
            let path = journal.archive_conflict()?;
            Ok(TxnError::Conflict(format!(
                "待恢复的编辑 {} 与磁盘内容或路径不一致。原始备份保存在 {}，可重新打开库继续操作",
                record.entry.entry_id,
                path.display()
            )))
        };
        let path = match self.edit_path(root, &record.entry.id) {
            Ok(path) => path,
            Err(TxnError::Conflict(_) | TxnError::Index(IndexError::Invalid(_))) => return Err(archive()?),
            Err(e) => return Err(e),
        };
        if record.root != root.to_string_lossy()
            || relative(root, &path)? != record.entry.relative_path
            || digest(record.old_bytes.as_bytes()) != record.entry.old_hash
        {
            return Err(archive()?);
        }
        let (_file, bytes) = read_current(&path)?;
        let current = digest(&bytes);
        if current == record.entry.new_hash {
            self.refresh_manifest(root, &record.entry.id)?;
            journal.publish(&record)?;
        } else if current != record.entry.old_hash {
            return Err(archive()?);
        }
        journal.clear()
    }

    pub fn list_edits(&mut self, root: &Path, limit: usize) -> Result<Vec<EditEntry>, TxnError> {
        let root = dunce::canonicalize(root)?;
        let journal = self.edit_journal(&root)?;
        self.recover_locked(&root, &journal)?;
        journal.recent(&root.to_string_lossy(), limit)
    }

    pub fn edit_manifest(
        &mut self,
        root: &Path,
        id: &str,
        patch: EditPatch,
        base_hash: &str,
    ) -> Result<EditResult, TxnError> {
        let root = dunce::canonicalize(root)?;
        let journal = self.edit_journal(&root)?;
        self.recover_locked(&root, &journal)?;
        let path = self.edit_path(&root, id)?;
        let (_, bytes) = read_current(&path)?;
        let old_hash = digest(&bytes);
        if old_hash != base_hash {
            return Err(TxnError::Conflict("基础 BLAKE3 不匹配".into()));
        }
        let schemas = Schemas::load().map_err(|e| TxnError::Schema(e.to_string()))?;
        let value = self.validate_edit(&root, id, &path, &bytes, &schemas)?;
        let original = String::from_utf8(bytes).map_err(|e| TxnError::Invalid(e.to_string()))?;
        let timestamp = now();
        let (new, changed_fields) =
            patch::apply(&original, &value, patch, &timestamp.date_naive().to_string())?;
        self.validate_edit(&root, id, &path, new.as_bytes(), &schemas)?;
        let record = Record {
            entry: EditEntry {
                entry_id: Uuid::new_v4().to_string(),
                id: id.into(),
                relative_path: relative(&root, &path)?,
                timestamp: timestamp.to_rfc3339_opts(SecondsFormat::Millis, false),
                old_hash,
                new_hash: digest(new.as_bytes()),
                changed_fields,
                undone: false,
            },
            root: root.to_string_lossy().into_owned(),
            old_bytes: original,
            undo_of: None,
        };
        self.commit_edit(&root, &journal, record, new.as_bytes())
    }

    pub fn undo_edit(&mut self, root: &Path, entry_id: &str) -> Result<EditResult, TxnError> {
        let root = dunce::canonicalize(root)?;
        let journal = self.edit_journal(&root)?;
        self.recover_locked(&root, &journal)?;
        let prior = journal.undo_target(&root.to_string_lossy(), entry_id)?;
        let path = self.edit_path(&root, &prior.entry.id)?;
        if relative(&root, &path)? != prior.entry.relative_path {
            return Err(TxnError::Conflict("资产路径已改变".into()));
        }
        let (_, bytes) = read_current(&path)?;
        if digest(&bytes) != prior.entry.new_hash {
            return Err(TxnError::Conflict("当前内容已不再是这次编辑的结果".into()));
        }
        let restore = prior.old_bytes;
        if digest(restore.as_bytes()) != prior.entry.old_hash {
            return Err(TxnError::Invalid("日志原文与 BLAKE3 不匹配".into()));
        }
        let schemas = Schemas::load().map_err(|e| TxnError::Schema(e.to_string()))?;
        self.validate_edit(&root, &prior.entry.id, &path, restore.as_bytes(), &schemas)?;
        let record = Record {
            entry: EditEntry {
                entry_id: Uuid::new_v4().to_string(),
                timestamp: now().to_rfc3339_opts(SecondsFormat::Millis, false),
                old_hash: prior.entry.new_hash,
                new_hash: prior.entry.old_hash,
                ..prior.entry
            },
            root: prior.root,
            old_bytes: String::from_utf8(bytes).map_err(|e| TxnError::Invalid(e.to_string()))?,
            undo_of: Some(entry_id.to_owned()),
        };
        self.commit_edit(&root, &journal, record, restore.as_bytes())
    }

    fn commit_edit(
        &mut self,
        root: &Path,
        journal: &Journal,
        record: Record,
        new: &[u8],
    ) -> Result<EditResult, TxnError> {
        let entry = &record.entry;
        let path = self.edit_path(root, &entry.id)?;
        let parent = path
            .parent()
            .ok_or_else(|| TxnError::Invalid("资产路径无效".into()))?;
        let mut temp = NamedTempFile::new_in(parent)?;
        temp.write_all(new)?;
        temp.as_file()
            .set_permissions(fs::metadata(&path)?.permissions())?;
        temp.as_file().sync_all()?;
        journal.prepare(&record)?;
        let replace = || -> Result<(), TxnError> {
            let current = self.edit_path(root, &entry.id)?;
            if relative(root, &current)? != entry.relative_path {
                return Err(TxnError::Conflict("资产目录已改变".into()));
            }
            let (guard, bytes) = read_current(&current)?;
            if digest(&bytes) != entry.old_hash {
                return Err(TxnError::Conflict("写回前文件再次改变".into()));
            }
            // MoveFileExW requires the previous destination handle to be closed even with
            // FILE_SHARE_DELETE. The application journal lock remains held across the rename.
            drop(guard);
            temp.persist(&current).map_err(|e| {
                io::Error::new(
                    e.error.kind(),
                    format!("原子替换 {}：{}", current.display(), e.error),
                )
            })?;
            Ok(())
        };
        if let Err(error) = replace() {
            journal.clear()?;
            return Err(error);
        }
        let mut finish = || -> Result<(), TxnError> {
            journal::sync_dir(parent)?;
            self.refresh_manifest(root, &entry.id)?;
            journal.publish(&record)?;
            journal.clear()
        };
        finish().map_err(|e| TxnError::Recovery(e.to_string()))?;
        Ok(EditResult {
            entry_id: entry.entry_id.clone(),
            id: entry.id.clone(),
            new_hash: entry.new_hash.clone(),
        })
    }
}
