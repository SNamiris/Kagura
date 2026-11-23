//! 只读扫描 Library。报告中的路径均相对传入根目录，并使用正斜杠。

use std::{fs, path::Path};

use serde_json::Value;

use crate::{
    consts::MANIFEST,
    layout::{self, LayoutError, SongMap},
    manifest::{self, DocumentFormat, MetadataRole, NewlineStyle},
    schema::Schemas,
};

#[derive(Debug, Clone)]
pub struct Record {
    pub dir: String,
    pub value: Value,
    /// 原始 manifest 的换行和 BOM，修改记录后写回时使用。
    pub format: DocumentFormat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkipReason {
    UnexpectedDepth,
    ForeignManifest,
    RuntimeConfig,
    SourceJson,
    Reserved,
    Reparse,
}

#[derive(Debug)]
pub struct Skipped {
    pub path: String,
    pub reason: SkipReason,
}

#[derive(Debug)]
pub struct ScanError {
    pub path: String,
    pub message: String,
}

#[derive(Debug)]
pub struct Directory {
    pub path: String,
    /// 按实际目录项判空，包括保留目录和重解析点。
    pub empty: bool,
}

#[derive(Debug, Default)]
pub struct ScanReport {
    pub records: Vec<Record>,
    /// 找到记录文件但跳过识别/纳入。
    pub skipped: Vec<Skipped>,
    /// Python 参考扫描在枚举阶段剪枝的目录和重解析点。
    pub pruned: Vec<Skipped>,
    /// 扫描已读取的 manifest 中，混用 CRLF 与 LF 的相对路径。
    pub mixed_newlines: Vec<String>,
    pub errors: Vec<ScanError>,
    pub directories: Vec<Directory>,
    pub files: Vec<String>,
}

impl ScanReport {
    fn error(&mut self, path: &str, message: impl ToString) {
        self.errors.push(ScanError {
            path: path.into(),
            message: message.to_string(),
        });
    }
}

#[cfg(windows)]
pub(crate) fn is_reparse(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes() & 0x400 != 0
}

#[cfg(not(windows))]
pub(crate) fn is_reparse(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

fn read_record(root: &Path, dir: &str, path: &str, schemas: &Schemas, report: &mut ScanReport) {
    let Some(expected) = layout::expected_type(dir) else {
        report.skipped.push(Skipped {
            path: path.into(),
            reason: SkipReason::UnexpectedDepth,
        });
        return;
    };
    let bytes = match fs::read(root.join(path)) {
        Ok(bytes) => bytes,
        Err(error) => {
            report.error(path, error);
            return;
        }
    };
    let format = DocumentFormat::detect(&bytes);
    if matches!(format.newline, NewlineStyle::Mixed { .. }) {
        report.mixed_newlines.push(path.into());
    }
    let value = match manifest::parse(&bytes) {
        Ok(value) => value,
        Err(error) => {
            report.error(path, error);
            return;
        }
    };
    let reason = match manifest::metadata_role(&value) {
        MetadataRole::Asset => None,
        MetadataRole::RuntimeConfig => Some(SkipReason::RuntimeConfig),
        MetadataRole::ForeignManifest => Some(SkipReason::ForeignManifest),
        MetadataRole::SourceJson => Some(SkipReason::SourceJson),
    };
    if let Some(reason) = reason {
        report.skipped.push(Skipped {
            path: path.into(),
            reason,
        });
        return;
    }
    match schemas.validate_asset(&value, 8) {
        Err(error) => report.error(path, error),
        Ok(issues) if !issues.is_empty() => {
            let messages: Vec<_> = issues
                .iter()
                .map(|issue| format!("{} {}", issue.path, issue.message))
                .collect();
            report.error(path, messages.join(" | "));
        }
        Ok(_) if value["type"] != expected => {
            report.error(
                path,
                format!("目录应包含 {expected}，记录类型为 {}", value["type"]),
            );
        }
        Ok(_) => report.records.push(Record {
            dir: dir.into(),
            value,
            format,
        }),
    }
}

/// 跳过重解析点和保留目录子树。
pub fn scan_library(root: &Path) -> ScanReport {
    scan(root, false)
}

/// 同时盘点保留目录里的文件和目录，供一致性检查使用，仍跳过其中的 manifest。
pub fn scan_library_for_check(root: &Path) -> ScanReport {
    scan(root, true)
}

fn scan(root: &Path, inventory_reserved: bool) -> ScanReport {
    let mut report = ScanReport::default();
    let schemas = match Schemas::load() {
        Ok(schemas) => schemas,
        Err(error) => {
            report.error("Library", error);
            return report;
        }
    };
    match fs::symlink_metadata(root.join("Library")) {
        Ok(metadata) if is_reparse(&metadata) => {
            report.pruned.push(Skipped {
                path: "Library".into(),
                reason: SkipReason::Reparse,
            });
            return report;
        }
        Err(error) => {
            report.error("Library", error);
            return report;
        }
        Ok(_) => {}
    }

    let mut pending = vec![(String::from("Library"), false)];
    while let Some((dir, reserved)) = pending.pop() {
        let entries = match fs::read_dir(root.join(&dir)) {
            Ok(entries) => entries,
            Err(error) => {
                report.error(&dir, error);
                continue;
            }
        };
        let mut empty = true;
        for entry in entries {
            empty = false;
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    report.error(&dir, error);
                    continue;
                }
            };
            let name = match entry.file_name().into_string() {
                Ok(name) => name,
                Err(name) => {
                    report.error(&dir, format!("文件名不是有效的 Unicode：{name:?}"));
                    continue;
                }
            };
            let path = format!("{dir}/{name}");
            // DirEntry::metadata 与 symlink_metadata 一样不会跟随符号链接。
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(error) => {
                    report.error(&path, error);
                    continue;
                }
            };
            if is_reparse(&metadata) {
                if !reserved && name == MANIFEST {
                    report.error(&path, "元数据不能是重解析点");
                }
                report.pruned.push(Skipped {
                    path,
                    reason: SkipReason::Reparse,
                });
            } else if metadata.is_dir() {
                let entering_reserved = !reserved && layout::is_song_reserved(&path);
                if entering_reserved {
                    report.pruned.push(Skipped {
                        path: path.clone(),
                        reason: SkipReason::Reserved,
                    });
                }
                if !entering_reserved || inventory_reserved {
                    pending.push((path, reserved || entering_reserved));
                }
            } else if metadata.is_file() {
                if !reserved && name == MANIFEST {
                    read_record(root, &dir, &path, &schemas, &mut report);
                }
                report.files.push(path);
            }
        }
        report.directories.push(Directory { path: dir, empty });
    }
    report.records.sort_by(|a, b| a.dir.cmp(&b.dir));
    report.directories.sort_by(|a, b| a.path.cmp(&b.path));
    report.files.sort();
    report.errors.sort_by(|a, b| a.path.cmp(&b.path));
    report.pruned.sort_by(|a, b| a.path.cmp(&b.path));
    report.skipped.sort_by(|a, b| a.path.cmp(&b.path));
    report.mixed_newlines.sort();
    report
}

pub fn song_map(records: &[Record]) -> Result<SongMap, LayoutError> {
    let mut songs = SongMap::new();
    for record in records.iter().filter(|r| r.value["type"] == "song") {
        let id = record.value["id"].as_str().ok_or(LayoutError::Field("id"))?;
        let expected = layout::song_dir(&record.value["key"])?;
        if record.dir != expected {
            return Err(LayoutError::SongDirectory {
                id: id.into(),
                actual: record.dir.clone(),
                expected,
            });
        }
        if songs.insert(id.to_owned(), record.dir.clone()).is_some() {
            return Err(LayoutError::DuplicateSong(id.into()));
        }
    }
    Ok(songs)
}
