# Kagura（神楽）开发计划

> MMD / 3D 资产的整理、浏览与简单预览工具
> 版本：v0.5 · 2026-11-24（M3 Kagura MVP 完成）

---

## 1. 定位

Kagura 是资产的**货架和试衣间**：把本地的模型、场景、动作套装按「一个文件夹 = 一份资产」整理好，方便批量浏览、筛选、简单预览，选好后交给 MMD / Blender / UE5 使用。

**做什么**
- 按文件夹扫描、建索引，读取 `_asset.manifest.json`
- 批量浏览：缩略图、标签、作者、类型筛选，收藏、评分、简评
- 简单预览：模型转一圈、动作配默认模型播放、音频试听
- 内容去重（BLAKE3）
- 送出：在资源管理器里打开、复制路径、在 Blender 中打开
- 管理 Mod 工程（游戏 / Mod），关联资产库里的原模型

**不做什么**
- 不做组合与兼容修复：这些在 Blender / UE5 里做
- 不管理渲染参数：Blender、UE5 的渲染设置留在各自软件里
- 不管理 MME / ray-mmd：MMD 9.32（2019）和 MME 0.37（2014）早已停更，效果只能在 MMD 里用。独立的 Effects 目录已于 2026-11-24 删除；场景自带的 MME（如镜面）留在场景文件夹里，作为场景的一部分，记在 `bundled_effects`
- 不复制、不搬动原件：就地建索引

**设计原则**
- 文件夹 + JSON 是唯一的原始数据，SQLite 只是索引，坏了可以重建
- 资产之间用 ULID 关联，不依赖路径
- 文件夹名由 key 生成，不手写；key 与文件夹名必须一致
- 每份资产根目录放一个 `_asset.manifest.json`（歌也一样，靠 `type` 区分）；内容必须有 `"format": "kagura-asset-manifest"`，版本看 `schema`。`format` 永远不改
- 识别：只读资产根目录；format + ULID + type 三项都对才算记录；同名但不是我们的文件不覆盖，记录改写到 `_asset.manifest.alt.json` 并进复核队列。原作自带的 `info.json` 等配置文件保持原样
- 主名保持引擎中立，各引擎支持做成模块，以后重心转向 UE5 也不用改名

---

## 2. 目录结构（资产库规范，已按此迁移完成）

```
G:\MMD\
├── Library\
│   ├── Models\
│   │   ├── Characters\<作者>\<模型名>\
│   │   └── Props\<作者>\<物品名>\
│   ├── Stages\<作者>\<场景名>\              (含天空球、HDRI；自带的 MME 留在场景内)
│   ├── Motions\
│   │   ├── <歌名(版本)>_<作曲者>\
│   │   │   ├── _asset.manifest.json        (type=song)
│   │   │   ├── _audio\ _docs\ _lyrics\ _archive_attachments\   (保留目录，没有 manifest)
│   │   │   ├── <类型>_<作者>[_<后缀>]\     (motion / face / lip / camera / pack / pose)
│   │   │   └── 歌目录下也可直接放 readme 类文本
│   │   └── _General\
│   │       ├── <类型>_<作者>[_<后缀>]\     (不属于任何歌的通用动作、姿势)
│   │       └── _audio\                     (通用 BGM、音效)
│   └── _index\authors.json                 (作者别名表)
├── Projects\<游戏名>\<Mod名>\               (_game.json / project.json / work / release)
├── Inbox\                                   (待整理、待 agent 归类；_notes 放旧笔记)
├── Archives\                                (原始压缩包)
├── Tools\                                   (PmxEditor、mmd_tools、MME 本体、教程)
├── _Duplicates\                             (确认重复后移出的文件，按批次分目录；项目基本完成后再清理)
└── _design\                                 (schema、提示词、迁移脚本与运行记录)
```

