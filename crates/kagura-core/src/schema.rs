//! 内嵌 JSON Schema（2020-12），按资产 `type` 校验 manifest。
//!
//! 所有 schema 在编译时用 `include_str!` 嵌入，用 `Registry` 解析跨文件 `$ref`，
//! 不读文件、不联网（`offline()`）。2020-12 默认只把 `format` 当注解，这里显式打开检查，
//! 与 Python 版 `Draft202012Validator(..., format_checker=FormatChecker())` 一致。

use jsonschema::{Registry, Validator};
use serde_json::Value;
use thiserror::Error;

/// (`$id` 用的文件名, 内容)
const SOURCES: [(&str, &str); 8] = [
    ("common.schema.json", include_str!("../../../schemas/common.schema.json")),
    ("model.schema.json", include_str!("../../../schemas/model.schema.json")),
    ("stage.schema.json", include_str!("../../../schemas/stage.schema.json")),
    ("song.schema.json", include_str!("../../../schemas/song.schema.json")),
    ("motion_part.schema.json", include_str!("../../../schemas/motion_part.schema.json")),
    ("project.schema.json", include_str!("../../../schemas/project.schema.json")),
    ("game.schema.json", include_str!("../../../schemas/game.schema.json")),
    ("aliases.schema.json", include_str!("../../../schemas/aliases.schema.json")),
];

/// 资产 manifest 的类型（不含 project / game / aliases）。
pub const ASSET_TYPES: [&str; 4] = ["model", "stage", "song", "motion_part"];

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("内嵌 schema 无法解析：{0}")]
    Parse(String),
    #[error("schema 编译失败：{0}")]
    Build(String),
    #[error("未知的资产类型：{0:?}")]
    UnknownType(Option<String>),
}

/// 一条校验错误：实例路径 + 说明。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub path: String,
    pub message: String,
}

pub struct Schemas {
    validators: Vec<(&'static str, Validator)>,
}

impl Schemas {
    /// 编译全部 schema。只需在启动时做一次。
    pub fn load() -> Result<Self, SchemaError> {
        let docs: Vec<(&str, Value)> = SOURCES
            .iter()
            .map(|(name, text)| {
                serde_json::from_str::<Value>(text)
                    .map(|v| (*name, v))
                    .map_err(|e| SchemaError::Parse(format!("{name}: {e}")))
            })
            .collect::<Result<_, _>>()?;

        let mut registry = Registry::new();
        for (name, doc) in &docs {
            let id = doc
                .get("$id")
                .and_then(Value::as_str)
                .ok_or_else(|| SchemaError::Parse(format!("{name}: 缺少 $id")))?;
            registry = registry
                .add(id, doc.clone())
                .map_err(|e| SchemaError::Build(format!("{name}: {e}")))?;
        }
        let registry = registry
            .prepare()
            .map_err(|e| SchemaError::Build(e.to_string()))?;

        let mut validators = Vec::new();
        for (name, doc) in &docs {
            let ty = name.trim_end_matches(".schema.json");
            if ty == "common" {
                continue;
            }
            let v = jsonschema::draft202012::options()
                .offline()
                .should_validate_formats(true)
                .with_registry(&registry)
                .build(doc)
                .map_err(|e| SchemaError::Build(format!("{name}: {e}")))?;
            validators.push((leak(ty), v));
        }
        Ok(Self { validators })
    }

    fn get(&self, ty: &str) -> Option<&Validator> {
        self.validators.iter().find(|(t, _)| *t == ty).map(|(_, v)| v)
    }

    /// 按 `type` 字段选择 schema 校验资产 manifest。**不修改**传入的数据。
    /// 返回最多 `limit` 条错误；空列表表示通过。
    pub fn validate_asset(&self, manifest: &Value, limit: usize) -> Result<Vec<Issue>, SchemaError> {
        let ty = manifest.get("type").and_then(Value::as_str);
        let ty = match ty {
            Some(t) if ASSET_TYPES.contains(&t) => t,
            other => return Err(SchemaError::UnknownType(other.map(str::to_owned))),
        };
        self.validate_as(ty, manifest, limit)
    }

    /// 用指定 schema（含 project / game / aliases）校验。
    pub fn validate_as(&self, ty: &str, value: &Value, limit: usize) -> Result<Vec<Issue>, SchemaError> {
        let v = self
            .get(ty)
            .ok_or_else(|| SchemaError::UnknownType(Some(ty.to_owned())))?;
        Ok(v.iter_errors(value)
            .take(limit)
            .map(|e| Issue {
                path: e.instance_path().to_string(),
                message: e.to_string(),
            })
            .collect())
    }
}

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_owned().into_boxed_str())
}
