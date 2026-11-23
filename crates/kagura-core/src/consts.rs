/// 每份资产根目录里的记录文件名。
pub const MANIFEST: &str = "_asset.manifest.json";
/// 同名但不是我们的记录时，改写到这个文件。
pub const MANIFEST_ALT: &str = "_asset.manifest.alt.json";
/// manifest 里固定的格式标识，永远不改；版本看 `schema` 字段。
pub const FORMAT: &str = "kagura-asset-manifest";
/// 歌曲目录下的保留子目录（比较时不区分大小写）。
pub const SONG_RESERVED: [&str; 4] = ["_audio", "_docs", "_lyrics", "_archive_attachments"];
/// 名称、版本、后缀的 UTF-16 长度上限。
pub const NAME_LIMIT: usize = 40;
pub const EDITION_LIMIT: usize = 24;
pub const SUFFIX_LIMIT: usize = 24;
/// 含根目录的完整路径上限（UTF-16 单位）。
pub const PATH_LIMIT: usize = 240;
