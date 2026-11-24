use std::io::{Seek, Write};

use super::*;

#[derive(Default)]
struct Bytes(Vec<u8>);

impl Bytes {
    fn append(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }
    fn zero(&mut self, n: usize) {
        self.0.resize(self.0.len() + n, 0);
    }
    fn u16(&mut self, n: u16) {
        self.append(&n.to_le_bytes());
    }
    fn u32(&mut self, n: u32) {
        self.append(&n.to_le_bytes());
    }
    fn fixed(&mut self, bytes: &[u8], size: usize) {
        self.append(bytes);
        self.zero(size - bytes.len());
    }
    fn text(&mut self, s: &str, utf16: bool) {
        let bytes = if utf16 {
            s.encode_utf16().flat_map(u16::to_le_bytes).collect()
        } else {
            s.as_bytes().to_vec()
        };
        self.u32(bytes.len() as u32);
        self.append(&bytes);
    }
    fn reader(&self) -> Reader {
        let mut f = tempfile::tempfile().unwrap();
        f.write_all(&self.0).unwrap();
        f.rewind().unwrap();
        Reader::new(f).unwrap()
    }
}

fn pmx_head(version: f32, utf16: bool) -> Bytes {
    let mut b = Bytes::default();
    b.append(b"PMX ");
    b.append(&version.to_le_bytes());
    // Two-byte bone indices, four-byte vertex indices and one additional UV.
    b.append(&[8, u8::from(!utf16), 1, 4, 1, 1, 2, 1, 1]);
    for s in ["日本語", "model", "紹介", "comment"] {
        b.text(s, utf16);
    }
    b
}

fn pmx_model(version: f32, utf16: bool) -> Bytes {
    let mut b = pmx_head(version, utf16);
    let weights = if version == 2.1 { 5 } else { 4 };
    b.u32(weights);
    for (kind, payload) in [2, 8, 24, 44, 24].into_iter().take(weights as usize).enumerate() {
        b.zero(48);
        b.append(&[kind as u8]);
        b.zero(payload + 4);
    }
    b.u32(3);
    b.zero(12);
    b.u32(1);
    b.text("tex/色.png", utf16);
    b.u32(1);
    b.text("mat", utf16);
    b.text("", utf16);
    b.zero(68);
    b.append(&[1, 0]);
    b.text("memo", utf16);
    b.u32(3);
    b.u32(1);
    b.text("頭", utf16);
    b.text("head", utf16);
    b.zero(18);
    b.u16(0x2f21);
    b.zero(2 + 6 + 12 + 24 + 4);
    b.zero(10);
    b.u32(1);
    b.zero(2);
    b.append(&[1]);
    b.zero(24);
    let types = if version == 2.1 { 11 } else { 9 };
    b.u32(types);
    for (i, stride) in [5, 16, 30, 20, 20, 20, 20, 20, 114, 5, 26]
        .into_iter()
        .take(types as usize)
        .enumerate()
    {
        b.text(&format!("M{i}"), utf16);
        b.text("", utf16);
        b.append(&[1, i as u8]);
        b.u32(1);
        b.zero(stride);
    }
    b.u32(1);
    b.text("display", utf16);
    b.text("", utf16);
    b.append(&[0]);
    b.u32(2);
    b.append(&[0, 0, 0, 1, 0]);
    b.u32(1);
    b.text("rigid", utf16);
    b.text("", utf16);
    b.zero(63);
    b.u32(1);
    b.text("joint", utf16);
    b.text("", utf16);
    b.zero(99);
    if version == 2.1 {
        b.u32(1);
        b.text("soft", utf16);
        b.text("", utf16);
        b.zero(126);
        b.u32(1);
        b.zero(6);
        b.u32(2);
        b.zero(8);
    }
    b
}

#[test]
fn pmx_versions_encodings_indices_and_variable_sections() {
    for (version, utf16) in [(2.0, true), (2.1, false)] {
        let b = pmx_model(version, utf16);
        let mut r = b.reader();
        let model = pmx::parse(&mut r).unwrap();
        assert_eq!(r.remaining(), 0);
        assert_eq!(
            (model.name.as_str(), model.comment_en.as_str()),
            ("日本語", "comment")
        );
        assert_eq!(model.version, version);
        assert_eq!(
            (model.vertices, model.faces, model.materials),
            (if version == 2.1 { 5 } else { 4 }, 1, 1)
        );
        assert_eq!(model.textures, ["tex/色.png"]);
        assert_eq!(model.bones, ["頭"]);
        assert_eq!(model.morphs.len(), if version == 2.1 { 11 } else { 9 });
        assert_eq!((model.rigid_bodies, model.joints), (1, 1));
        assert!(model.has_required_bones(&["頭"]));
        assert!(!model.has_required_bones(STANDARD_BONES));
    }
}

fn pmd_model() -> Bytes {
    let mut b = Bytes::default();
    b.append(b"Pmd");
    b.append(&1f32.to_le_bytes());
    b.fixed(&[0x83, 0x7e, 0x83, 0x4e], 20);
    b.fixed(b"comment", 256);
    b.u32(3);
    b.zero(114);
    b.u32(3);
    b.zero(6);
    b.u32(1);
    b.zero(50);
    b.fixed(b"body.png*env.spa", 20);
    b.u16(1);
    b.fixed(b"bone", 20);
    b.zero(19);
    b.u16(1);
    b.zero(4);
    b.append(&[2]);
    b.zero(10);
    b.u16(2);
    for (name, kind) in [("base", 0), ("happy", 3)] {
        b.fixed(name.as_bytes(), 20);
        b.u32(1);
        b.append(&[kind]);
        b.zero(16);
    }
    b.append(&[1]);
    b.zero(2);
    b.append(&[1]);
    b.zero(50);
    b.u32(1);
    b.zero(3);
    b.append(&[1]);
    b.fixed(b"Miku", 20);
    b.fixed(b"hello", 256);
    b.zero(20 + 20 + 50 + 1000);
    b.u32(1);
    b.zero(83);
    b.u32(1);
    b.zero(124);
    b
}

