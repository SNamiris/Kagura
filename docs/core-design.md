# Kagura 核心模块设计（Rust）

> v0.1 · 2026-11-24 · 对应 PLAN.md 的 M3 / M4
> 参考实现：`G:\MMD\_design\migration\plan.py`、`scan.py`、`transaction.py`

---

## 1. 原则

- **核心库与界面分开**：`kagura-core` 不依赖 Tauri。Tauri 应用和以后的 MCP 服务都调用同一个核心库，规则只写一份。
- **行为与 Python 版一致**：命名、路径、识别规则以 `plan.py` 为准。用 Python 版从真实库导出「标准答案」，Rust 版逐条对照（见第 5 节）。
- **只读优先**：M3 只做读取、校验、索引。写操作（修改 manifest、移动文件夹）放到最后，并且一律走「计划 → 预检 → 确认 → 执行 → 日志」。
- **文件为准，索引可丢**：SQLite 只是缓存，删掉后重新扫描就能恢复。

---

## 2. 仓库结构

```
E:\Repos\kagura\
├── Cargo.toml                 (workspace)
├── crates\
│   ├── kagura-core\           核心库（本文档的主体）
│   └── kagura-mcp\            MCP 服务（M5 起，只依赖 core）
├── src-tauri\                 Tauri 应用：只做命令转发和窗口
├── ui\                        前端
├── schemas\                   JSON Schema（从 _design\schemas 复制，编译时嵌入）
├── tests\
│   ├── golden\                Python 版导出的标准答案
│   └── fixtures\              手工构造的小型测试库
├── tools\export_golden.py     导出标准答案的脚本
└── docs\
```

---

## 3. kagura-core 模块

| 模块 | 职责 | Python 对应 |
|---|---|---|
| `consts` | `MANIFEST`、`MANIFEST_ALT`、`FORMAT`、`SONG_RESERVED`、长度上限 | plan.py 顶部常量 |
| `naming` | `safe()`、`utf16_len()`、相对路径检查、Windows 保留名 | `safe` `length` `rel` `DISALLOWED` `RESERVED` |
| `layout` | 由 key 算目录：`destination()`、`song_dir()`、保留目录判断、目标路径检查 | `destination` `song_path` `is_song_reserved` `check_destination` |
| `manifest` | 读写 manifest、识别是不是我们的记录、类型定义 | `load` `dumps` `metadata_role` |
| `schema` | 嵌入 schema，校验 manifest | `validators` `validate` |
| `scan` | 遍历 Library，产出记录、跳过项和错误 | `iter_library_metadata` `known_songs` `scan.reparse` |
| `check` | 全库一致性检查（路径、引用、文件存在、ID 重复） | 这几轮复核用的脚本 |
| `index` | SQLite 索引 + FTS5 全文搜索 | `migrate.py index`（简化版） |
| `hash` | BLAKE3 内容哈希、去重分组 | `scan.hashfile`（SHA-256） |
| `ids` | ULID 生成与格式检查 | `ulid` |
| `txn` | 写操作：计划、虚拟预检、执行、日志、撤回（M4 以后） | `transaction.py`、`dedup.py` |

### 3.1 `naming`

```rust
pub fn utf16_len(s: &str) -> usize;
pub fn safe(name: &str, limit: usize) -> Result<String, NamingError>;
pub fn check_rel(path: &str) -> Result<(), NamingError>;   // 对应 rel()
pub fn is_reserved_device(segment: &str) -> bool;           // CON / PRN / COM1 …
```

`safe()` 必须**逐步复刻** Python 的行为，包括它的小毛病，否则文件夹名会对不上：

1. 把 `< > : " / \ | ? *` 和 `\x00-\x1f` 换成 `_`
2. 两端去掉空格和点（`strip(' .')`）
3. 按 **UTF-16 长度**截断：每次去掉最后一个字符（码点），直到长度 ≤ limit。日文的扩展汉字、emoji 一个字符占 2 个单位
4. 再去掉末尾的空格和点
5. 如果第一个 `.` 之前的部分（去掉末尾空格）是 Windows 保留名：在前面加 `_`，然后取 `name[:limit-1]`。注意这里是**按码点**截断，不是按 UTF-16，这是原实现的行为，照抄
6. 结果为空就报错

长度上限：名称 40，歌曲版本 `edition` 24，动作后缀 `suffix` 24。

`check_rel()`：每一段不能是空、`.`、`..`，不能含非法字符，不能以空格或点结尾，不能是保留名，不能超过 255 个 UTF-16 单位。

### 3.2 `layout`

```rust
pub fn song_dir(key: &SongKey) -> Result<RelPath>;
pub fn destination(asset: &AssetView, songs: &SongMap) -> Result<RelPath>;
pub fn is_song_reserved(rel: &RelPath) -> bool;
pub fn check_target(root: &Path, target: &RelPath, extra: &[RelPath]) -> Result<()>;
```

