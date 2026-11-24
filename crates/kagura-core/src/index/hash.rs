//! 按需计算资产文件的 BLAKE3。缓存有效性与增量扫描共用 size 和纳秒 mtime。
use std::fs::Metadata;
use std::path::{Path, PathBuf};

use rusqlite::{params, OptionalExtension};

use super::{paths::open_input, Index, IndexError};
use crate::scan::{is_reparse, mtime_ns};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HashReport {
    pub hashed_files: usize,
    pub cached_files: usize,
    pub bytes_hashed: u64,
}

/// 每读完或命中缓存一个文件时通知调用者。rel 相对资产目录。
#[derive(Debug)]
pub struct HashProgress<'a> {
    pub asset_id: &'a str,
    pub rel: &'a str,
    pub completed_files: usize,
    pub total_files: usize,
}

struct IndexedFile {
    rel: String,
    size: i64,
    mtime: i64,
    cached: bool,
}

struct DigestUpdate {
    old: IndexedFile,
    size: i64,
    mtime: i64,
    digest: String,
}

fn invalid(msg: impl Into<String>) -> IndexError {
    IndexError::Invalid(msg.into())
}

fn checked_path(root: &Path, dir: &str, rel: &str) -> Result<(PathBuf, Metadata), IndexError> {
    super::paths::checked_entry(root, dir, Some(rel))
}

fn stamp(metadata: &Metadata) -> Result<(i64, i64), IndexError> {
    let size = i64::try_from(metadata.len()).map_err(|_| invalid("文件大小超出 SQLite 整数范围"))?;
    Ok((size, mtime_ns(metadata)?))
}

impl Index {
    fn hash_root(&self) -> Result<String, IndexError> {
        self.conn
            .query_row("SELECT root FROM meta WHERE id = 1", [], |row| row.get(0))
            .optional()?
            .ok_or_else(|| invalid("索引尚未构建"))
    }

    /// 对单个资产计算缺少或过期的文件摘要；只在写回该资产时短暂开启事务。
    pub fn hash_asset(&mut self, id: &str) -> Result<HashReport, IndexError> {
        self.hash_files(&self.hash_root()?, id, |_| {})
    }

    /// 按资产和相对路径顺序计算。已完成资产的缓存可在中断后复用。
    pub fn hash_all(
        &mut self,
        mut progress: impl FnMut(&HashProgress<'_>),
    ) -> Result<HashReport, IndexError> {
        let root = self.hash_root()?;
        let count: i64 = self
            .conn
            .query_row("SELECT count(*) FROM files", [], |row| row.get(0))?;
        let total_files = usize::try_from(count).map_err(|_| invalid("文件计数超出范围"))?;
        let assets = self
            .conn
            .prepare("SELECT id FROM assets ORDER BY id")?
            .query_map([], |r| r.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        let mut total = HashReport::default();
        let mut completed_files = 0;
        for id in assets {
            let asset = self.hash_files(&root, &id, |rel| {
                completed_files += 1;
                progress(&HashProgress {
                    asset_id: &id,
                    rel,
                    completed_files,
                    total_files,
                });
            })?;
            total.hashed_files += asset.hashed_files;
            total.cached_files += asset.cached_files;
            total.bytes_hashed += asset.bytes_hashed;
        }
        Ok(total)
    }

    fn hash_files(
        &mut self,
        root: &str,
        id: &str,
        mut progress: impl FnMut(&str),
    ) -> Result<HashReport, IndexError> {
        let dir: String = self
            .conn
            .query_row("SELECT rel_dir FROM assets WHERE id = ?1", [id], |r| r.get(0))
            .optional()?
            .ok_or_else(|| invalid(format!("索引中没有资产 {id}")))?;
        let files = self
            .conn
            .prepare(
                "SELECT rel, size, mtime, blake3 IS NOT NULL FROM files WHERE asset_id = ?1 ORDER BY rel",
            )?
            .query_map([id], |r| {
                Ok(IndexedFile {
                    rel: r.get(0)?,
                    size: r.get(1)?,
                    mtime: r.get(2)?,
                    cached: r.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut updates = Vec::new();
        let mut report = HashReport::default();
        for indexed in files {
            let (path, meta) = checked_path(Path::new(root), &dir, &indexed.rel)?;
            let before = stamp(&meta)?;
            if indexed.cached && before == (indexed.size, indexed.mtime) {
                report.cached_files += 1;
                progress(&indexed.rel);
                continue;
            }
            let mut file = open_input(&path)?;
            let opened = file.metadata()?;
            if is_reparse(&opened) || !opened.is_file() || stamp(&opened)? != before {
                return Err(invalid(format!("开始哈希前文件已改变：{}", path.display())));
            }
            let mut hasher = blake3::Hasher::new();
            hasher.update_reader(&mut file)?;
            let bytes = hasher.count();
            let (_, current) = checked_path(Path::new(root), &dir, &indexed.rel)?;
            if bytes != meta.len() || stamp(&file.metadata()?)? != before || stamp(&current)? != before {
                return Err(invalid(format!("计算哈希时文件已改变：{}", path.display())));
            }
            report.hashed_files += 1;
            report.bytes_hashed += bytes;
            progress(&indexed.rel);
            updates.push(DigestUpdate {
                old: indexed,
                size: before.0,
                mtime: before.1,
                digest: hasher.finalize().to_hex().to_string(),
            });
        }
        if !updates.is_empty() {
            // Earlier files can change while the remaining files of this asset are read.
            for update in &updates {
                let (path, current) = checked_path(Path::new(root), &dir, &update.old.rel)?;
                if stamp(&current)? != (update.size, update.mtime) {
                    return Err(invalid(format!("写入哈希缓存前文件已改变：{}", path.display())));
                }
            }
            let tx = self.conn.transaction()?;
            {
                let mut write = tx.prepare_cached(
                    "UPDATE files SET size=?1, mtime=?2, blake3=?3
                     WHERE asset_id=?4 AND rel=?5 AND size=?6 AND mtime=?7
                     AND EXISTS (SELECT 1 FROM assets WHERE id=?4 AND rel_dir=?8)
                     AND EXISTS (SELECT 1 FROM meta WHERE id=1 AND root=?9)",
                )?;
                for update in updates {
                    let changed = write.execute(params![
                        update.size,
                        update.mtime,
                        update.digest,
                        id,
                        update.old.rel,
                        update.old.size,
                        update.old.mtime,
                        dir,
                        root
                    ])?;
                    if changed != 1 {
                        return Err(invalid(format!("索引在哈希期间已改变：{dir}/{}", update.old.rel)));
                    }
                }
            }
            tx.commit()?;
        }
        Ok(report)
    }
}