**规则摘要**
- 模型的唯一 key = 作者 + 模型名（不是角色名）。小改动（颜色、饰品、版本、物理、尺寸）作为 `variants`；体型、拓扑或角色变了就新建；别人改的模型新建在改模者名下，并用 `derived_from` 关联原模型。
- 模型只分 角色 / 物品 两类，往下用 `series`、`character`、`tags` 筛选，不再分文件夹。
- 场景沿用「作者 / 名称」，作者不明写 `_Unknown`。天空球（PMX）和 HDRI（.hdr/.exr，Blender / UE5 环境光用）都算场景，用 `kind` 区分，不另开目录。
- 歌的 key = 歌名 + 版本 + 作曲者（不含歌手）。short / full / remix、或剪辑长度差 20 秒以上的，算不同的歌，用 `edition` 区分（如 `RL Grime Remix`、`53s`），并用 `related` 互相关联；只差几秒或换歌手是同一首歌下的不同音频，用 `offset_frames` 对齐。
- 作曲者只填有依据的（readme、音频文件名），不凭记忆补；不明写 `_Unknown`。
- 同名、同版本的歌只保留一首：素材合并到同一个歌目录，完全相同的文件移到 `_Duplicates`，信息更全的一份保留。
- 歌下的素材按「类型_作者[_后缀]」一级；作者整套发布的用 `pack` 不拆；只有第 0 帧的 VMD 算 `pose`。音频统一放歌的 `_audio`。
- 名称保留原文，非法字符替换为 `_`，末尾的点和空格去掉；名称 ≤ 40 字符，后缀 ≤ 24 字符；开启 Windows 长路径支持（现最长 214 字符）。
- 同一作者统一写法，旧写法记进 `_index\authors.json`。

---

## 3. 数据格式