| 类型 | 目录 |
|---|---|
| model | `Library/Models/{Characters\|Props}/{safe(author)}/{safe(name)}` |
| stage | `Library/Stages/{safe(author)}/{safe(name)}`（天空球、HDRI 也一样） |
| song | `Library/Motions/{safe(title)}[({safe(edition,24)})]_{safe(composer)}` |
| motion_part | `{歌目录或 Library/Motions/_General}/{part}_{safe(author)}[_{safe(suffix,24)}]` |

- `part` 直接拼接，不经过 `safe()`（取值是固定的枚举）。
- 保留目录只在 `Library/Motions/<歌>/<名字>` 这一层生效，名字比较不区分大小写。
- `check_target`：整条路径（含根目录）不超过 240 个 UTF-16 单位；同一批目标里不能有只差大小写的路径；不能和保留目录同名。
- 路径比较：Windows 不区分大小写。程序内部统一用 `/` 分隔的相对路径，只在调用系统接口时转换。

### 3.3 `manifest`

- **读取**：允许 UTF-8 BOM（Python 用 `utf-8-sig` 读）。
- **写入**：要和 Python 输出的字节一致，避免无意义的改动：缩进 2 空格、不转义非 ASCII 字符、末尾换行、**字段顺序保持原样**（`serde_json` 开 `preserve_order`）。
- **存储方式**：以 `serde_json::Value` 为准，保证读写往返不丢信息；另外提供类型化的只读视图（`ModelView`、`SongView` 等）给业务逻辑用。
- **识别规则**（按顺序）：
  1. 文件名是 `_asset.manifest.json`
  2. 像资产记录：有 `schema` 或 `classified_by`，或 `id` 形如 ULID，或 `type` 是已知类型且带 `id` / `key`，或同时有 `key` 和 `variants`
  3. 像资产但 `format` 不是 `kagura-asset-manifest` → 标为 `ForeignManifest`，跳过并报告，不覆盖
  4. 不像资产：`name` 以 `char:` / `stage:` 开头的 → `RuntimeConfig`；其他 → `SourceJson`。两种都跳过
  5. 像资产且 format 正确 → 交给 schema 校验，**校验失败要报错，不能静默跳过**
- **和 Python 版的一处差别**：Python 的 `validate()` 会顺手往记录里补 `format` 字段（修改了数据）。Rust 版的校验不修改任何数据，补字段只在生成新记录时做。

### 3.4 `schema`

- 用 `include_str!` 把 8 个 schema 编译进程序，构建一个 `$id` → schema 的注册表，校验时能解析跨文件的 `$ref`（`common.schema.json#/$defs/...`）。
- 按 `type` 选择 schema：`model` / `stage` / `song` / `motion_part`。`project`、`game`、`aliases` 也一起注册。
- 错误信息返回 JSON 路径加说明，最多返回前 8 条（和 Python 一致）。
- 要开启 `format` 校验（`date`、`uri`）。

### 3.5 `scan`

```rust
pub struct ScanReport {
    pub records: Vec<Record>,        // 路径 + Value + 类型化视图
    pub skipped: Vec<Skipped>,       // ForeignManifest / RuntimeConfig / SourceJson / Reserved
    pub errors: Vec<ScanError>,      // schema 失败、重解析点、读取失败
}
pub fn scan_library(root: &Path) -> ScanReport;
pub fn song_map(records: &[Record]) -> Result<SongMap>;   // 对应 known_songs()
```

- 遍历时跳过**重解析点**（符号链接、目录联接）：检查 Windows 文件属性 `FILE_ATTRIBUTE_REPARSE_POINT`（0x400）。
- 跳过歌曲下的保留目录。
- **比 Python 版更严格的一点**：Python 版会遍历整个 Library，任何深度的 `_asset.manifest.json` 都会读。Rust 版只在**预期的资产根目录深度**找记录：
  - Models、Stages：`<分类>/<作者>/<名称>/`
  - Motions：`<歌>/`（歌）和 `<歌>/<素材>/`（素材）
  - 其他深度出现的同名文件放进 `skipped`，标为 `UnexpectedDepth`。
  - 现在的库按 Python 版扫描结果为 8,824 份，Rust 版应该得到同样的数字。如果不一致，说明有记录放错了层级，需要排查。
- `song_map`：歌曲 ID 不能重复，歌目录必须和 `song_dir(key)` 完全一致，否则报错。

### 3.6 `check`

把这几轮复核做成固定的检查，输出结构化报告：

| 检查 | 说明 |
|---|---|
| `id_unique` | ULID 不重复 |
| `path_matches_key` | 记录所在目录等于 `destination()` 的结果 |
| `refs_resolve` | `song`、`related`、`sets`、`requires`、`derived_from`、`tested_with` 都能找到对应的资产 |
| `files_exist` | `variants`、`parts`、`files`、`audio` 里列出的文件都存在 |
| `no_orphan_dirs` | 资产目录和素材目录都有 manifest；没有空目录 |
| `author_case` | 同一分类下没有只差大小写的作者目录 |
| `path_length` | 没有超过 240 的路径 |

### 3.7 `index`

