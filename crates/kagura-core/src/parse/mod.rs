//! Streaming structural readers for MikuMikuDance model and motion files.
mod pmd;
mod pmx;
mod reader;
#[cfg(test)]
mod tests;
mod vmd;

use std::{fs::File, path::Path};

use serde::Serialize;
use thiserror::Error;

use reader::Reader;

pub const STANDARD_BONES: &[&str] = &[
    "センター",
    "上半身",
    "下半身",
    "首",
    "頭",
    "左腕",
    "右腕",
    "左足",
    "右足",
];

#[derive(Debug, Serialize, Default)]
pub struct ModelInfo {
    pub format: String,
    pub version: f32,
    pub encoding: String,
    pub name: String,
    pub name_en: String,
    pub comment: String,
    pub comment_en: String,
    pub vertices: u32,
    /// Number of triangles, i.e. the on-disk vertex-index count divided by three.
    pub faces: u32,
    pub textures: Vec<String>,
    pub materials: u32,
    pub bones: Vec<String>,
    pub morphs: Vec<String>,
    pub rigid_bodies: u32,
    pub joints: u32,
    pub standard_bones: bool,
    pub has_physics: bool,
}

impl ModelInfo {
    pub fn has_required_bones(&self, required: &[&str]) -> bool {
        required
            .iter()
            .all(|name| self.bones.iter().any(|bone| bone == name))
    }
}

#[derive(Debug, Serialize, Default)]
pub struct MotionInfo {
    pub target_model: String,
    pub is_camera: bool,
    pub max_frame: u32,
    pub seconds: f64,
    pub bone_keys: u32,
    pub morph_keys: u32,
    pub camera_keys: u32,
    pub light_keys: u32,
    pub shadow_keys: u32,
    pub ik_keys: u32,
    pub bones: Vec<String>,
    pub morphs: Vec<String>,
}

#[derive(Debug, Error, Serialize)]
#[error("{section} @ {offset}: {message}")]
pub struct ParseError {
    pub section: &'static str,
    pub offset: u64,
    pub message: String,
}

fn open_error(error: std::io::Error) -> ParseError {
    ParseError {
        section: "file",
        offset: 0,
        message: error.to_string(),
    }
}

pub fn parse_model(path: &Path) -> Result<ModelInfo, ParseError> {
    parse_model_file(
        File::open(path).map_err(open_error)?,
        path.extension().and_then(|s| s.to_str()).unwrap_or(""),
    )
}

pub fn parse_model_file(file: File, extension: &str) -> Result<ModelInfo, ParseError> {
    parse_model_file_with_bones(file, extension, STANDARD_BONES)
}

/// Parse with a caller-supplied skeletal convention (e.g. an animation pipeline's required names).
pub fn parse_model_file_with_bones(
    file: File,
    extension: &str,
    required: &[&str],
) -> Result<ModelInfo, ParseError> {
    let mut reader = Reader::new(file)?;
    let mut info = if extension.eq_ignore_ascii_case("pmx") {
        pmx::parse(&mut reader)?
    } else if extension.eq_ignore_ascii_case("pmd") {
        pmd::parse(&mut reader)?
    } else {
        return Err(reader.error("unsupported model extension"));
    };
    info.standard_bones = info.has_required_bones(required);
    info.has_physics = info.rigid_bodies > 0;
    Ok(info)
}

pub fn parse_motion(path: &Path) -> Result<MotionInfo, ParseError> {
    parse_motion_file(File::open(path).map_err(open_error)?)
}

pub fn parse_motion_file(file: File) -> Result<MotionInfo, ParseError> {
    vmd::parse(&mut Reader::new(file)?)
}
