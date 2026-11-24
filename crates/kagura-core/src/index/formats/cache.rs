use rusqlite::{params, Connection};

use super::{
    file_key,
    worker::{Candidate, Parsed, Texture},
    CachedParseError,
};
use crate::{
    index::IndexError,
    parse::{ModelInfo, MotionInfo},
};

fn names(db: &Connection, table: &str, id: i64, list: &[String]) -> Result<(), IndexError> {
    let mut insert = db.prepare_cached(&format!(
        "INSERT INTO {table}(info_id,ordinal,name) VALUES(?,?,?)"
    ))?;
    for (position, name) in (0_i64..).zip(list) {
        insert.execute(params![id, position, name])?;
    }
    Ok(())
}

fn save_model(
    db: &Connection,
    file: &Candidate,
    m: &ModelInfo,
    textures: &[Texture],
) -> Result<(), IndexError> {
    let mut insert_model = db.prepare_cached(
        "INSERT INTO model_info(asset_id,file,file_key,format,version,encoding,name,name_en,comment,comment_en,vertices,faces,materials,
         bones,morphs,rigid_bodies,joints,standard_bones,has_physics)
         VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?) RETURNING info_id",
    )?;
    let fields = params![
        file.id,
        file.rel,
        file_key(&file.rel),
        m.format,
        m.version,
        m.encoding,
        m.name,
        m.name_en,
        m.comment,
        m.comment_en,
        m.vertices,
        m.faces,
        m.materials,
        m.bones.len() as i64,
        m.morphs.len() as i64,
        m.rigid_bodies,
        m.joints,
        m.standard_bones,
        m.has_physics
    ];
    let id: i64 = insert_model.query_row(fields, |r| r.get(0))?;
    names(db, "model_bones", id, &m.bones)?;
    names(db, "model_morphs", id, &m.morphs)?;
    let mut insert = db.prepare_cached(
        "INSERT INTO model_textures(info_id,ordinal,name,resolved_path,present) VALUES(?,?,?,?,?)",
    )?;
    for (n, (name, texture)) in (0_i64..).zip(m.textures.iter().zip(textures)) {
        insert.execute(params![id, n, name, texture.rel, texture.present])?;
    }
    Ok(())
}

fn save_motion(db: &Connection, file: &Candidate, m: &MotionInfo) -> Result<(), IndexError> {
    let mut insert_motion = db.prepare_cached(
        "INSERT INTO motion_info(asset_id,file,target_model,is_camera,max_frame,seconds,bone_keys,morph_keys,camera_keys,
         light_keys,shadow_keys,ik_keys) VALUES(?,?,?,?,?,?,?,?,?,?,?,?) RETURNING info_id",
    )?;
    let fields = params![
        file.id,
        file.rel,
        m.target_model,
        m.is_camera,
        m.max_frame,
        m.seconds,
        m.bone_keys,
        m.morph_keys,
        m.camera_keys,
        m.light_keys,
        m.shadow_keys,
        m.ik_keys
    ];
    let id: i64 = insert_motion.query_row(fields, |r| r.get(0))?;
    names(db, "motion_bones", id, &m.bones)?;
    names(db, "motion_morphs", id, &m.morphs)
}

pub(super) fn save(
    db: &Connection,
    file: &Candidate,
    parsed: &Result<Parsed, CachedParseError>,
) -> Result<(), IndexError> {
    match parsed {
        Ok(Parsed::Model(m, textures)) => save_model(db, file, m, textures),
        Ok(Parsed::Motion(m)) => save_motion(db, file, m),
        Err(error) => {
            let json = serde_json::to_string(error)?;
            if file.format == "vmd" {
                db.prepare_cached("INSERT INTO motion_info(asset_id,file,parse_error) VALUES(?,?,?)")?
                    .execute(params![file.id, file.rel, json])?;
            } else {
                db.prepare_cached(
                    "INSERT INTO model_info(asset_id,file,file_key,format,parse_error) VALUES(?,?,?,?,?)",
                )?
                .execute(params![
                    file.id,
                    file.rel,
                    file_key(&file.rel),
                    file.format.to_ascii_uppercase(),
                    json
                ])?;
            }
            Ok(())
        }
    }
}
