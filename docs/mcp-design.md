# Kagura MCP 工具设计

> v0.1 · 2026-11-24 · 对应 PLAN.md 的 M5（只读）/ M6（提交草稿）
> 依赖：`docs/core-design.md` 里的 `kagura-core`

---

## 1. 定位

- Kagura 本身不调用 LLM。外部 agents（Claude Desktop、Kiro 等）通过 MCP 读库、查资料、**提交修改草稿**。
- **agent 永远不能直接改库**。它只能生成草稿；草稿要通过 schema 校验和预检，再由用户在 Kagura 界面里确认，才会执行。
- 所有规则（命名、路径、校验、预检）都在 `kagura-core` 里，MCP 层只做参数检查和转发，不另写一套逻辑。

---

## 2. 权限分级

| 级别 | 能做什么 | 是否改磁盘 | 默认 |
|---|---|---|---|
| **R 只读** | 搜索、读 manifest、读 readme、解析 PMX/VMD、检查、统计 | 否 | 开启 |
| **P 提交** | 校验、预览路径、创建和撤销草稿、对草稿做预检 | 只写 Kagura 自己的草稿目录（`%APPDATA%\Kagura\drafts`），不碰资产库 | 开启 |
| **X 执行** | 执行草稿、撤回 | 是 | **不通过 MCP 提供**。低风险草稿自动执行（见第 9 节），其余在界面里由用户确认 |

工具都标注 MCP 的工具提示：R 级标 `readOnlyHint`；P 级标非只读、非破坏性。具体字段名以实现时的 MCP 规范版本为准。

---

## 3. 工具清单

### 3.1 R：查询

| 工具 | 参数 | 返回 |
|---|---|---|
| `library_stats` | — | 各类型数量、元数据缺失统计（作者 / 作曲者 / 授权未知数） |
| `search_assets` | `query?` `type?` `kind?` `author?` `tags?` `missing?`（`author` / `composer` / `license`）`limit≤100` `cursor?` | 资产摘要列表 + 下一页游标 |
| `get_asset` | `id` | 完整 manifest、所在路径、manifest 的内容哈希（提交草稿时要带上） |
| `get_song` | `id` | 歌的 manifest、下面所有素材的摘要、`_audio` 文件列表、`related` 版本 |
| `list_asset_files` | `id` `glob?` `limit≤500` | 资产目录里的文件（相对路径、大小、扩展名） |
| `read_asset_text` | `id` `path` `max_chars≤8000` | 文本内容，自动按 utf-8 → cp932 → gbk 解码，返回用了哪种编码 |
| `inspect_pmx` | `id` `file?` | 模型名、英文名、注释、顶点 / 骨骼 / 表情数量、贴图列表 |
| `inspect_vmd` | `id` `file` | 目标模型名、帧数、骨骼 / 表情 / 镜头关键帧数量、换算时长 |
| `find_duplicates` | `scope`（`asset` / `song` / `type`）`id?` | 内容完全相同的文件分组（BLAKE3） |
| `run_check` | `checks?` | `check` 模块的报告：路径、引用、文件存在、ID、作者大小写等 |
| `resolve_author` | `name` | 别名表里的标准写法，没有就返回空 |
| `list_review_queue` | `limit` | 待复核项：外来 manifest、校验失败、草稿被拒的原因 |

### 3.2 P：校验与草稿

| 工具 | 参数 | 返回 |
|---|---|---|
| `validate_manifest` | `manifest` | 校验结果（不保存任何东西） |
| `preview_destination` | `manifest` | 按 key 算出的目录，以及是否和现有资产冲突 |
| `create_draft` | `title` `reason` `ops[]` | 草稿 ID + 预检结果 |
| `precheck_draft` | `draft_id` | 重新预检（库被改过之后用） |
| `get_draft` / `list_drafts` | `draft_id?` `status?` | 草稿内容和状态 |
| `withdraw_draft` | `draft_id` | 撤销尚未执行的草稿 |

**草稿里允许的操作 `ops`：**

| op | 用途 | 说明 |
|---|---|---|
| `patch_manifest` | 补作者、作曲者、授权、来源、系列、角色、标签、简评 | 用 JSON Patch；必须带 `base_hash`（取自 `get_asset`），库里的文件变了就失效 |
| `rekey` | 改 key 导致目录改名 | 目录由 core 按新 key 算，agent 不能指定路径 |
| `merge_songs` | 同名同版本的歌合并 | 参考这次 15 组的处理方式 |
| `move_part` | 把素材移到另一首歌 | 同上 |
| `quarantine` | 把完全相同的文件移到 `_Duplicates` | 只有 BLAKE3 完全相同才允许 |
| `add_alias` | 往作者别名表加一条 | — |

