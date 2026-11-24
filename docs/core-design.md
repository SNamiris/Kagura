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

- **读取**时记录 UTF-8 BOM 和换行风格。没有换行按 LF 处理。LF 与 CRLF 混用时记录各自数量，并在扫描报告中列出文件路径。
- **写入**时保留字段顺序，使用两个空格缩进，保留非 ASCII 字符并以换行结尾。库内 manifest 统一为 LF，新文件默认使用 LF 且不带 BOM。写回已有文件时沿用原换行和 BOM，避免只改一个字段却让整份文件显示为改动。混用时选原文件中占多数的换行，数量相同则用新文件默认值，并在写出结果中标记换行已规范化。
- Library 根目录的 `.editorconfig` 指定编辑器将 `_asset.manifest.json` 和 `_index/*.json` 保存为 LF。手动改出 CRLF 或混合换行时，`check_library` 会在 `newline_style` 项下列出资产 manifest，可用 `normalize_newlines` 恢复 LF。
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

`Index::default_path()` 返回 `%APPDATA%\Kagura\index.sqlite`。`Index::open(path)` 接受自定义缓存路径。已知库根时，使用 `Index::open_for_library(path, root)` 在创建数据库和目录前解析真实路径并拒绝库根内的输出。SQLite 采用 WAL。缓存的 `application_id` 为 `0x4B475241`，`user_version` 为 2。版本不符时重新建表，`update(root)` 填充资产与文件清单，`parse_files` 从模型与动作文件重建技术数据。

| 表 | 内容 |
|---|---|
| `assets` | `id` 主键，`rel_dir` 唯一且相对 MMD 根。列有 `type, kind, author, name, name_original, title, composer, edition, part, suffix, song_id, character, series, rating, added, updated, license_explicit, manifest_mtime, manifest_size, manifest_json`。可选字段缺省为 NULL，完整 JSON 保留键顺序。 |
| `refs` | `from_id, to_id, field`。复用 check 对 song、related、sets、requires、derived_from、tested_with 的遍历，字段路径保留数组下标，如 `sets[0].motion[0]`。 |
| `tags` | 主键为 `(id, tag)`，并按标签建索引。 |
| `files` | 主键为 `(asset_id, rel)`，相对资产目录记录文件的字节数 `size`、Unix 纳秒 `mtime`、可空的十六进制 `blake3`。资产自己的 manifest 由 assets 表记录。歌曲拥有直属文件及保留目录中的文件，素材拥有各自的子树。 |
| `assets_fts` | FTS5 `trigram`，以 rowid 对应 assets。搜索 name、name_original、author、character、series、title、title_alias、tags、note。显示名、key 中的名称和作者别名均可检索，数组用换行连接。 |
| `meta` | 单行记录规范化的库根、上次成功扫描的 Unix 秒数、资产数及归属到资产的负载文件数。 |
| `skipped_manifests` | 以路径为主键，缓存预期深度的外来 manifest、运行配置与普通 JSON 的 mtime 和 size。内容发生变化后重新识别。 |
| `model_info`、`motion_info` | 每个资产内相对文件名各有一行。模型保存双语名及注释、格式、编码、网格与骨骼等计数和标记。动作保存目标、最大帧号、秒数及六类关键帧数。失败行保存段名、偏移和原因，未知读数为 NULL。 |
| `model_bones`、`model_morphs`、`motion_bones`、`motion_morphs` | 每行通过 `info_id` 关联解析结果。模型按文件顺序保存名称，动作保存去重的轨道名。 |
| `model_textures`、`model_variants` | 保存贴图原名、库内解析路径和存在状态；变体表关联 manifest 声明的模型文件，用于筛选。 |
| `parse_state` | 记录文件清单与贴图复核的版本。移除文件时级联清理结果，大小或修改时间变化时清除该文件的旧解析。 |

`rebuild(&report)` 在一个事务内重建缓存。推荐传入 `scan_library_for_check(root)` 的完整清单。`update(root)` 遍历目录项，以大小和纳秒修改时间判断哪些 manifest 需要重新读取、校验。资产的增删改和文件清单在同一事务内同步，未变文件保留哈希，失败时回滚。切换到另一个库根会重新填充缓存。