#[test]
fn pmd_english_extension_texture_pairs_and_base_morph() {
    let b = pmd_model();
    let mut r = b.reader();
    let model = pmd::parse(&mut r).unwrap();
    assert_eq!(r.remaining(), 0);
    assert_eq!((model.name.as_str(), model.name_en.as_str()), ("ミク", "Miku"));
    assert_eq!((model.vertices, model.faces, model.materials), (3, 1, 1));
    assert_eq!(model.textures, ["body.png", "env.spa"]);
    assert_eq!(model.bones, ["bone"]);
    assert_eq!(model.morphs, ["happy"]);
    assert_eq!((model.rigid_bodies, model.joints), (1, 1));
}

fn vmd_motion(legacy: bool) -> Bytes {
    let mut b = Bytes::default();
    b.fixed(
        if legacy {
            b"Vocaloid Motion Data file"
        } else {
            b"Vocaloid Motion Data 0002"
        },
        30,
    );
    b.fixed(&[0x83, 0x7e, 0x83, 0x4e], if legacy { 10 } else { 20 });
    b.u32(2);
    for frame in [18, 1] {
        b.fixed(b"bone", 15);
        b.u32(frame);
        b.zero(92);
    }
    b.u32(1);
    b.fixed(&[0x82, 0, 0xfd], 15);
    b.u32(25);
    b.zero(4);
    b.u32(1);
    b.u32(50);
    b.zero(57);
    b.u32(1);
    b.u32(60);
    b.zero(24);
    b.u32(1);
    b.u32(80);
    b.zero(5);
    b.u32(2);
    for frame in [120, 30] {
        b.u32(frame);
        b.append(&[1]);
        b.u32(2);
        b.zero(42);
    }
    b
}

#[test]
fn both_vmd_headers_all_six_sections_and_lossy_fixed_width_names() {
    for legacy in [false, true] {
        let mut r = vmd_motion(legacy).reader();
        let v = vmd::parse(&mut r).unwrap();
        assert_eq!(r.remaining(), 0);
        assert_eq!(v.target_model, "ミク");
        assert_eq!(
            [
                v.bone_keys,
                v.morph_keys,
                v.camera_keys,
                v.light_keys,
                v.shadow_keys,
                v.ik_keys
            ],
            [2, 1, 1, 1, 1, 2]
        );
        assert!(v.is_camera);
        assert_eq!(v.max_frame, 120);
        assert_eq!(v.seconds, 4.0);
        assert_eq!(v.bones, ["bone"]);
        assert_eq!(v.morphs, ["�"]);
    }
    let mut b = Bytes::default();
    b.fixed(b"Vocaloid Motion Data 0002", 30);
    b.fixed(&encoding_rs::SHIFT_JIS.encode("カメラ・照明").0, 20);
    b.u32(0);
    b.u32(0);
    let v = vmd::parse(&mut b.reader()).unwrap();
    assert!(v.is_camera);
    assert_eq!(v.ik_keys, 0);
    assert_eq!(v.max_frame, 0);
}

#[test]
fn open_files_rewind_and_standard_bones_can_be_configured() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("model.PMX");
    std::fs::write(&path, pmx_model(2.1, false).0).unwrap();
    let default = parse_model(&path).unwrap();
    assert!(default.has_physics);
    assert!(!default.standard_bones);
    let mut file = File::open(&path).unwrap();
    file.seek(std::io::SeekFrom::End(0)).unwrap();
    let custom = parse_model_file_with_bones(file, "PMX", &["頭"]).unwrap();
    assert!(custom.standard_bones);
}

#[test]
fn truncated_files_and_hostile_counts_report_section_and_offset() {
    let p = pmx_model(2.1, false);
    for cut in [3, 9, 18, 100, 200, p.0.len() - 1] {
        let err = pmx::parse(&mut Bytes(p.0[..cut].to_vec()).reader()).unwrap_err();
        assert!(!err.section.is_empty());
        assert!(err.offset <= cut as u64);
    }
    let mut bad = pmx_head(2.0, false);
    bad.u32(u32::MAX);
    let e = pmx::parse(&mut bad.reader()).unwrap_err();
    assert_eq!(e.section, "vertices");
    let mut n = pmx_head(2.0, false);
    n.0.truncate(17);
    n.u32(i32::MAX as u32);
    let e = pmx::parse(&mut n.reader()).unwrap_err();
    assert_eq!(e.section, "model_text");
    let p = pmd_model();
    for cut in [6, 100, 400, p.0.len() - 2] {
        let e = pmd::parse(&mut Bytes(p.0[..cut].to_vec()).reader()).unwrap_err();
        assert!(e.offset <= cut as u64);
    }
    let v = vmd_motion(false);
    for (cut, section) in [
        (20, "header"),
        (49, "header"),
        (52, "bone_keys"),
        (56, "bone_keys"),
        (v.0.len() - 1, "ik_keys"),
    ] {
        let e = vmd::parse(&mut Bytes(v.0[..cut].to_vec()).reader()).unwrap_err();
        assert_eq!(e.section, section);
        assert!(e.offset <= cut as u64);
    }
}
