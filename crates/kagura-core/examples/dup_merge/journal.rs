use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Write},
    path::Path,
};

use kagura_core::{consts::MANIFEST, naming, scan::is_reparse};
use serde::{Deserialize, Serialize};
use unicase::UniCase;

use crate::{ensure, plan::Prepared, Result, ARCHIVE, RUN};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Dir,
    File,
}

pub fn kind(root: &Path, rel: &str) -> Result<Option<Kind>> {
    let mut p = root.to_owned();
    for component in rel.split('/') {
        p.push(component);
        match fs::symlink_metadata(&p) {
            Ok(meta) if is_reparse(&meta) => return Err(format!("重解析点：{p:?}").into()),
            Ok(meta) if meta.is_dir() => (),
            Ok(meta) if meta.is_file() && p == root.join(rel) => return Ok(Some(Kind::File)),
            Ok(_) => return Err(format!("非预期路径类型：{p:?}").into()),
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(Some(Kind::Dir))
}

fn same(a: &str, b: &str) -> bool {
    UniCase::new(a) == UniCase::new(b)
}

fn tail<'a>(path: &'a str, base: &str) -> Option<&'a str> {
    let mut rest = path;
    for part in base.split('/') {
        let (first, next) = rest.split_once('/').unwrap_or((rest, ""));
        if !same(first, part) {
            return None;
        }
        rest = next;
    }
    Some(rest)
}

fn join(a: &str, b: &str) -> String {
    if b.is_empty() {
        a.to_owned()
    } else {
        format!("{a}/{b}")
    }
}