`search(query, filters, limit, offset)` 返回总数、当前页及 `used_like`。三个及以上 Unicode 码点走 FTS5 字面子串搜索，一个或两个改走转义过通配符的 LIKE，空串用于浏览。LIKE 自带的大小写折叠限于 ASCII。可按类型、kind、key.author、标签、元数据缺失筛选，按名称、添加日期、更新日期或评分排序，以 ID 稳定分页。`get(id)` 返回完整 manifest，`children(song_id)` 返回歌曲的动作素材。`stats()` 按类型统计资产。作者和作曲者不明取 key 中的 `_Unknown`。授权明确表示 commercial、modify、redistribute 至少一项为布尔值，明确禁止也计入。

运行以下示例可在库外重建缓存，并查看耗时、数量和大小。

```powershell
cargo run -p kagura-core --release --example build_index -- G:\MMD
```

可在库根参数后追加自定义数据库路径。真实库对照测试使用系统临时数据库。设置 `KAGURA_LIBRARY` 后运行 `cargo test -p kagura-core --release --test index_real -- --ignored --nocapture`。

### 3.8 `hash`

- `index` 默认只读取负载文件的元数据。`hash_asset(id)` 按需流式计算 BLAKE3 并按大小和纳秒修改时间复用缓存。`hash_all(progress_cb)` 逐资产写回，每处理一个文件都报告进度（含缓存命中）。扫描重建后可以重新计算摘要。
- 真实库测试中，额外设置 `KAGURA_INDEX_HASH_SAMPLE=1` 可试算 20 份资产。每类按 ID 取前五份负载非空且总大小不超过 64 MiB 的资产，再验证第二次全部命中缓存。
- 迁移日志里记的是 SHA-256。要校验旧日志时，单独提供一个 SHA-256 函数，只用于这个场景。

### 3.9 `txn`（M3 单份 manifest 编辑）

打开详情时，`Index` 从同一份磁盘字节解析 manifest 并计算 BLAKE3。桌面编辑器将该哈希与补丁交给 `Index::edit_manifest(root, id, patch, base_hash)`。可编辑字段为 `tags`、`rating`（0～5，0 表示未评）、`note` 和 `source.url`。补丁必须符合白名单，省略表示保持原值，`source.url = null` 表示移除链接。评分只由用户在桌面编辑器设置。保存实质修改时按上海当天日期填写 `updated`。

核心库通过索引中的 ID 定位库内普通文件，核对原始字节哈希、资产角色、schema 和 key 对应的实际目录。它在原始 UTF-8 文本中替换修改字段的值或插入新成员，保留其他成员的字节、顺序、BOM 和现有换行。生成的内容再次通过 schema 与目录校验后，在同目录写入临时文件并刷盘，重新核对路径和哈希，再原子替换。索引用 SQLite 事务单独刷新该条资产，标签及全文搜索随即生效。

编辑日志位于 `%APPDATA%\Kagura\edits\YYYY-MM.jsonl`。每条记录含编辑编号、库根、资产 ID、相对路径、上海时间、旧/新 BLAKE3、改动字段和旧文件的完整 UTF-8 文本（含原始 BOM 和行尾，按 JSON 字符串转义）。Kagura 各实例共用操作系统文件锁。替换前先持久化待完成记录，更新索引后原子发布月度日志。中断后打开库会按磁盘哈希补全索引和日志；如果出现第三种内容或资产已移走，保留恢复记录并报告冲突。

`list_edits` 按当前库返回最近修改和撤回状态。`undo_edit` 要求当前字节仍匹配该次编辑的新哈希，才从日志恢复原字节，并用独立记录关联原编辑。其他程序在打开详情后所做的更改会触发冲突。在 Windows 上，目标文件的读句柄须在原子替换前关闭，未参与 Kagura 文件锁的外部写入者仍可能在最后一次哈希核对与替换之间抢先写入。

后续批量移动与整理沿用迁移工具的计划流程：

1. **计划**：生成操作列表（`Move` / `EditManifest` / `RemoveEmptyDir`），不碰磁盘。
2. **预检**：在虚拟文件系统上依次模拟每一步（同 `dedup.py`），检查来源存在、目标不存在、schema 通过、引用不会悬空、路径合法。有任何问题就整批拒绝。
3. **执行**：移动前确认文件标识（卷序列号 + 文件 ID）没变；每完成一步立即写一行 JSONL 日志；编辑操作在日志里保存原内容。
4. **撤回**：按日志倒序执行。

MCP 的写操作也走这一套，agent 只能提交计划，执行需要用户确认。

### 3.10 `parse`（模型与动作）

解析器使用 128 KiB 的 `BufReader<File>`，按段读取计数、名称和帧号，按索引宽度跳过几何、插值及物理参数。跳过前检查是否超出文件长度。错误包含段名、从文件开头计的字节偏移和原因。PMX 单段解码文本上限为 8 MiB，文本和名称列表的读入预算为 64 MiB。无效编码用替换字符解码。

