//! 读取、识别和序列化 manifest。原始 JSON 保留字段顺序及视图没有用到的字段。

use crate::consts::FORMAT;
use crate::schema::ASSET_TYPES;
use serde_json::{Map, Value};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub fn read(path: &Path) -> Result<Value, ManifestError> {
    Ok(parse(&std::fs::read(path)?)?)
}

pub fn parse(bytes: &[u8]) -> serde_json::Result<Value> {
    serde_json::from_slice(bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes))
}

/// 以两个空格缩进并保留非 ASCII 字符，返回带 LF 结尾的 JSON 文本。
/// 数字采用 serde_json 的 i64/u64/f64 表示。浮点数的拼写可能与 Python 不同，
/// 例如 Python 输出 `1e-05`，当前 serde_json 输出 `0.00001`。
pub fn dumps(value: &Value) -> serde_json::Result<String> {
    let mut text = serde_json::to_string_pretty(value)?;
    text.push('\n');
    Ok(text)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataRole {
    Asset,
    ForeignManifest,
    RuntimeConfig,
    SourceJson,
}

fn looks_like_ulid(id: &str) -> bool {
    let bytes = id.as_bytes();
    bytes.len() == 26
        && matches!(bytes[0], b'0'..=b'7')
        && bytes[1..].iter().all(|b| matches!(b, b'0'..=b'9' | b'A'..=b'H' | b'J'..=b'K' | b'M'..=b'N' | b'P'..=b'T' | b'V'..=b'Z'))
}

/// 根据内容识别角色。调用者须用 schema 校验 Asset，再使用其类型化字段。
pub fn metadata_role(value: &Value) -> MetadataRole {
    let Some(object) = value.as_object() else {
        return MetadataRole::SourceJson;
    };
    let asset_like = object.contains_key("schema")
        || object.contains_key("classified_by")
        || value["id"].as_str().is_some_and(looks_like_ulid)
        || (value["type"].as_str().is_some_and(|ty| ASSET_TYPES.contains(&ty))
            && (object.contains_key("id") || object.contains_key("key")))
        || (object.contains_key("key") && object.contains_key("variants"));
    if asset_like {
        return if value["format"].as_str() == Some(FORMAT) {
            MetadataRole::Asset
        } else {
            MetadataRole::ForeignManifest
        };
    }
    let name = value["name"].as_str().unwrap_or_default();
    if (name.starts_with("char:") && value["character"].is_object())
        || (name.starts_with("stage:") && value["stage"].is_object())
    {
        MetadataRole::RuntimeConfig
    } else {
        MetadataRole::SourceJson
    }
}

/// 指向已校验资产的公共字段；不复制或改写 JSON。
#[derive(Debug, Clone, Copy)]
pub struct AssetView<'a> {
    pub id: &'a str,
    pub ty: &'a str,
    pub key: &'a Map<String, Value>,
}

impl<'a> AssetView<'a> {
    pub fn new(value: &'a Value) -> Option<Self> {
        Some(Self {
            id: value.get("id")?.as_str()?,
            ty: value.get("type")?.as_str()?,
            key: value.get("key")?.as_object()?,
        })
    }
}
