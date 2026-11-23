//! 由 manifest 的 key 和歌曲索引生成相对资产库根的 POSIX 目录。

use std::collections::BTreeMap;

use serde_json::Value;
use thiserror::Error;

use crate::consts::{EDITION_LIMIT, NAME_LIMIT, SONG_RESERVED, SUFFIX_LIMIT};
use crate::naming::{safe, NamingError};

/// 歌曲 ID → 相对歌曲目录。
pub type SongMap = BTreeMap<String, String>;

#[derive(Debug, Error)]
pub enum LayoutError {
    #[error(transparent)]
    Name(#[from] NamingError),
    #[error("字段缺失或类型错误：{0}")]
    Field(&'static str),
    #[error("不支持的资产类型或模型 kind：{0}")]
    UnknownType(String),
    #[error("动作引用的歌曲 ID 无法验证：{0}")]
    UnknownSong(String),
    #[error("歌曲 ID 重复：{0}")]
    DuplicateSong(String),
    #[error("歌曲 {id} 目录与 key 不一致：实际 {actual}，期望 {expected}")]
    SongDirectory {
        id: String,
        actual: String,
        expected: String,
    },
}

fn text<'a>(object: &'a Value, field: &'static str) -> Result<&'a str, LayoutError> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or(LayoutError::Field(field))
}

fn optional_name(object: &Value, field: &'static str, limit: usize) -> Result<Option<String>, LayoutError> {
    match object.get(field) {
        None => Ok(None),
        Some(Value::String(s)) if s.is_empty() => Ok(None),
        Some(Value::String(s)) => Ok(Some(safe(s, limit)?)),
        _ => Err(LayoutError::Field(field)),
    }
}

pub fn song_dir(key: &Value) -> Result<String, LayoutError> {
    let title = safe(text(key, "title")?, NAME_LIMIT)?;
    let composer = safe(text(key, "composer")?, NAME_LIMIT)?;
    let edition = optional_name(key, "edition", EDITION_LIMIT)?
        .map(|e| format!("({e})"))
        .unwrap_or_default();
    Ok(format!("Library/Motions/{title}{edition}_{composer}"))
}

/// 所有类型均返回目录；歌曲的 manifest 位于该目录下。
pub fn destination(value: &Value, songs: &SongMap) -> Result<String, LayoutError> {
    let key = value.get("key").ok_or(LayoutError::Field("key"))?;
    match text(value, "type")? {
        "model" | "stage" => {
            let author = safe(text(key, "author")?, NAME_LIMIT)?;
            let name = safe(text(key, "name")?, NAME_LIMIT)?;
            let base = if value["type"] == "stage" {
                "Library/Stages"
            } else {
                match text(value, "kind")? {
                    "character" => "Library/Models/Characters",
                    "prop" => "Library/Models/Props",
                    other => return Err(LayoutError::UnknownType(other.into())),
                }
            };
            Ok(format!("{base}/{author}/{name}"))
        }
        "song" => song_dir(key),
        "motion_part" => {
            let base = match value.get("song") {
                Some(Value::Null) => "Library/Motions/_General",
                Some(Value::String(id)) => songs
                    .get(id)
                    .ok_or_else(|| LayoutError::UnknownSong(id.clone()))?,
                _ => return Err(LayoutError::Field("song")),
            };
            let author = safe(text(key, "author")?, NAME_LIMIT)?;
            let part = text(key, "part")?;
            let suffix = optional_name(key, "suffix", SUFFIX_LIMIT)?
                .map(|s| format!("_{s}"))
                .unwrap_or_default();
            Ok(format!("{base}/{part}_{author}{suffix}"))
        }
        other => Err(LayoutError::UnknownType(other.into())),
    }
}

/// 是否位于 Library/Motions/<歌>/<保留名> 或其子树；输入为仓库相对 POSIX 路径。
pub fn is_song_reserved(dir: &str) -> bool {
    let Some(path) = dir.strip_prefix("Library/Motions/") else {
        return false;
    };
    let Some((_, descendants)) = path.split_once('/') else {
        return false;
    };
    let name = descendants.split('/').next().unwrap_or_default();
    SONG_RESERVED
        .iter()
        .any(|reserved| unicase::UniCase::unicode(name) == unicase::UniCase::unicode(*reserved))
}

/// 给定的目录是否在允许放置资产的层级上，以及这里应是哪种资产。
pub fn expected_type(dir: &str) -> Option<&'static str> {
    let mut parts = dir.split('/');
    if parts.next() != Some("Library") {
        return None;
    }
    let ty = match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some("Models"), Some("Characters" | "Props"), Some(_), Some(_)) => "model",
        (Some("Stages"), Some(_), Some(_), None) => "stage",
        (Some("Motions"), Some("_General"), None, None) => return None,
        (Some("Motions"), Some(_), None, None) => "song",
        (Some("Motions"), Some(_), Some(_), None) if !is_song_reserved(dir) => "motion_part",
        _ => return None,
    };
    parts.next().is_none().then_some(ty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reserved_at_song_level_only() {
        for name in ["_AUDIO", "_lyricſ", "_DocS/inner"] {
            assert!(is_song_reserved(&format!("Library/Motions/song/{name}")));
        }
        for dir in [
            "Library/Motions/_Audio",
            "Library/Motions/song/part/_docs",
            "Library/Stages/song/_audio",
        ] {
            assert!(!is_song_reserved(dir));
        }
    }

    #[test]
    fn optional_components_and_unknown_song() {
        assert_eq!(
            song_dir(&json!({"title":"a","composer":"b","edition":""})).unwrap(),
            "Library/Motions/a_b"
        );
        assert_eq!(
            song_dir(&json!({"title":"a","composer":"b"})).unwrap(),
            "Library/Motions/a_b"
        );
        let mut v = json!({"type":"motion_part","key":{"part":"face","author":"a","suffix":""},"song":null});
        assert_eq!(
            destination(&v, &SongMap::new()).unwrap(),
            "Library/Motions/_General/face_a"
        );
        v["song"] = json!("missing");
        assert!(matches!(
            destination(&v, &SongMap::new()),
            Err(LayoutError::UnknownSong(_))
        ));
    }
}