fn parent(path: &str) -> &str {
    path.rsplit_once('/').map(|(p, _)| p).unwrap_or("")
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum Step {
    Mkdir {
        path: String,
    },
    Rmdir {
        path: String,
    },
    Move {
        from: String,
        to: String,
        manifest_blake3: String,
    },
    Write {
        path: String,
        before: String,
        after: String,
        old_blake3: String,
        new_blake3: String,
    },
}

impl Step {
    pub fn edit(path: String, before: String, after: String) -> Self {
        Self::Write {
            path,
            old_blake3: digest(before.as_bytes()),
            new_blake3: digest(after.as_bytes()),
            before,
            after,
        }
    }

    fn reversed(&self) -> Self {
        match self {
            Self::Mkdir { path } => Self::Rmdir { path: path.clone() },
            Self::Rmdir { path } => Self::Mkdir { path: path.clone() },
            Self::Move {
                from,
                to,
                manifest_blake3,
            } => Self::Move {
                from: to.clone(),
                to: from.clone(),
                manifest_blake3: manifest_blake3.clone(),
            },
            Self::Write {
                path, before, after, ..
            } => Self::edit(path.clone(), after.clone(), before.clone()),
        }
    }

    fn paths(&self) -> Vec<&str> {
        match self {
            Self::Move { from, to, .. } => vec![from, to],
            Self::Mkdir { path } | Self::Rmdir { path } | Self::Write { path, .. } => vec![path],
        }
    }

    fn validate_paths(&self, root: &Path) -> Result<()> {
        for path in self.paths() {
            naming::check_rel(path)?;
            let library = path.starts_with("Library/");
            let archive = path == ARCHIVE || path.starts_with(&format!("{ARCHIVE}/"));
            ensure(library || archive, || format!("路径超出迁移范围：{path}"))?;
            ensure(
                naming::utf16_len(&root.join(path).to_string_lossy()) <= 240,
                || format!("路径超过 240 个 UTF-16 单元：{path}"),
            )?;
        }
        if let Self::Write {
            path,
            old_blake3,
            new_blake3,
            before,
            after,
        } = self
        {
            ensure(
                path.starts_with("Library/")
                    && path.ends_with(&format!("/{MANIFEST}"))
                    && *old_blake3 == digest(before.as_bytes())
                    && *new_blake3 == digest(after.as_bytes()),
                || format!("日志中的 manifest 路径或摘要不符：{path}"),
            )?;
        }
        Ok(())
    }

    // True when the step still needs to run. False is accepted only for recovery of a pending intent.
    fn needed(&self, root: &Path, view: &View, allow_done: bool) -> Result<bool> {
        self.validate_paths(root)?;
        match self {
            Self::Mkdir { path } => match view.kind(root, path)? {
                None => {
                    let p = parent(path);
                    ensure(view.kind(root, p)? == Some(Kind::Dir), || {
                        format!("缺少父目录：{p}")
                    })?;
                }
                Some(Kind::Dir) if allow_done => return Ok(false),
                _ => return Err(format!("目录已存在：{path}").into()),
            },
            Self::Rmdir { path } => match view.kind(root, path)? {
                Some(Kind::Dir) => ensure(view.is_empty(root, path)?, || format!("撤回目录非空：{path}"))?,
                None if allow_done => return Ok(false),
                _ => return Err(format!("撤回目录不存在：{path}").into()),
            },
            Self::Move {
                from,
                to,
                manifest_blake3,
            } => {
                let src = view.kind(root, from)?;
                let dst = view.kind(root, to)?;
                if allow_done && src.is_none() && dst == Some(Kind::Dir) {
                    ensure(
                        digest(&view.bytes(root, &join(to, MANIFEST))?) == *manifest_blake3,
                        || format!("已移动目录的 manifest 已变更：{to}"),
                    )?;
                    return Ok(false);
                }
                ensure(src == Some(Kind::Dir) && dst.is_none(), || {
                    format!("移动源缺失或目标已占用：{from} → {to}")
                })?;
                ensure(view.kind(root, parent(to))? == Some(Kind::Dir), || {
                    format!("目标父目录缺失：{to}")
                })?;
                ensure(
                    digest(&view.bytes(root, &join(from, MANIFEST))?) == *manifest_blake3,
                    || format!("移动源 manifest 已变更：{from}"),
                )?;
            }
            Self::Write {
                path, before, after, ..
            } => {
                let bytes = view.bytes(root, path)?;
                if allow_done && bytes == after.as_bytes() {
                    return Ok(false);
                }
                ensure(bytes == before.as_bytes(), || {
                    format!("manifest 字节已变更：{path}")
                })?;
            }
        }
        Ok(true)
    }

    fn run(&self, root: &Path) -> Result<()> {
        self.needed(root, &View::default(), false)?;
        match self {
            Self::Mkdir { path } => fs::create_dir(root.join(path))?,
            Self::Rmdir { path } => fs::remove_dir(root.join(path))?,
            Self::Move { from, to, .. } => fs::rename(root.join(from), root.join(to))?,
            Self::Write { path, after, .. } => {
                let path = root.join(path);
                let mut file = tempfile::NamedTempFile::new_in(path.parent().ok_or("缺少父目录")?)?;
                file.write_all(after.as_bytes())?;
                file.as_file().sync_all()?;
                file.persist(path)?.sync_all()?;
            }
        }
        Ok(())
    }
}

pub fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

#[derive(Default)]
pub struct View(pub Vec<Step>);

enum Origin {
    Disk(String),
    Bytes(Vec<u8>),
    EmptyDir,
    Missing,
}

impl View {
    fn origin(&self, path: &str) -> Origin {
        let mut name = path.to_owned();
        for step in self.0.iter().rev() {
            match step {
                Step::Move { from, to, .. } => {
                    if let Some(rel) = tail(&name, to) {
                        name = join(from, rel);
                    } else if tail(&name, from).is_some() {
                        return Origin::Missing;
                    }
                }
                Step::Write { path, after, .. } if same(path, &name) => {
                    return Origin::Bytes(after.as_bytes().to_vec());
                }
                Step::Mkdir { path } if tail(&name, path).is_some() => {
                    return if same(path, &name) {
                        Origin::EmptyDir
                    } else {
                        Origin::Missing
                    };
                }
                Step::Rmdir { path } if tail(&name, path).is_some() => return Origin::Missing,
                _ => (),
            }
        }
        Origin::Disk(name)
    }

    pub fn kind(&self, root: &Path, path: &str) -> Result<Option<Kind>> {
        match self.origin(path) {
            Origin::Disk(path) => kind(root, &path),
            Origin::Bytes(_) => Ok(Some(Kind::File)),
            Origin::EmptyDir => Ok(Some(Kind::Dir)),
            Origin::Missing => Ok(None),
        }
    }

    pub fn bytes(&self, root: &Path, path: &str) -> Result<Vec<u8>> {
        match self.origin(path) {
            Origin::Disk(path) if kind(root, &path)? == Some(Kind::File) => Ok(fs::read(root.join(path))?),
            Origin::Bytes(data) => Ok(data),
            _ => Err(format!("文件不存在：{path}").into()),
        }
    }

    pub fn is_empty(&self, root: &Path, path: &str) -> Result<bool> {
        if let Origin::Disk(origin) = self.origin(path) {
            for entry in fs::read_dir(root.join(origin))? {
                let name = entry?
                    .file_name()
                    .into_string()
                    .map_err(|_| "目录名称不是 UTF-8")?;
                if self.kind(root, &join(path, &name))?.is_some() {
                    return Ok(false);
                }
            }
        }
        for step in &self.0 {
            for candidate in step.paths() {
                if same(parent(candidate), path) && self.kind(root, candidate)?.is_some() {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    pub fn add(&mut self, root: &Path, step: Step) -> Result<()> {
        step.needed(root, self, false)?;
        self.0.push(step);
        Ok(())
    }

    pub fn parents(&mut self, root: &Path, path: &str) -> Result<()> {
        let p = parent(path);
        if self.kind(root, p)?.is_none() {
            self.parents(root, p)?;
            self.add(root, Step::Mkdir { path: p.into() })?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
enum Event {
    Begin { group: u32 },
    Intent { step: Step },
    Commit { group: u32 },
    Reverted { intent: usize },
}

pub fn check_run_paths(root: &Path) -> Result<()> {
    for path in [ARCHIVE, RUN, &format!("{RUN}/log.jsonl"), &format!("{RUN}/lock")] {
        kind(root, path)?;
    }
    Ok(())
}

pub struct Journal {
    events: Vec<Event>,
    log: Option<File>,
    _lock: Option<File>,
}

impl Journal {
    pub fn read(root: &Path) -> Result<Self> {
        let path = root.join(RUN).join("log.jsonl");
        let events = match fs::read_to_string(&path) {
            Ok(text) => {
                ensure(text.is_empty() || text.ends_with('\n'), || {
                    "日志最后一行不完整；先核对该行再继续".into()
                })?;
                text.lines()
                    .map(serde_json::from_str)
                    .collect::<std::result::Result<Vec<_>, _>>()?
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e.into()),
        };
        Ok(Self {
            events,
            log: None,
            _lock: None,
        })
    }

    pub fn open(root: &Path) -> Result<Self> {
        fs::create_dir_all(root.join(RUN))?;
        let lock = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(root.join(RUN).join("lock"))?;
        lock.try_lock()
            .map_err(|e| format!("另一个合并或撤回进程正在运行：{e}"))?;
        let mut journal = Self::read(root)?;
        journal.log = Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(root.join(RUN).join("log.jsonl"))?,
        );
        journal._lock = Some(lock);
        Ok(journal)
    }

    fn append(&mut self, event: Event) -> Result<()> {
        let log = self.log.as_mut().ok_or("日志未打开")?;
        let mut line = serde_json::to_vec(&event)?;
        line.push(b'\n');
        log.write_all(&line)?;
        log.sync_all()?;
        self.events.push(event);
        Ok(())
    }

    fn reverted(&self) -> BTreeSet<usize> {
        self.events
            .iter()
            .filter_map(|e| match e {
                Event::Reverted { intent } => Some(*intent),
                _ => None,
            })
            .collect()
    }

    pub fn completed(&self) -> Result<BTreeSet<u32>> {
        let reverted = self.reverted();
        let mut result = BTreeSet::new();
        let mut intents = Vec::new();
        let mut committed = false;
        for (i, event) in self.events.iter().enumerate() {
            match event {
                Event::Begin { .. } => {
                    ensure(committed || intents.iter().all(|i| reverted.contains(i)), || {
                        "日志有未完成的组，先用 --undo 恢复".into()
                    })?;
                    intents.clear();
                    committed = false;
                }
                Event::Intent { .. } => intents.push(i),
                Event::Commit { group } => {
                    committed = true;
                    let restored = intents.iter().filter(|i| reverted.contains(i)).count();
                    ensure(restored == 0 || restored == intents.len(), || {
                        "撤回尚未结束，先用 --undo 恢复".into()
                    })?;
                    if restored == 0 {
                        result.insert(*group);
                    }
                }
                Event::Reverted { .. } => (),
            }
        }
        ensure(committed || intents.iter().all(|i| reverted.contains(i)), || {
            "日志有未完成的组，先用 --undo 恢复".into()
        })?;
        Ok(result)
    }

    pub fn apply(&mut self, root: &Path, group: Prepared) -> Result<()> {
        self.append(Event::Begin { group: group.number })?;
        for step in group.steps {
            self.append(Event::Intent { step: step.clone() })?;
            step.run(root)
                .map_err(|e| format!("第 {:02} 组中断，按日志运行 --undo：{e}", group.number))?;
        }
        self.append(Event::Commit { group: group.number })?;
        println!("已完成组 {:02}", group.number);
        Ok(())
    }

    pub fn undo(&mut self, root: &Path, dry_run: bool) -> Result<()> {
        let mut view = View::default();
        let mut reverse = Vec::new();
        let reverted = self.reverted();
        // The latest unapplied intent may have been interrupted either before or after its operation.
        // In either case the original paths and manifest must agree with the recorded state.
        for (i, event) in self.events.iter().enumerate().rev() {
            if let Event::Intent { step } = event {
                if reverted.contains(&i) {
                    continue;
                }
                let step = step.reversed();
                let needed = step.needed(root, &view, true)?;
                if needed {
                    view.0.push(step.clone());
                }
                reverse.push((i, needed.then_some(step)));
            }
        }
        println!(
            "撤回预检通过：{} 步（移动 {}，写回 {}），已撤回或未落地 {} 步",
            view.0.len(),
            view.0.iter().filter(|a| matches!(a, Step::Move { .. })).count(),
            view.0.iter().filter(|a| matches!(a, Step::Write { .. })).count(),
            reverse.len() - view.0.len()
        );
        if !dry_run {
            for (intent, step) in reverse {
                if let Some(step) = step {
                    step.run(root)?;
                }
                self.append(Event::Reverted { intent })?;
            }
        }
        Ok(())
    }
}
