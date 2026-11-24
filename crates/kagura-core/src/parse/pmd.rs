use std::collections::HashSet;

use super::{
    reader::{Reader, Result},
    ModelInfo,
};

pub(super) fn parse(r: &mut Reader) -> Result<ModelInfo> {
    if &r.bytes::<3>()? != b"Pmd" {
        return Err(r.error("invalid PMD signature"));
    }
    let version = r.f32()?;
    if version != 1.0 {
        return Err(r.error(format!("unsupported PMD version {version}")));
    }
    r.section = "model_text";
    let mut m = ModelInfo {
        format: "PMD".into(),
        version,
        encoding: "Shift_JIS".into(),
        name: r.sjis::<20>()?,
        comment: r.sjis::<256>()?,
        ..Default::default()
    };
    r.section = "vertices";
    m.vertices = r.u32()?;
    r.rows(m.vertices, 38)?;
    r.section = "faces";
    let indices = r.u32()?;
    m.faces = r.triangles(indices, 2)?;
    r.section = "materials";
    m.materials = r.u32()?;
    let mut seen = HashSet::new();
    for _ in 0..m.materials {
        r.skip(50)?;
        for path in r.sjis::<20>()?.split('*').filter(|p| !p.is_empty()) {
            if seen.insert(path.to_owned()) {
                m.textures.push(path.to_owned());
            }
        }
    }

    r.section = "bones";
    let bones = r.u16()?;
    for _ in 0..bones {
        m.bones.push(r.sjis::<20>()?);
        r.skip(19)?;
    }
    r.section = "ik";
    for _ in 0..r.u16()? {
        r.skip(4)?;
        let links = r.u8()?;
        r.skip(6 + 2 * u64::from(links))?;
    }
    r.section = "morphs";
    let morphs = r.u16()?;
    for _ in 0..morphs {
        let name = r.sjis::<20>()?;
        let vertices = r.u32()?;
        let category = r.u8()?;
        if category != 0 {
            m.morphs.push(name);
        } // category zero contains base vertices
        r.rows(vertices, 16)?;
    }
    r.section = "display_frames";
    let facial = r.u8()?;
    r.skip(2 * u64::from(facial))?;
    let frames = r.u8()?;
    r.skip(50 * u64::from(frames))?;
    let displays = r.u32()?;
    r.rows(displays, 3)?;

    // Historical PMD can end at the display table, or after the toon table (before physics).
    if r.remaining() == 0 {
        return Ok(m);
    }
    r.section = "english_extension";
    if r.flag()? {
        m.name_en = r.sjis::<20>()?;
        m.comment_en = r.sjis::<256>()?;
        r.skip(20 * u64::from(bones) + 20 * u64::from(morphs.saturating_sub(1)) + 50 * u64::from(frames))?;
    }
    r.section = "toon_textures";
    r.skip(10 * 100)?;
    if r.remaining() == 0 {
        return Ok(m);
    }
    r.section = "rigid_bodies";
    m.rigid_bodies = r.u32()?;
    r.rows(m.rigid_bodies, 83)?;
    r.section = "joints";
    m.joints = r.u32()?;
    r.rows(m.joints, 124)?;
    Ok(m)
}
