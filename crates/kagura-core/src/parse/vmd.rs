use std::collections::HashSet;

use super::{
    reader::{decode_sjis, unpad, Reader, Result},
    MotionInfo,
};

fn named_frames(
    r: &mut Reader,
    section: &'static str,
    stride: u64,
    max_frame: &mut u32,
) -> Result<(u32, Vec<String>)> {
    r.section = section;
    let count = r.u32()?;
    r.ensure(u64::from(count) * stride)?;
    // Store the raw names once; thousands of keys usually address the same handful of channels.
    let mut names = HashSet::new();
    for _ in 0..count {
        let mut raw = r.bytes::<15>()?;
        let end = unpad(&raw).len();
        raw[end..].fill(0);
        names.insert(raw);
        if names.len() > 1_000_000 {
            return Err(r.error("more than one million distinct track names"));
        }
        *max_frame = (*max_frame).max(r.u32()?);
        r.skip(stride - 19)?;
    }
    let mut names: Vec<_> = names.iter().map(|raw| decode_sjis(raw)).collect();
    names.sort_unstable();
    names.dedup(); // Different malformed byte strings may decode to the same replacement character.
    Ok((count, names))
}

fn frame_section(r: &mut Reader, section: &'static str, stride: u64, max_frame: &mut u32) -> Result<u32> {
    r.section = section;
    if r.remaining() == 0 {
        return Ok(0);
    }
    let count = r.u32()?;
    r.ensure(u64::from(count) * stride)?;
    for _ in 0..count {
        *max_frame = (*max_frame).max(r.u32()?);
        r.skip(stride - 4)?;
    }
    Ok(count)
}

pub(super) fn parse(r: &mut Reader) -> Result<MotionInfo> {
    let signature = r.bytes::<30>()?;
    let target_model = match unpad(&signature) {
        b"Vocaloid Motion Data 0002" => r.sjis::<20>()?,
        b"Vocaloid Motion Data file" => r.sjis::<10>()?,
        _ => return Err(r.error("invalid VMD signature")),
    };
    let mut max_frame = 0;
    let (bone_keys, bones) = named_frames(r, "bone_keys", 111, &mut max_frame)?;
    let (morph_keys, morphs) = named_frames(r, "morph_keys", 23, &mut max_frame)?;
    // Older writers omit later sections entirely. A partial count or frame is always a truncation.
    let camera_keys = frame_section(r, "camera_keys", 61, &mut max_frame)?;
    let light_keys = frame_section(r, "light_keys", 28, &mut max_frame)?;
    let shadow_keys = frame_section(r, "shadow_keys", 9, &mut max_frame)?;
    r.section = "ik_keys";
    let ik_keys = if r.remaining() > 0 { r.u32()? } else { 0 };
    for _ in 0..ik_keys {
        max_frame = max_frame.max(r.u32()?);
        r.skip(1)?; // display on/off
        let ik_states = r.u32()?;
        r.rows(ik_states, 21)?; // fixed-width name and enabled flag per IK controller
    }
    Ok(MotionInfo {
        is_camera: target_model == "カメラ・照明" || camera_keys > 0,
        target_model,
        max_frame,
        seconds: f64::from(max_frame) / 30.0,
        bone_keys,
        morph_keys,
        camera_keys,
        light_keys,
        shadow_keys,
        ik_keys,
        bones,
        morphs,
    })
}
