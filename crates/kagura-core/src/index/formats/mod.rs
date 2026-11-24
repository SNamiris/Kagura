mod cache;
mod worker;

use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};

use super::IndexError;

pub(super) fn file_key(rel: &str) -> String {
    if cfg!(windows) {
        rel.to_lowercase()
    } else {
        rel.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct CachedParseError {
    pub section: String,
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub offset: u64,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct ModelFileInfo {
    pub file: String,
    pub format: String,
    pub version: Option<f32>,
    pub encoding: Option<String>,
    pub name: Option<String>,
    pub name_en: Option<String>,
    pub comment: Option<String>,
    pub comment_en: Option<String>,
    pub vertices: Option<u32>,
    pub faces: Option<u32>,
    pub materials: Option<u32>,
    pub bones: Option<u32>,
    pub morphs: Option<u32>,
    pub rigid_bodies: Option<u32>,
    pub joints: Option<u32>,
    pub standard_bones: Option<bool>,
    pub has_physics: Option<bool>,
    pub textures: Option<u32>,
    pub missing_textures: Option<u32>,
    pub parse_error: Option<CachedParseError>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct MotionFileInfo {
    pub file: String,
    pub target_model: Option<String>,
    pub is_camera: Option<bool>,
    pub max_frame: Option<u32>,
    pub seconds: Option<f64>,
    pub bone_keys: Option<u32>,
    pub morph_keys: Option<u32>,
    pub camera_keys: Option<u32>,
    pub light_keys: Option<u32>,
    pub shadow_keys: Option<u32>,
    pub ik_keys: Option<u32>,
    pub parse_error: Option<CachedParseError>,
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
pub struct ParseProgress {
    pub total: usize,
    pub cached: usize,
    pub processed: usize,
    pub failed: usize,
}

fn failure(row: &Row<'_>) -> rusqlite::Result<Option<CachedParseError>> {
    let index = row.as_ref().column_index("parse_error")?;
    let raw: Option<String> = row.get(index)?;
    raw.map(|json| {
        serde_json::from_str(&json).map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Text, Box::new(error))
        })
    })
    .transpose()
}

pub(super) fn models(db: &Connection, id: &str) -> Result<Vec<ModelFileInfo>, IndexError> {
    let mut statement = db.prepare_cached(
        "SELECT m.*, CASE WHEN a.type='model' AND a.kind='character' THEN m.standard_bones END AS character_bones,
         (SELECT count(*) FROM model_textures t WHERE t.info_id=m.info_id) AS textures,
         (SELECT count(*) FROM model_textures t WHERE t.info_id=m.info_id AND t.present=0) AS missing_textures
         FROM model_info m JOIN assets a ON a.id=m.asset_id WHERE m.asset_id=? ORDER BY m.file",
    )?;
    let rows = statement.query_map([id], |r| {
        let parse_error = failure(r)?;
        Ok(ModelFileInfo {
            file: r.get("file")?,
            format: r.get("format")?,
            version: r.get("version")?,
            encoding: r.get("encoding")?,
            name: r.get("name")?,
            name_en: r.get("name_en")?,
            comment: r.get("comment")?,
            comment_en: r.get("comment_en")?,
            vertices: r.get("vertices")?,
            faces: r.get("faces")?,
            materials: r.get("materials")?,
            bones: r.get("bones")?,
            morphs: r.get("morphs")?,
            rigid_bodies: r.get("rigid_bodies")?,
            joints: r.get("joints")?,
            standard_bones: r.get("character_bones")?,
            has_physics: r.get("has_physics")?,
            textures: if parse_error.is_none() {
                r.get("textures")?
            } else {
                None
            },
            missing_textures: if parse_error.is_none() {
                r.get("missing_textures")?
            } else {
                None
            },
            parse_error,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

pub(super) fn motions(db: &Connection, id: &str) -> Result<Vec<MotionFileInfo>, IndexError> {
    let mut statement = db.prepare_cached("SELECT * FROM motion_info WHERE asset_id=? ORDER BY file")?;
    let rows = statement.query_map([id], |r| {
        Ok(MotionFileInfo {
            file: r.get("file")?,
            target_model: r.get("target_model")?,
            is_camera: r.get("is_camera")?,
            max_frame: r.get("max_frame")?,
            seconds: r.get("seconds")?,
            bone_keys: r.get("bone_keys")?,
            morph_keys: r.get("morph_keys")?,
            camera_keys: r.get("camera_keys")?,
            light_keys: r.get("light_keys")?,
            shadow_keys: r.get("shadow_keys")?,
            ik_keys: r.get("ik_keys")?,
            parse_error: failure(r)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}
