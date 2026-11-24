use std::{
    fs::{self, File, Metadata, OpenOptions},
    io,
    path::{Component, Path, PathBuf},
};

use super::IndexError;
use crate::scan::is_reparse;

pub(super) fn open_input(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // FILE_FLAG_OPEN_REPARSE_POINT lets the caller check the actual opened handle.
        options.custom_flags(0x0020_0000);
    }
    options.open(path)
}

/// Resolve an indexed directory or one of its files, rejecting reparse points at every level.
pub(super) fn checked_entry(
    root: &Path,
    dir: &str,
    file: Option<&str>,
) -> Result<(PathBuf, Metadata), IndexError> {
    if (dir != "Library" && !dir.starts_with("Library/")) || file == Some("") {
        return Err(IndexError::Invalid(format!("无效的索引路径：{dir} {file:?}")));
    }
    let mut path = root.to_owned();
    let mut metadata = fs::symlink_metadata(&path)?;
    for component in Path::new(dir)
        .components()
        .chain(file.into_iter().flat_map(|rel| Path::new(rel).components()))
    {
        if is_reparse(&metadata) || !metadata.is_dir() {
            return Err(IndexError::Invalid(format!("索引目录已改变：{}", path.display())));
        }
        let Component::Normal(name) = component else {
            return Err(IndexError::Invalid(format!("无效的索引路径：{dir} {file:?}")));
        };
        path.push(name);
        metadata = fs::symlink_metadata(&path)?;
    }
    if is_reparse(&metadata)
        || (file.is_some() && !metadata.is_file())
        || (file.is_none() && !metadata.is_dir())
    {
        return Err(IndexError::Invalid(format!(
            "索引路径类型已改变：{}",
            path.display()
        )));
    }
    Ok((path, metadata))
}

/// Explorer and WebView2 accept standard drive-letter and UNC paths.
pub(super) fn display_path(path: &Path) -> String {
    dunce::simplified(path).to_string_lossy().into_owned()
}
