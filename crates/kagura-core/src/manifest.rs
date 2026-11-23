//! 读取、识别和序列化 manifest。原始 JSON 保留字段顺序及视图没有用到的字段。

use crate::consts::{DEFAULT_MANIFEST_NEWLINE, FORMAT};
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

const UTF8_BOM: &[u8] = b"\xef\xbb\xbf";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewlineStyle {
    Lf,
    Crlf,
    Mixed { lf: usize, crlf: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentFormat {
    pub newline: NewlineStyle,
    pub bom: bool,
}

impl Default for DocumentFormat {
    fn default() -> Self {
        Self {
            newline: DEFAULT_MANIFEST_NEWLINE,
            bom: false,
        }
    }
}

impl DocumentFormat {
    /// 在解析 JSON 前检查原始字节；字符串中的反斜杠转义不会计为换行。
    pub fn detect(bytes: &[u8]) -> Self {
        let mut lf = 0;
        let mut crlf = 0;
        for line in bytes.split_inclusive(|&b| b == b'\n') {
            if line.ends_with(b"\r\n") {
                crlf += 1;
            } else if line.ends_with(b"\n") {
                lf += 1;
            }
        }
        let newline = match (lf, crlf) {
            (_, 0) => NewlineStyle::Lf,
            (0, _) => NewlineStyle::Crlf,
            _ => NewlineStyle::Mixed { lf, crlf },
        };
        Self {
            newline,
            bom: bytes.starts_with(UTF8_BOM),
        }
    }
}

#[derive(Debug)]
pub struct Document {
    pub value: Value,
    pub format: DocumentFormat,
}

pub fn read_document(path: &Path) -> Result<Document, ManifestError> {
    Ok(parse_document(&std::fs::read(path)?)?)
}

pub fn parse_document(bytes: &[u8]) -> serde_json::Result<Document> {
    Ok(Document {
        value: parse(bytes)?,
        format: DocumentFormat::detect(bytes),
    })
}

/// 仅读取 JSON 值；要编辑并写回原文件时使用 read_document 保存文件格式。
pub fn read(path: &Path) -> Result<Value, ManifestError> {
    Ok(parse(&std::fs::read(path)?)?)
}

pub fn parse(bytes: &[u8]) -> serde_json::Result<Value> {
    serde_json::from_slice(bytes.strip_prefix(UTF8_BOM).unwrap_or(bytes))
}

/// 以两个空格缩进并保留非 ASCII 字符，返回带 LF 结尾的 JSON 文本。
/// 数字采用 serde_json 的 i64/u64/f64 表示。浮点数的拼写可能与 Python 不同，
/// 例如 Python 输出 `1e-05`，当前 serde_json 输出 `0.00001`。
pub fn dumps(value: &Value) -> serde_json::Result<String> {
    let mut text = serde_json::to_string_pretty(value)?;
    text.push('\n');
    Ok(text)
}

#[derive(Debug)]
pub struct EncodedDocument {
    pub bytes: Vec<u8>,
    pub normalized_newlines: bool,
}

/// 写回时传入原格式，新文件传入 DocumentFormat::default()。
pub fn encode(value: &Value, format: DocumentFormat) -> serde_json::Result<EncodedDocument> {
    let use_crlf = match format.newline {
        NewlineStyle::Lf => false,
        NewlineStyle::Crlf => true,
        NewlineStyle::Mixed { lf, crlf } => {
            crlf > lf || (crlf == lf && DEFAULT_MANIFEST_NEWLINE == NewlineStyle::Crlf)
        }
    };
    let text = dumps(value)?;
    let text = if use_crlf {
        text.replace('\n', "\r\n")
    } else {
        text
    };
    let bytes = if format.bom {
        [UTF8_BOM, text.as_bytes()].concat()
    } else {
        text.into_bytes()
    };
    Ok(EncodedDocument {
        bytes,
        normalized_newlines: matches!(format.newline, NewlineStyle::Mixed { .. }),
    })
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