- SQLite（`rusqlite`，打开 `bundled` 特性，自带 FTS5）。
- 表：`assets(id, type, kind, path, author, name, title, composer, rating, added, updated, mtime)`、`refs(from_id, to_id, field)`、`tags(id, tag)`、`files(asset_id, rel, size, blake3)`。
- FTS5 表对 `name`、`name_original`、`author`、`character`、`series`、`title`、`title_alias`、`tags`、`note` 做全文索引。日文、中文用 `trigram` 分词器，不需要额外的分词库。
- **增量更新**：按 manifest 的修改时间判断要不要重读；文件哈希按「大小 + 修改时间」缓存，变了才重算。
- 索引文件放在 `%APPDATA%\Kagura\index.sqlite`，不写进资产库。

### 3.8 `hash`

- 新代码统一用 BLAKE3。
- 迁移日志里记的是 SHA-256。要校验旧日志时，单独提供一个 SHA-256 函数，只用于这个场景。

### 3.9 `txn`（M4 以后）

参照这次迁移的做法：

1. **计划**：生成操作列表（`Move` / `EditManifest` / `RemoveEmptyDir`），不碰磁盘。
2. **预检**：在虚拟文件系统上依次模拟每一步（同 `dedup.py`），检查来源存在、目标不存在、schema 通过、引用不会悬空、路径合法。有任何问题就整批拒绝。
3. **执行**：移动前确认文件标识（卷序列号 + 文件 ID）没变；每完成一步立即写一行 JSONL 日志；编辑操作在日志里保存原内容。
4. **撤回**：按日志倒序执行。

MCP 的写操作也走这一套，agent 只能提交计划，执行需要用户确认。

---

## 4. 依赖（待确认版本）

| 用途 | crate |
|---|---|
| JSON | `serde`、`serde_json`（`preserve_order`） |
| Schema 校验 | `jsonschema`（需确认支持 2020-12 和跨文件 `$ref`） |
| SQLite | `rusqlite`（`bundled`） |
| 哈希 | `blake3`、`sha2` |
| ULID | `ulid` |
| 遍历 | `walkdir` |
| 错误 | `thiserror` |
| MCP | 官方 Rust SDK `rmcp`（到 M5 再确认） |

以上 crate 的版本和特性在建仓库时再核对，这里只是选型方向。

---

## 5. 对照测试

用 Python 版对真实库导出标准答案，Rust 版逐条比对。

| 文件 | 内容 | 来源 |
|---|---|---|
| `golden/safe.jsonl` | 库里出现过的所有作者、名称、歌名、作曲者、版本、后缀 → `safe()` 的结果 | `plan.safe` |
| `golden/destination.jsonl` | 8,824 份记录的 id → 期望目录 | `plan.destination` + `known_songs` |
| `golden/scan_summary.json` | 记录数（按类型）、跳过项清单 | `plan.iter_library_metadata` |
| `golden/schema_cases.jsonl` | 若干合法和非法记录 → 是否通过 | `plan.validate` |
| `fixtures/edge_names.json` | 手写边界用例：保留名（`CON`、`com1.txt`）、末尾点和空格、全角空格、UTF-16 截断边界（扩展汉字、emoji）、全是非法字符 | 手写 |

- 标准答案里只存路径和 ID，不存文件内容，可以放进仓库。
- 另外做一个小型测试库 `fixtures/mini_library/`，每类资产几份，包括外来 `info.json`、同名但 format 不对的 manifest、保留目录、重解析点（测试时临时创建），给 CI 用。

---

## 6. M3 实施顺序

| 步骤 | 内容 | 验收 |
|---|---|---|
| 1 | 建 workspace、复制 schema、写 `export_golden.py` 导出标准答案 | 标准答案文件生成 |
| 2 | `consts` + `naming` | `safe.jsonl` 和边界用例全部一致 |
| 3 | `manifest` + `schema` | 8,824 份全部通过校验；读后写回与原文件字节一致 |
| 4 | `layout` + `scan` + `song_map` | `destination.jsonl`、`scan_summary.json` 全部一致 |
| 5 | `check` | 对真实库输出零问题；对测试库能报出每一种预设问题 |
| 6 | `index` | 全库建索引的时间、增量更新、全文搜索日文和中文 |
| 7 | Tauri 命令：扫描、列表、筛选、详情、打开文件夹 | 界面能浏览全库 |
| 8 | 编辑 manifest（标签、评分、简评），带校验和日志 | 改动可撤回 |

第 1～5 步是纯库代码，不需要界面，可以先做。

---

## 7. 待确认

- `jsonschema` crate 对 2020-12 `unevaluatedProperties` 和跨文件 `$ref` 的支持程度，不够的话改用别的校验库或者把 schema 合并成一个文件。
- Python `casefold()` 和 Rust `to_lowercase()` 在少数字符上结果不同（比如德语 ß）。路径比较的大小写规则以 Windows 的为准，需要对照测试确认。
- ~~读取时是否要求 manifest 位于预期深度~~：已确认（2026-11-24）。真实库 8,824 份记录全部在预期深度，采用严格规则。
