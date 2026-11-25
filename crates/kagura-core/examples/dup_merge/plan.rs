use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs::{self, File},
    io::{ErrorKind, Read},
    path::Path,
};

use kagura_core::{
    consts::MANIFEST,
    index::Index,
    layout::{self, SongMap},
    manifest, naming,
    scan::{is_reparse, scan_library_for_check},
    schema::Schemas,
};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use unicase::UniCase;

use super::{
    ensure,
    journal::{self, digest, Kind, Step, View},
    Result, ARCHIVE, DATE,
};

#[derive(Deserialize)]
pub struct Report {
    format: String,
    groups: Vec<Group>,
}

#[derive(Deserialize)]
pub struct Group {
    group: u32,
    assets: Vec<Asset>,
    review: Review,
    plan: Value,
}

#[derive(Deserialize)]
struct Asset {
    id: String,
    rel_dir: String,
    manifest: Value,
    manifest_sha256: String,
    #[serde(default)]
    manifest_blake3: Option<String>,
    payload: Vec<Payload>,
    incoming: Vec<Incoming>,
}

#[derive(Deserialize)]
struct Payload {
    rel: String,
    size: u64,
    blake3: String,
}

#[derive(Deserialize)]
struct Review {
    automatic_draft: bool,
    category: String,
    keep_id: Option<String>,
    canonical_dir: Option<String>,
    candidate_archive_ids: Vec<String>,
}

#[derive(Deserialize)]
struct Plan {
    actions: Vec<Action>,
}

#[derive(Deserialize)]
struct Action {
    step: usize,
    #[serde(flatten)]
    op: Operation,
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Operation {
    Merge {
        scope: String,
        keep_id: String,
        retire_ids: Vec<String>,
        set_fields: Vec<FieldEdit>,
        redirects: Vec<Redirect>,
        target_dir: String,
    },
    Quarantine {
        id: String,
        from_dir: String,
    },
    Rekey {
        id: String,
        from_dir: String,
        to_dir: String,
        fields: Vec<FieldEdit>,
    },
}

#[derive(Deserialize)]
struct FieldEdit {
    pointer: String,
    op: String,
    before: Value,
    value: Value,
}

#[derive(Deserialize)]
struct Redirect {
    from_id: String,
    pointer: String,
    before: String,
    after: String,
}

#[derive(Deserialize)]
struct Incoming {
    from_id: Option<String>,
    pointer: String,
}

impl Report {
    pub fn eligible(&self, only: &BTreeSet<u32>) -> Result<Vec<&Group>> {
        ensure(self.format == "kagura-duplicate-asset-review/v1", || {
            "不支持的报告格式".into()
        })?;
        let mut seen = BTreeSet::new();
        for g in &self.groups {
            ensure(seen.insert(g.group), || format!("重复组号：{}", g.group))?;
        }
        let eligible: Vec<_> = self
            .groups
            .iter()
            .filter(|g| g.review.automatic_draft && (only.is_empty() || only.contains(&g.group)))
            .collect();
        ensure(only.is_empty() || eligible.len() == only.len(), || {
            "--only 包含未知或人工确认组".into()
        })?;
        Ok(eligible)
    }
}

type RefKey = (Option<String>, String);

struct Library {
    records: HashMap<String, (String, Value)>,
    files: HashMap<String, BTreeMap<String, u64>>,
    refs: HashMap<String, BTreeSet<RefKey>>,
    songs: SongMap,
    dirs: BTreeMap<UniCase<String>, String>,
    schemas: Schemas,
}

impl Library {
    fn read(root: &Path) -> Result<Self> {
        let report = scan_library_for_check(root);
        // The scanner omits unreadable or invalid records. Continuing could miss incoming edges.
        ensure(report.errors.is_empty(), || {
            format!("扫描失败，无法可靠复核全库引用：{:?}", report.errors.first())
        })?;
        let temp = tempfile::tempdir()?;
        let db = temp.path().join("fresh.sqlite");
        Index::open_for_library(&db, root)?.rebuild(&report)?;
        let conn = Connection::open(&db)?;
        let mut stmt = conn.prepare("SELECT asset_id,rel,size FROM files")?;
        let mut files = HashMap::<String, BTreeMap<String, u64>>::new();
        for row in stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
            ))
        })? {
            let (id, rel, size) = row?;
            files.entry(id).or_default().insert(rel, u64::try_from(size)?);
        }
        let mut refs = HashMap::<String, BTreeSet<RefKey>>::new();
        let mut stmt = conn.prepare("SELECT from_id,to_id,field FROM refs")?;
        for row in stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })? {
            let (from, to, field) = row?;
            let pointer = format!("/{}", field.replace(['.', '['], "/").replace(']', ""));
            refs.entry(to).or_default().insert((Some(from), pointer));
        }
        read_project_refs(&root.join("Projects"), &mut refs)?;
        let songs = kagura_core::scan::song_map(&report.records)?;
        let dirs = report
            .directories
            .into_iter()
            .map(|d| (UniCase::new(d.path.clone()), d.path))
            .collect();
        let records = report
            .records
            .into_iter()
            .map(|r| {
                (
                    r.value["id"].as_str().expect("validated id").to_owned(),
                    (r.dir, r.value),
                )
            })
            .collect();
        Ok(Self {
            records,
            files,
            refs,
            songs,
            dirs,
            schemas: Schemas::load()?,
        })
    }
}