- **PMX 2.0 / 2.1** 识别 UTF-16LE / UTF-8、双语模型名和注释、顶点、三角面、贴图、材质、骨骼、表情、刚体与关节。保留骨骼名和表情名。
- **PMD** 以 Shift-JIS 读取模型名、注释与定长名称，支持英文扩展。从材质的纹理文件名字段拆出 `*` 分隔的贴图及球面贴图，按首次出现去重。表情计数排除 base。
- **VMD** 识别 `0002` 的 20 字节目标名和 `file` 的 10 字节目标名。统计骨骼、表情、镜头、光照、自阴影和 IK 显示六段，记录各段共同的最大帧号、使用的骨骼及表情名。时长按最大帧号 / 30 计算。目标为「カメラ・照明」或包含镜头关键帧时标记为镜头。

`STANDARD_BONES` 默认检查「センター、上半身、下半身、首、頭、左腕、右腕、左足、右足」九个名称。调用者可用 `has_required_bones` 指定另一份清单。刚体数大于零表示有物理。贴图相对模型文件定位，允许库内共享贴图，越出 Library 的路径与重解析点记为缺失。模型筛选按资产去重，只统计 manifest 声明的变体。标准骨骼只用于角色模型，其他资产的详情返回空标记。

每次打开库或重扫后，桌面用独立 SQLite 连接在后台运行解析，按批提交。读写解析缓存期间仍可浏览已有资产。成功结果和损坏文件诊断均以大小和纳秒修改时间复用；贴图增删可以单独更新存在状态。解析全程只读资产文件，技术数据存入索引。可用 `cargo run -p kagura-core --release --example parse_library -- G:\MMD` 复核计数与耗时；再运行 `python -B tools/verify_parse.py G:\MMD <索引路径>` 按稳定路径抽样，各对照 200 个 PMX 和 VMD。

#### 真实库验收（2026-11-24）

Windows release 版本在空解析缓存上读取 20,953 个文件并写入结果，用时 256.100 秒。另计基础索引重建 25.785 秒。无变化的复跑中，清单更新 0.501 秒，解析缓存复核 0.062 秒，重读 0 个文件。

| 格式 | 文件数 | 成功 | 失败 |
|---|---:|---:|---:|
| PMX | 6,125 | 6,125 | 0 |
| PMD | 1,022 | 1,022 | 0 |
| VMD | 13,806 | 13,708 | 98 |

| VMD 出错段 | 数量 | 实际文件及诊断示例 |
|---|---:|---|
| 光照 | 67 | `爆発モーション.vmd`，偏移 181，声明需要 3,640 字节，余 61。 |
| 自阴影 | 18 | `Let me in the bathroom.vmd`，偏移 1,047,434，声明需要 29,300,097,024 字节，余 61。 |
| 表情 | 6 | `Motion0.vmd`，偏移 22,890,807，到文件末尾仍缺 4 字节的段计数。 |
| IK 显示 | 5 | `メダロット・ハク.vmd`，偏移 1,137,741，读取 IK 控制器列表时仅余 15 字节。 |
| 骨骼 | 1 | `into the unknown (panic at the disco ver).vmd`，偏移 54，声明需要 11,235,975 字节，余 4,405,923。 |
| 签名 | 1 | `phuthon.vmd`，前 30 字节与两种已知签名均不匹配。 |

独立 Python 读取器按库内路径 SHA-256 排序抽样，候选集包括解析失败的文件。200 份 PMX 的模型名、顶点数、骨骼数、表情数逐项一致。200 份 VMD 中，197 份的目标名、最大帧号与六段计数逐项一致，另 3 份均被两端拒绝。额外核对了全库 4 份 PMX 2.1。Blender 的 MMD Tools 4.5.14 独立读取 24 份 PMX 2.0，96 个字段均一致。

按声明变体统计，227 份模型缺少贴图，120 份角色标准骨骼不齐全，其中 24 份角色同时缺图。若计入随附文件，缺图涉及 229 份模型，标准骨骼不齐全涉及 211 份角色。采集前后逐一核对路径、修改时间和内容摘要，8,824 份 manifest 全部一致。

---

## 4. 依赖（待确认版本）

| 用途 | crate |
|---|---|
| JSON | `serde`、`serde_json`（`preserve_order`） |
| 模型与动作文本解码 | `encoding_rs 0.8.44`，用于 Shift-JIS、UTF-16LE 和 UTF-8 |
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
