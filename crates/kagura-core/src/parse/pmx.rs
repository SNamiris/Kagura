use encoding_rs::{Encoding, UTF_16LE, UTF_8};

use super::{
    reader::{Reader, Result},
    ModelInfo,
};

struct Header {
    version: f32,
    encoding: &'static Encoding,
    extra_uv: u64,
    vertex: u64,
    texture: u64,
    material: u64,
    bone: u64,
    morph: u64,
    rigid: u64,
}

impl Header {
    fn read(r: &mut Reader) -> Result<Self> {
        if &r.bytes::<4>()? != b"PMX " {
            return Err(r.error("invalid PMX signature"));
        }
        let version = r.f32()?;
        if version != 2.0 && version != 2.1 {
            return Err(r.error(format!("unsupported PMX version {version}")));
        }
        let len = r.u8()?;
        if len < 8 {
            return Err(r.error("PMX header needs eight settings"));
        }
        let encoding = match r.u8()? {
            0 => UTF_16LE,
            1 => UTF_8,
            n => return Err(r.error(format!("unsupported PMX encoding {n}"))),
        };
        let extra_uv = r.u8()?;
        if extra_uv > 4 {
            return Err(r.error("additional UV count exceeds four"));
        }
        let sizes = r.bytes::<6>()?;
        if !sizes.iter().all(|size| matches!(size, 1 | 2 | 4)) {
            return Err(r.error("invalid PMX index width"));
        }
        let [vertex, texture, material, bone, morph, rigid] = sizes.map(u64::from);
        r.skip(u64::from(len - 8))?;
        Ok(Self {
            version,
            encoding,
            extra_uv: u64::from(extra_uv),
            vertex,
            texture,
            material,
            bone,
            morph,
            rigid,
        })
    }

    fn vertices(&self, r: &mut Reader) -> Result<u32> {
        r.section = "vertices";
        let count = r.count()?;
        for _ in 0..count {
            r.skip(32 + 16 * self.extra_uv)?;
            let weight = match r.u8()? {
                0 => self.bone,
                1 => 2 * self.bone + 4,
                2 => 4 * self.bone + 16,
                3 => 2 * self.bone + 40,
                4 if self.version == 2.1 => 4 * self.bone + 16,
                n => return Err(r.error(format!("unsupported vertex weight type {n}"))),
            };
            r.skip(weight + 4)?;
        }
        Ok(count)
    }

    fn materials(&self, r: &mut Reader) -> Result<u32> {
        r.section = "materials";
        let count = r.count()?;
        for _ in 0..count {
            r.skip_text()?;
            r.skip_text()?;
            r.skip(65 + 2 * self.texture + 1)?;
            let toon_width = if r.flag()? { 1 } else { self.texture };
            r.skip(toon_width)?;
            r.skip_text()?;
            r.skip(4)?;
        }
        Ok(count)
    }

    fn bones(&self, r: &mut Reader) -> Result<Vec<String>> {
        r.section = "bones";
        let mut names = Vec::new();
        for _ in 0..r.count()? {
            names.push(r.pmx_text(self.encoding)?);
            r.skip_text()?;
            r.skip(16 + self.bone)?;
            let flags = r.u16()?;
            r.skip(if flags & 1 != 0 { self.bone } else { 12 })?;
            if flags & 0x0300 != 0 {
                r.skip(self.bone + 4)?;
            }
            if flags & 0x0400 != 0 {
                r.skip(12)?;
            }
            if flags & 0x0800 != 0 {
                r.skip(24)?;
            }
            if flags & 0x2000 != 0 {
                r.skip(4)?;
            }
            if flags & 0x0020 != 0 {
                r.skip(self.bone + 8)?;
                for _ in 0..r.count()? {
                    r.skip(self.bone)?;
                    if r.flag()? {
                        r.skip(24)?;
                    }
                }
            }
        }
        Ok(names)
    }

    fn morphs(&self, r: &mut Reader) -> Result<Vec<String>> {
        r.section = "morphs";
        let mut names = Vec::new();
        for _ in 0..r.count()? {
            names.push(r.pmx_text(self.encoding)?);
            r.skip_text()?;
            r.skip(1)?; // display panel
            let stride = match r.u8()? {
                0 => self.morph + 4,
                1 => self.vertex + 12,
                2 => self.bone + 28,
                3..=7 => self.vertex + 16,
                8 => self.material + 113,
                9 if self.version == 2.1 => self.morph + 4,
                10 if self.version == 2.1 => self.rigid + 25,
                n => return Err(r.error(format!("unsupported morph type {n}"))),
            };
            let count = r.count()?;
            r.rows(count, stride)?;
        }
        Ok(names)
    }

    fn display(&self, r: &mut Reader) -> Result<()> {
        r.section = "display_frames";
        for _ in 0..r.count()? {
            r.skip_text()?;
            r.skip_text()?;
            r.skip(1)?;
            for _ in 0..r.count()? {
                let width = if r.flag()? { self.morph } else { self.bone };
                r.skip(width)?;
            }
        }
        Ok(())
    }

    fn named_physics(&self, r: &mut Reader, section: &'static str, stride: u64) -> Result<u32> {
        r.section = section;
        let count = r.count()?;
        for _ in 0..count {
            r.skip_text()?;
            r.skip_text()?;
            r.skip(stride)?;
        }
        Ok(count)
    }

    fn soft_bodies(&self, r: &mut Reader) -> Result<()> {
        r.section = "soft_bodies";
        for _ in 0..r.count()? {
            r.skip_text()?;
            r.skip_text()?;
            // Shape, material, group/mask/flags, topology, configuration and Bullet coefficients.
            r.skip(125 + self.material)?;
            let anchors = r.count()?;
            r.rows(anchors, self.rigid + self.vertex + 1)?;
            let pins = r.count()?;
            r.rows(pins, self.vertex)?;
        }
        Ok(())
    }
}

pub(super) fn parse(r: &mut Reader) -> Result<ModelInfo> {
    let h = Header::read(r)?;
    r.section = "model_text";
    let mut m = ModelInfo {
        format: "PMX".into(),
        version: h.version,
        encoding: h.encoding.name().into(),
        name: r.pmx_text(h.encoding)?,
        name_en: r.pmx_text(h.encoding)?,
        comment: r.pmx_text(h.encoding)?,
        comment_en: r.pmx_text(h.encoding)?,
        ..Default::default()
    };
    m.vertices = h.vertices(r)?;
    r.section = "faces";
    let indices = r.count()?;
    m.faces = r.triangles(indices, h.vertex)?;
    r.section = "textures";
    for _ in 0..r.count()? {
        m.textures.push(r.pmx_text(h.encoding)?);
    }
    m.materials = h.materials(r)?;
    m.bones = h.bones(r)?;
    m.morphs = h.morphs(r)?;
    h.display(r)?;
    m.rigid_bodies = h.named_physics(r, "rigid_bodies", h.bone + 61)?;
    m.joints = h.named_physics(r, "joints", 2 * h.rigid + 97)?;
    if h.version == 2.1 {
        h.soft_bodies(r)?;
    }
    Ok(m)
}