位于 `G:\MMD\_design\schemas\`（JSON Schema 2020-12）：

| 文件 | 内容 |
|---|---|
| common | format（必填，固定值）、schema、id(ULID)、作者、来源、结构化授权、标签、评分、变体、部件 |
| model | kind(character/prop)、series、character、derived_from、variants、tech、mod_base |
| stage | kind(stage / skydome 天空球 PMX / hdri 环境贴图 .hdr/.exr)、variants 或 parts、bundled_effects、scale、requires |
| song | key(title/edition/composer)、title_alias、audio(+offset_frames)、related、sets(常用组合) |
| motion_part | key(part+author+suffix)、song、files(role/variant/made_for)、required_morphs |
| project / game | Mod 工程与游戏信息，base_assets 用 id 引用资产 |
| aliases | 作者、歌名、作曲者别名表 |

effect 类型已于 2026-11-24 停用，schema 已删除。

---

## 4. 系统结构

```
┌─────────────── Kagura ───────────────┐
│  UI（浏览 / 筛选 / 详情 / 预览）        │
│  ├─ 索引层   SQLite（可重建）          │
│  ├─ 解析层   PMX / PMD / VMD / readme  │
│  ├─ MCP 层   供外部 agents 调用        │
│  ├─ 预览层   babylon-mmd（WebView）    │
│  └─ 交接层   打开文件夹 / Blender / UE │
└──── 读写：文件夹 + _asset.manifest.json ────┘
```

**技术选型**
- 桌面壳：**Tauri**（Rust 后端 + Web 前端），已定。Rust 生态有 BLAKE3、SQLite、JSON Schema 校验
- 前端：React + TypeScript + Vite，虚拟列表 / 网格使用 TanStack Virtual
- 预览：babylon-mmd（PMX、VMD、镜头、音频），不支持 MME
- 索引：SQLite + FTS5（日文、中文全文搜索）
- 解析：自写 PMX/VMD 头部解析（格式公开），readme 按 cp932 → utf-8 → gbk 解码
- 整理协作：内置 **MCP 服务**（查询、读 manifest、提交修改草稿、校验、预检、执行与撤回），由外部 agents 归类和补元数据；工具本身不调用 LLM。写操作一律走「草稿 → schema 校验 → 预检 → 用户确认 → 执行 → 日志」
- workspace 声明 Rust 1.90+（本机 1.100，锁定的 Tauri 要求 1.90）。`jsonschema` 0.63 的 2020-12、跨文件 `$ref`、`unevaluatedProperties`、离线和 format 检查均通过与 Python 版的对照。`serde_json` 启用 `preserve_order`。索引使用 `rusqlite` 0.43.0（内嵌 SQLite 3.54.1，已验证 FTS5 trigram）和 `blake3` 1.8.7。
- 参考实现：`_design\migration\plan.py`。Rust 版已逐条对照：`safe()` 7,105 个名字、8,824 条记录的目录全部一致

---

## 5. 功能清单与优先级

| 优先级 | 功能 |
|---|---|
| P0 | 扫描 Library，读取 _asset.manifest.json，建 SQLite 索引 |
| P0 | 资产列表：缩略图网格、按类型 / 作者 / 标签 / 系列筛选、搜索 |
| P0 | 详情页：元数据、变体、授权、readme，打开所在文件夹 |
| P0 | 编辑 _asset.manifest.json（标签、评分、简评、来源链接），带 schema 校验 |
| P1 | PMX / PMD / VMD 解析，模型技术信息、目标模型、帧数和所需表情进入索引缓存 |
| P1 | BLAKE3 去重，重复报告（排除场景自带的 MME 控制器这类配件） |
| P1 | 歌曲视图：一首歌下的动作 / 表情 / 镜头 / 音频、常用组合 sets、相关版本 related |
| P1 | 模型 3D 预览（旋转、切换变体） |
| P2 | 动作预览：默认模型或选中模型播放 VMD + 镜头 + 音频 |
| P2 | 选择时的兼容提示（只提示，不修复）：缺多少表情、骨骼是否标准 |
| P2 | Projects 视图：游戏 / Mod，关联原模型；发布前检查授权 |
| P2 | 「在 Blender 中打开」（mmd_tools 脚本导入） |
| P2 | MCP 服务（只读）：搜索、读 manifest、统计、校验 |
| P3 | MCP 服务（写）：提交修改草稿、预检、执行、撤回，带确认与日志 |
| P3 | Inbox 新资产导入 + agent 归类 + 人工复核队列 |
| P3 | 元数据补全：作者、作曲者、授权（读 readme / PMX 注释，由 agent 经 MCP 提交） |
| P3 | 缩略图自动生成（用预览层离屏渲染） |
| P3 | UE5 支持：FBX / glTF 预览，导出路径给 UE Python 导入 |

---

## 6. 里程碑

| 阶段 | 内容 | 状态 |
|---|---|---|
| **M0 迁移准备** | 备份 / 文件清单、长路径、建目录骨架、移动工具与压缩包 | ✅ 完成 |
| **M1 迁移计划表** | 扫描、规则 + LLM 归类、计划表；检查贴图跨目录引用、路径长度 | ✅ 完成 |
| **M2 实际迁移** | 按 场景 → 模型 → 动作 整批迁移；全程记日志可撤回 | ✅ 完成（2026-11-23） |
| **M2.5 迁移复核与修正** | 记录改名为 `_asset.manifest.json`、key 与文件夹对齐、作者别名表、保留目录、删除 Effects、15 组同名歌合并 / 拆分 | ✅ 完成（2026-11-24） |
| **M3 Kagura MVP** | P0 全部：扫描、索引、浏览、筛选、详情、编辑 | ✅ 完成（2026-11-24） |
| **M4 解析与去重** | P1：头部解析、去重、歌曲视图、模型预览 | 1～2 周 |
| **M5 预览与工程** | P2：动作预览、兼容提示、Projects、Blender 交接、只读 MCP | 2～4 周 |
| **M6 扩展** | P3：MCP 写操作、Inbox + agent 归类、元数据补全、自动缩略图、UE5 | 按需 |


### M3 进度（仓库 [SNamiris/Kagura](https://github.com/SNamiris/Kagura)）

| 步骤 | 内容 | 状态 | commit |
|---|---|---|---|
| 0 | Cargo workspace、kagura-core、Tauri 2 骨架、内嵌 schema；rustfmt / clippy 配置 | ✅ | `3126a27` `5783ed8` `41bff92` |
| 1 | `tools/export_golden.py` 导出标准答案（safe 7,105 条、destination 8,824 条、scan_summary） | ✅ | `ce0254b` |
| 2 | `naming`：复刻 `safe` / `rel`，22 个边界用例 | ✅ | `d29c94f` |
| 3 | `manifest`：识别 Asset / ForeignManifest / RuntimeConfig / SourceJson；保留换行与 BOM 的写回 | ✅ | `b75f3f8` `4f83d7a` |
| 4 | `layout` + `scan`：按 key 算目录、只在预期深度找记录、跳过保留目录与重解析点、歌曲索引 | ✅ | `25cff65` |
| 5 | `check`：报告共 9 项，包括 8 项一致性检查（含 `newline_style`）和 1 项扫描错误统计（`scan`）；`check_library` 命令约 1.8 秒 | ✅ | `068bf88` `5a0a4e4` |
| 6 | `index`：SQLite + FTS5 trigram、增量更新、筛选与统计、按需 BLAKE3 | ✅ | `a39ae75` |
| 7 | [Tauri 界面](docs/ui.md)：真实库扫描、虚拟网格与列表、筛选、详情与引用跳转、打开目录、库检查 | ✅ | `3fd73f3` |
| 8 | 在界面编辑标签、评分、简评和来源链接，校验冲突、同步索引、按日志撤回 | ✅ | `a0030d1` `78678c9` |

workspace 共 52 项测试全部通过，覆盖 Tauri 命令、编辑与撤回、并发与中断恢复，以及真实库索引、manifest 往返和目录对照（设 `KAGURA_LIBRARY=G:\MMD` 并加 `--include-ignored`）。

自己检查库：`cargo run -p kagura-core --example check_library -- G:\MMD`

**和 Python 版的已知差异**（均不影响现有库）：浮点 `1e-05` 写成 `0.00001`；大小写折叠用完整 Unicode casefold，与 Python 3.10 有 116 个码点不同，库内名字均未用到；扫描更严格（错误层级标为 `UnexpectedDepth`，缺 `format` 视为外来文件）。

### M4 进度

| 步骤 | 内容 | 状态 | commit |
|---|---|---|---|
| 4.1 | 流式读取 PMX / PMD / VMD，按文件段报告解析错误，抽样与 Python 对照 | 进行中 | |
| 4.2 | 增量缓存解析结果，详情显示模型与动作数据，筛选缺失贴图和非标准骨骼 | 进行中 | |
| 4.3 | 后台计算 BLAKE3，显示进度与重复文件报告 | 待做 | |
| 4.4 | 歌曲视图对照帧数、时长和目标模型，比较动作需要的表情与模型表情 | 待做 | |
| 4.5 | 使用 babylon-mmd 预览模型、旋转视角并切换变体 | 待做 | |

---

## 7. 当前库概况（2026-11-24 复核）

**Library：111,968 个文件，179.9 GB，8,824 份记录**

| 类型 | 数量 | 细分 |
|---|---|---|
| 模型 | 2,036 | 角色 1,627 / 物品 409 |
| 场景 | 1,358 | 一般场景 1,180 / 天空球 178 / HDRI 0 |
| 歌 | 2,220 | 有作曲者 14 首 |
| 动作素材 | 3,210 | pack 1,925 / motion 973 / camera 251 / pose 38 / lip 14 / face 9 |

**复核结果**：ID 无重复；所有记录都在 key 对应的路径；资产间引用（song / related / sets / requires / derived_from）无悬空；记录里列出的文件全部存在；歌曲索引可正常建立；没有空目录、没有缺 manifest 的素材目录；作者无大小写重复；最长路径 214 字符。

**待补的元数据**
- 作者不明：模型 1,289（63%）、场景 725（53%）、动作素材 2,898（90%）
- 作曲者不明：2,206 / 2,220 首歌
- 授权有明确信息：模型 433、场景 65，其余未知（做 Mod 前要优先补）
- 作者别名表只有 9 条

**库外目录**
| 目录 | 文件数 | 大小 | 说明 |
|---|---|---|---|
| Tools | 1,910 | 2.9 GB | 工具与教程 |
| _Duplicates | 51,668 | 9.0 GB | 确认重复后移出，项目基本完成后清理 |
| _design | 36,643 | 19.9 GB | 其中 migration_runs 是撤回用的暂存，项目基本完成后清理 |
| Projects | 44 | — | 现有 Mod 工程 |
| Inbox | 8 | — | 旧笔记 txt |
| Archives | 0 | — | 压缩包已解压归档，目录为空 |

**历史记录**
- 2026-11-22 首次扫描：约 24.4 万文件、290 GB
- 2026-11-24 删除 `Library\Effects`（1,757 份记录，含 547 个 ray-mmd HDRI，约 42.6 GB）；记录清单在 `_design\migration_runs\effects-removal-20261124\`
- 2026-11-24 15 组同名歌：11 组合并、Satisfaction 按 Remix 区分、bad guy / Gentleman / Say So 按剪辑长度拆分；日志在 `_design\migration_runs\song-dedup-20261124\`
- 2026-11-24 全部 8,824 份 manifest 由 CRLF 统一转为 LF（只改换行，内容不变）；备份、日志与 `undo.py` 在 `_design\migration_runs\manifest-lf-20261124\`。`Library\.editorconfig` 规定记录文件用 LF

---

## 8. 决定记录

**已决定（2026-11-24）**
- [x] 前端采用 React + TypeScript + Vite；资产浏览的虚拟列表和网格使用 TanStack Virtual。
- [x] 记录文件换行：库内统一 LF；新建文件用 LF、无 BOM；写回已有文件时保留原换行和 BOM（兜底）；check 报告不是 LF 的文件，可用 `normalize_newlines` 恢复
- [x] MCP 草稿：JSON Patch 和整份 manifest 都接受；低风险草稿（只改标签、来源链接、别名、简评）允许自动执行，其余需确认
- [x] 作者不明：文件夹名保持 `_Unknown`，界面上显示为「未知」
- [x] 桌面框架：**Tauri**（Rust 后端 + Web 前端）。读取、校验、命名规则用 Rust 重写，`_design\migration\plan.py` 作为参考实现与对照测试
- [x] 暂存清理：`_Duplicates`、`migration_runs`（约 29 GB）**暂不清理**，等项目基本做完再处理
- [x] 乱码文件名：等 Kagura 实际扫描时再评估（PMX 按文件名找贴图，改名需同步改 PMX 内路径）
- [x] 不内置 LLM：改为提供 **MCP 服务**，由外部 agents 协作整理（归类、补元数据、复核）

**以后再定**
- [ ] 子模块命名：等真开始做 UE5 支持再说
- [x] MCP 工具清单与权限：见 `docs/mcp-design.md`。只读 + 提交草稿两级，执行只能在界面里由用户确认

## 9. 相关文件

- 仓库 `E:\Repos\kagura`：`docs/core-design.md`（核心库设计）、`docs/mcp-design.md`（MCP 设计）、`schemas/`（以仓库为准）
- `G:\MMD\_design\schemas\`：JSON Schema（8 个，已复制进仓库）
- `G:\MMD\_design\prompts\classify_prompt.md`：归类规则（可改写为给 agents 的 MCP 使用说明）
- `G:\MMD\_design\migration\`：迁移脚本（plan.py 中的读取与路径函数可复用到 Kagura）
- `G:\MMD\_design\review_20261123.md`、`review_song_dups_20261124.md`：复核记录
- `G:\MMD\_design\trial_library\`：早期 29 份试填草稿（格式已过时，仅供参考）