fn read_project_refs(dir: &Path, refs: &mut HashMap<String, BTreeSet<RefKey>>) -> Result<()> {
    match fs::symlink_metadata(dir) {
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e.into()),
        Ok(m) if is_reparse(&m) || !m.is_dir() => return Ok(()),
        Ok(_) => (),
    }
    for e in fs::read_dir(dir)? {
        let p = e?.path();
        let m = fs::symlink_metadata(&p)?;
        if is_reparse(&m) {
            continue;
        }
        if m.is_dir() {
            read_project_refs(&p, refs)?;
        } else if m.is_file() && p.extension().is_some_and(|x| x.eq_ignore_ascii_case("json")) {
            let raw = fs::read(&p)?;
            let value = match manifest::parse(&raw) {
                Ok(v) => v,
                // Some UE logs have a .json extension but contain multiple independent objects.
                Err(_) if !raw.windows(b"base_assets".len()).any(|w| w == b"base_assets") => continue,
                Err(e) => return Err(format!("存在无法解析的工程引用文件 {p:?}：{e}").into()),
            };
            if let Some(a) = value["base_assets"].as_array() {
                for (i, item) in a.iter().enumerate() {
                    if let Some(id) = item["id"].as_str() {
                        refs.entry(id.into()).or_default().insert((
                            value["id"].as_str().map(String::from),
                            format!("/base_assets/{i}/id"),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn hash_file(path: &Path, buffer: &mut [u8]) -> Result<(u64, String)> {
    let mut file = File::open(path)?;
    let before = file.metadata()?;
    let mut hasher = blake3::Hasher::new();
    loop {
        let n = file.read(buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let after = file.metadata()?;
    ensure(
        !is_reparse(&before) && before.modified()? == after.modified()? && hasher.count() == before.len(),
        || format!("哈希期间文件发生变化：{path:?}"),
    )?;
    Ok((before.len(), hasher.finalize().to_hex().to_string()))
}

#[derive(Clone)]
struct Document {
    path: String,
    before: String,
    updated: Value,
}

fn recheck(root: &Path, lib: &Library, group: &Group) -> Result<BTreeMap<String, Document>> {
    let mut docs = BTreeMap::new();
    let mut group_hashes = None;
    let mut buffer = vec![0; 256 * 1024];
    for asset in &group.assets {
        let id = &asset.id;
        let (dir, v) = lib
            .records
            .get(id)
            .ok_or_else(|| format!("索引中找不到资产 {id}"))?;
        ensure(
            v["id"] == *id && dir == &asset.rel_dir && v == &asset.manifest,
            || format!("ID、目录或 manifest 值与报告不符：{id}（当前 {dir}）"),
        )?;
        naming::check_rel(dir)?;
        let path = format!("{dir}/{MANIFEST}");
        ensure(journal::kind(root, &path)? == Some(Kind::File), || {
            format!("manifest 不是普通文件：{path}")
        })?;
        let raw = fs::read(root.join(&path))?;
        ensure(
            format!("{:x}", Sha256::digest(&raw)) == asset.manifest_sha256
                && asset
                    .manifest_blake3
                    .as_ref()
                    .is_none_or(|expected| digest(&raw) == *expected),
            || format!("manifest 文件摘要与报告不符：{path}"),
        )?;
        let actual = lib.files.get(id).ok_or_else(|| format!("缺少负载：{dir}"))?;
        ensure(asset.payload.len() == actual.len(), || {
            format!("负载文件数量变更：{dir}")
        })?;
        let mut set = BTreeSet::new();
        for file in &asset.payload {
            naming::check_rel(&file.rel)?;
            let rel = format!("{dir}/{}", file.rel);
            ensure(
                actual.get(&file.rel) == Some(&file.size) && journal::kind(root, &rel)? == Some(Kind::File),
                || format!("负载路径或大小变更：{rel}"),
            )?;
            let (size, hash) = hash_file(&root.join(&rel), &mut buffer)?;
            ensure(size == file.size && hash == file.blake3, || {
                format!("负载 BLAKE3 与报告不符：{rel}")
            })?;
            set.insert(hash);
        }
        ensure(
            !set.is_empty() && group_hashes.as_ref().is_none_or(|previous| previous == &set),
            || format!("组内哈希集合不相同：{dir}"),
        )?;
        group_hashes = Some(set);
        if group.review.candidate_archive_ids.contains(id) {
            let known: BTreeSet<_> = asset
                .incoming
                .iter()
                .map(|r| (r.from_id.clone(), r.pointer.clone()))
                .collect();
            for edge in lib.refs.get(id).into_iter().flatten() {
                ensure(known.contains(edge), || {
                    format!("待归档 {id} 出现新的入向引用：{edge:?}")
                })?;
            }
        }
        docs.insert(
            id.clone(),
            Document {
                path: dir.clone(),
                before: String::from_utf8(raw)?,
                updated: v.clone(),
            },
        );
    }
    Ok(docs)
}

fn fields(doc: &mut Value, edits: &[FieldEdit], key_edit: bool) -> Result<()> {
    for e in edits {
        let k = e.pointer.strip_prefix('/').ok_or("字段路径不是 JSON Pointer")?;
        ensure(
            if key_edit {
                k == "key"
            } else {
                matches!(
                    k,
                    "author"
                        | "name"
                        | "name_original"
                        | "note"
                        | "license"
                        | "source"
                        | "preview"
                        | "bundled_effects"
                        | "tested_with"
                        | "parts"
                        | "variants"
                        | "files"
                        | "tags"
                        | "rating"
                        | "series"
                        | "character"
                )
            },
            || format!("该步骤不允许编辑字段：{}", e.pointer),
        )?;
        let current = doc.get(k);
        ensure(
            (e.op == "add" && current.is_none() && e.before.is_null())
                || (e.op == "replace" && current == Some(&e.before)),
            || format!("字段原值与报告不符：{}", e.pointer),
        )?;
        doc[k] = e.value.clone();
    }
    Ok(())
}

fn inspect_tree(root: &Path, from: &str, to: &str) -> Result<()> {
    for entry in fs::read_dir(root.join(from))? {
        let entry = entry?;
        let name = entry.file_name().into_string().map_err(|_| "文件名不是 UTF-8")?;
        let source = format!("{from}/{name}");
        let dest = format!("{to}/{name}");
        let metadata = fs::symlink_metadata(entry.path())?;
        ensure(!is_reparse(&metadata), || format!("资产包含重解析点：{source}"))?;
        ensure(
            naming::utf16_len(&root.join(&dest).to_string_lossy()) <= 240,
            || format!("目标下的路径超过 240 个 UTF-16 单元：{dest}"),
        )?;
        if metadata.is_dir() {
            inspect_tree(root, &source, &dest)?
        } else {
            ensure(metadata.is_file(), || format!("资产包含非常规文件：{source}"))?
        }
    }
    Ok(())
}

fn declared_paths(v: &Value) -> impl Iterator<Item = &str> {
    ["files", "parts", "variants", "audio"]
        .into_iter()
        .flat_map(|k| {
            v[k].as_array()
                .into_iter()
                .flatten()
                .filter_map(|item| item["file"].as_str())
        })
        .chain(
            ["preview", "bundled_effects"]
                .into_iter()
                .flat_map(|k| v[k].as_array().into_iter().flatten().filter_map(Value::as_str)),
        )
        .chain(v["license"]["readme"].as_str())
}

fn targets(v: &Value) -> Vec<&str> {
    let mut out = Vec::new();
    for k in ["song", "derived_from"] {
        out.extend(v[k].as_str())
    }
    for k in ["related", "requires", "tested_with"] {
        out.extend(v[k].as_array().into_iter().flatten().filter_map(Value::as_str));
    }
    for set in v["sets"].as_array().into_iter().flatten() {
        out.extend(
            set["motion"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str),
        );
        for k in ["face", "lip", "camera", "pack"] {
            out.extend(set[k].as_str())
        }
    }
    out
}

pub struct Prepared {
    pub number: u32,
    pub steps: Vec<Step>,
}

fn group_plan(
    root: &Path,
    lib: &Library,
    group: &Group,
    view: &mut View,
    previously_retired: &BTreeSet<String>,
) -> Result<()> {
    ensure(matches!(group.review.category.as_str(), "A" | "C"), || {
        "只接受报告中的 A/C 自动草稿".into()
    })?;
    let mut docs = recheck(root, lib, group)?;
    let keep = group.review.keep_id.as_ref().ok_or("缺少保留 ID")?;
    let expected = group.review.canonical_dir.as_ref().ok_or("缺少保留目录")?;
    let drops: BTreeSet<_> = group.review.candidate_archive_ids.iter().cloned().collect();
    ensure(
        docs.contains_key(keep)
            && !drops.is_empty()
            && !drops.contains(keep)
            && docs.len() == drops.len() + 1
            && drops.iter().all(|id| docs.contains_key(id)),
        || "资产名单、保留 ID 或归档 ID 不一致".into(),
    )?;
    for step in &view.0 {
        if let Step::Write { after, .. } = step {
            let doc = manifest::parse(after.as_bytes())?;
            ensure(!targets(&doc).iter().any(|id| drops.contains(*id)), || {
                "归档会使同一批次中已经预演的引用悬空".into()
            })?;
        }
    }
    let mut retired = BTreeSet::new();
    let mut redirects = BTreeSet::new();
    let mut changed_parents = BTreeMap::new();
    let parents: BTreeSet<_> = docs
        .values()
        .filter_map(|d| d.updated["song"].as_str().map(String::from))
        .collect();
    let mut merged = false;
    let plan: Plan = serde_json::from_value(group.plan.clone())?;
    for (i, action) in plan.actions.iter().enumerate() {
        ensure(action.step == i + 1, || "报告动作序号不连续".into())?;
        match &action.op {
            Operation::Merge {
                scope,
                keep_id,
                retire_ids,
                set_fields,
                redirects: edges,
                target_dir,
            } => {
                ensure(
                    !merged
                        && scope == "asset"
                        && keep_id == keep
                        && target_dir == expected
                        && retire_ids.iter().cloned().collect::<BTreeSet<_>>() == drops,
                    || "merge 的范围或目标与审核结果不一致".into(),
                )?;
                merged = true;
                fields(
                    &mut docs.get_mut(keep).ok_or("缺少保留方")?.updated,
                    set_fields,
                    false,
                )?;
                for edge in edges {
                    ensure(
                        edge.after == *keep
                            && drops.contains(&edge.before)
                            && parents.contains(&edge.from_id)
                            && (edge.pointer.starts_with("/sets/") || edge.pointer.starts_with("/related/")),
                        || format!("引用修改超出获准的父歌范围：{} {}", edge.from_id, edge.pointer),
                    )?;
                    let parent = if let Some(parent) = changed_parents.get_mut(&edge.from_id) {
                        parent
                    } else {
                        let (dir, v) = lib.records.get(&edge.from_id).ok_or("父歌不存在")?;
                        let path = format!("{dir}/{MANIFEST}");
                        let raw = String::from_utf8(view.bytes(root, &path)?)?;
                        ensure(
                            v["type"] == "song" && manifest::parse(raw.as_bytes())? == *v,
                            || format!("父歌 manifest 已变更：{dir}"),
                        )?;
                        changed_parents.entry(edge.from_id.clone()).or_insert(Document {
                            path: dir.clone(),
                            before: raw,
                            updated: v.clone(),
                        })
                    };
                    let value = parent
                        .updated
                        .pointer_mut(&edge.pointer)
                        .ok_or("父歌没有待更新的引用")?;
                    ensure(*value == edge.before, || {
                        format!("父歌引用原值变化：{}", edge.pointer)
                    })?;
                    *value = Value::String(edge.after.clone());
                    redirects.insert((
                        edge.before.clone(),
                        Some(edge.from_id.clone()),
                        edge.pointer.clone(),
                    ));
                }
            }
            Operation::Quarantine { id, from_dir } => {
                ensure(merged && drops.contains(id) && retired.insert(id.clone()), || {
                    format!("无效归档 ID：{id}")
                })?;
                let d = &docs[id];
                ensure(d.path == *from_dir, || format!("归档源不符：{from_dir}"))?;
                let inside = from_dir
                    .strip_prefix("Library/")
                    .ok_or("归档来源不在 Library 下")?;
                let to = format!("{ARCHIVE}/{inside}");
                inspect_tree(root, from_dir, &to)?;
                view.parents(root, &to)?;
                view.add(
                    root,
                    Step::Move {
                        from: from_dir.clone(),
                        to,
                        manifest_blake3: digest(d.before.as_bytes()),
                    },
                )?;
            }
            Operation::Rekey {
                id,
                from_dir,
                to_dir,
                fields: edits,
            } => {
                ensure(merged && id == keep, || "rekey 只允许修改保留方".into())?;
                let d = docs.get_mut(id).ok_or("缺少 rekey 资产")?;
                ensure(d.path == *from_dir, || format!("rekey 来源不符：{from_dir}"))?;
                fields(&mut d.updated, edits, true)?;
                ensure(layout::destination(&d.updated, &lib.songs)? == *to_dir, || {
                    format!("rekey 路径与 layout 不符：{to_dir}")
                })?;
                if from_dir != to_dir {
                    inspect_tree(root, from_dir, to_dir)?;
                    let parent = to_dir.rsplit_once('/').ok_or("不合法的目标目录")?.0;
                    if let Some(other) = lib.dirs.get(&UniCase::new(parent.into())) {
                        ensure(other == parent, || {
                            format!("目标父目录大小写冲突：{parent} / {other}")
                        })?;
                    }
                    view.parents(root, to_dir)?;
                    view.add(
                        root,
                        Step::Move {
                            from: from_dir.clone(),
                            to: to_dir.clone(),
                            manifest_blake3: digest(d.before.as_bytes()),
                        },
                    )?;
                    d.path = to_dir.clone();
                }
            }
        }
    }
    ensure(merged && retired == drops, || "操作未完整覆盖计划退役资产".into())?;
    for id in &drops {
        for (from, ptr) in lib.refs.get(id).into_iter().flatten() {
            if from.as_ref().is_some_and(|id| drops.contains(id)) {
                continue;
            }
            ensure(
                redirects.contains(&(id.clone(), from.clone(), ptr.clone())),
                || format!("归档会留下悬空引用：{from:?} {ptr} → {id}"),
            )?;
        }
    }
    let mut d = docs.remove(keep).ok_or("缺少保留方")?;
    ensure(
        d.path == *expected && layout::destination(&d.updated, &lib.songs)? == *expected,
        || format!("最终目录与 layout 不一致：{expected}"),
    )?;
    for asset in &group.assets {
        let parent = asset.rel_dir.rsplit_once('/').ok_or("缺少原父目录")?.0;
        ensure(!view.is_empty(root, parent)?, || {
            format!("迁移会留下空的原父目录：{parent}")
        })?;
    }
    let note = d.updated["note"].as_str().unwrap_or_default();
    let line = format!(
        "{DATE} 合并同内容资产 {}",
        drops.iter().cloned().collect::<Vec<_>>().join("、")
    );
    d.updated["note"] = Value::String(if note.is_empty() {
        line
    } else {
        format!("{note}\n{line}")
    });
    changed_parents.insert(keep.clone(), d);
    for (_, mut doc) in changed_parents {
        doc.updated["updated"] = Value::from(DATE);
        let issues = lib.schemas.validate_asset(&doc.updated, 8)?;
        ensure(issues.is_empty(), || {
            format!("schema 校验失败：{} {issues:?}", doc.path)
        })?;
        for id in targets(&doc.updated) {
            ensure(
                lib.records.contains_key(id) && !drops.contains(id) && !previously_retired.contains(id),
                || format!("修改产生悬空引用：{} → {id}", doc.path),
            )?;
        }
        for file in declared_paths(&doc.updated) {
            let path = format!("{}/{}", doc.path, file.replace('\\', "/"));
            naming::check_rel(&path)?;
            ensure(view.kind(root, &path)? == Some(Kind::File), || {
                format!("修改后的声明文件缺失：{path}")
            })?;
        }
        let fmt = manifest::DocumentFormat::detect(doc.before.as_bytes());
        let data = manifest::encode(&doc.updated, fmt)?;
        ensure(!data.normalized_newlines, || {
            format!("混合换行需人工确认：{}", doc.path)
        })?;
        let after = String::from_utf8(data.bytes)?;
        view.add(
            root,
            Step::edit(format!("{}/{MANIFEST}", doc.path), doc.before, after),
        )?;
    }
    Ok(())
}

fn check_completed(root: &Path, lib: &Library, group: &Group) -> Result<()> {
    let current = group.review.keep_id.as_ref().and_then(|id| lib.records.get(id));
    let expected = group.review.canonical_dir.as_ref();
    ensure(
        current.map(|(dir, _)| dir) == expected && expected.is_some(),
        || format!("第 {:02} 组日志已完成，但当前保留目录与报告不同", group.group),
    )?;
    for asset in &group.assets {
        if !group.review.candidate_archive_ids.contains(&asset.id) {
            continue;
        }
        let inside = asset
            .rel_dir
            .strip_prefix("Library/")
            .ok_or("归档来源不在 Library 下")?;
        let path = format!("{ARCHIVE}/{inside}/{MANIFEST}");
        naming::check_rel(&path)?;
        ensure(journal::kind(root, &path)? == Some(Kind::File), || {
            format!("已归档 manifest 缺失：{path}")
        })?;
        let raw = fs::read(root.join(&path))?;
        ensure(
            format!("{:x}", Sha256::digest(&raw)) == asset.manifest_sha256
                && !lib.records.contains_key(&asset.id),
            || format!("第 {:02} 组归档与报告不同：{path}", group.group),
        )?;
    }
    Ok(())
}

pub fn prepare(root: &Path, groups: Vec<&Group>, applied: &BTreeSet<u32>) -> Result<Vec<Prepared>> {
    let lib = Library::read(root)?;
    println!(
        "重新索引 {} 份资产；复核使用磁盘 BLAKE3 及报告 manifest 摘要",
        lib.records.len()
    );
    let mut view = View::default();
    let mut prepared = Vec::new();
    let mut skipped = 0;
    let mut previous = 0;
    let mut retired = BTreeSet::new();
    for group in groups {
        if applied.contains(&group.group) {
            check_completed(root, &lib, group)?;
            previous += 1;
            continue;
        }
        let start = view.0.len();
        match group_plan(root, &lib, group, &mut view, &retired) {
            Ok(()) => {
                retired.extend(group.review.candidate_archive_ids.iter().cloned());
                prepared.push(Prepared {
                    number: group.group,
                    steps: view.0[start..].to_vec(),
                });
            }
            Err(e) => {
                view.0.truncate(start);
                skipped += 1;
                println!("跳过组 {:02}：{e}", group.group);
            }
        }
    }
    let moves = view.0.iter().filter(|s| matches!(s, Step::Move { .. })).count();
    let edits = view.0.iter().filter(|s| matches!(s, Step::Write { .. })).count();
    let mkdirs = view.0.iter().filter(|s| matches!(s, Step::Mkdir { .. })).count();
    println!(
        "预检：待执行 {} 组，移动 {moves}，编辑 {edits}，建目录 {mkdirs}，跳过 {skipped}，已执行 {previous}",
        prepared.len()
    );
    println!(
        "待执行组：{}",
        prepared
            .iter()
            .map(|g| format!("{:02}", g.number))
            .collect::<Vec<_>>()
            .join(",")
    );
    Ok(prepared)
}