**agent 不能改的字段**：`id`、`format`、`schema`、`type`、`added`、`rating`（评分是你个人的判断）。`classified_by` 由程序自动填 `agent`。

---

## 4. 资源和提示词

| 类型 | 名称 | 内容 |
|---|---|---|
| 资源 | `kagura://schema/{type}` | 对应的 JSON Schema |
| 资源 | `kagura://rules` | 整理规则：由 `_design\prompts\classify_prompt.md` 改写，去掉 LLM 输出格式部分，换成「怎么用工具和草稿」 |
| 资源 | `kagura://asset/{id}` | 单份资产的 manifest |
| 提示词 | `fill_metadata` | 选一批缺作者或授权的资产，读 readme 和 PMX 注释，提交补全草稿 |
| 提示词 | `review_song_duplicates` | 找同名歌，对比帧数和哈希，提交合并或拆分草稿 |

---

## 5. 执行流程

```
agent ──create_draft──▶ 草稿目录 ──自动预检──▶ 待确认
                                              │
                         用户在 Kagura 界面里查看差异、勾选
                                              │
                               执行（txn）──▶ JSONL 日志，可撤回
```

- 界面按草稿显示改动：manifest 显示字段差异，移动显示原路径和新路径。
- 可以整份确认，也可以只确认其中几条。
- 执行前再预检一次；只要有一个 `base_hash` 对不上，整份草稿拒绝，agent 需要重新读取后再提交。
- 执行记录写进日志，标明来自哪份草稿、哪个 agent。

---

## 6. 安全

- **路径**：所有工具只接受资产 `id` 加资产内的相对路径。相对路径用 `check_rel()` 检查，不能用 `..` 跳出资产目录，也不能跟随重解析点。
- **readme 是不可信内容**：返回给 agent 的文本可能有诱导指令。工具在返回结果里注明「以下为文件内容」，而且就算 agent 被误导，它也只能提交草稿，执行要你确认。
- **数量限制**：每次读取的文本长度、列表条数都有上限；一份草稿最多 200 条操作。
- **R18 内容**：设置里可以选，搜索结果里不返回带指定标签的资产（默认不过滤）。
- **审计**：每次工具调用都记一行日志（工具名、参数摘要、时间），放在 `%APPDATA%\Kagura\mcp.log`。

---

## 7. 运行方式

- **首选 stdio**：`kagura-mcp --library G:\MMD`，由 agent 客户端启动。不需要 Kagura 界面在运行，只读和提交草稿都能用；草稿留到下次打开界面时确认。
- **可选本机 HTTP**：Kagura 界面运行时，在 `127.0.0.1` 开一个端口，方便多个 agent 同时连接。以后有需要再做。
- **索引**：MCP 和界面共用同一个 SQLite 索引文件。用 WAL 模式，允许一边读一边写；两边同时写时由 core 加文件锁。

---

## 8. 实施顺序

| 阶段 | 内容 |
|---|---|
| M5-a | `kagura-mcp` 骨架、stdio 传输、`library_stats`、`search_assets`、`get_asset` |
| M5-b | 其余 R 级工具、资源 `kagura://schema` 和 `kagura://rules` |
| M6-a | `validate_manifest`、`preview_destination`、草稿目录、`create_draft`（只支持 `patch_manifest`） |
| M6-b | 界面里的草稿审核页、执行与撤回 |
| M6-c | 其余 op（`rekey`、`merge_songs`、`move_part`、`quarantine`、`add_alias`）、两个提示词 |

---

## 9. 决定记录（2026-11-24）

- [x] 草稿格式：**JSON Patch（RFC 6902）和整份新 manifest 都接受**，内部统一转成 Patch 再预检、展示差异。
- [x] **允许自动执行低风险草稿**。范围（可在设置里关掉）：
  - 只含 `patch_manifest`，且只动 `tags`、`source.url`、`source.platform`、`title_alias`、`note`
  - 不改 key、不移动文件、不改授权字段
  - 预检通过、`base_hash` 全部对得上
  - 自动执行同样写日志，可以撤回；界面上有「最近自动执行」列表
  - 其他草稿仍需用户确认
- [ ] MCP Rust SDK（`rmcp`）版本与工具注解写法：到 M5 再核对。
