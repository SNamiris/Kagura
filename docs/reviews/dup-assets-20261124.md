# 94 组整份重复资产合并审查

2026-11-24 · 只读审查 · 库根 `G:\MMD`

**78 组可进入自动草稿，16 组需要人工确认。** 自动候选涉及 157 份资产，建议保留 78 份、归档 79 份。拟归档的 1,023 个负载路径合计 2,763,909,271 字节（2.57 GiB）。隔离到同一个 G 卷会保留这些字节，只有最终清理才可能释放空间。

同名 [JSON](dup-assets-20261124.json) 逐份保留磁盘 manifest、人工信息标记、入向和出向引用、负载清单与摘要、声明项检查、同内容路径映射、操作顺序和字段新值；另有全部 300 对子集的逐对依据。

## 范围与证据

- 从既有索引复核 94 组、189 份资产，包括 140 份动作素材、39 份模型、10 份场景。逐目录核对磁盘清单，重新计算 **2,538 个负载路径、6,343,161,239 字节**的 BLAKE3，全部与索引吻合。共核对 928 项文件声明，包括 788 个变体、部件或素材入口，以及 140 项许可说明和预览路径；全部存在。
- 22 组的相对路径与内容逐一相同，72 组存在改名或目录层次差异。03、24 两组连同一内容在包内重复出现的次数也不同，空间按实际归档侧计算。
- 按评分、简评、标签、明确许可或来源 URL 筛查，27 份 manifest 含有须保留的信息。署名取自 readme 前 1,500 字、索引中的 PMX/PMD 注释和有明确署名的原包装路径。分别辨认最终制作者、改模者、转换者、配件作者和歌手；依据不足时保留未知值。
- 反查了 refs 的 song、sets、related、derived_from、tested_with、requires 等字段。指向这 189 个资产 ID 的入边共 0 条；部分素材自身有指向配件的 tested_with。跨歌操作另列父歌的所有入边。Projects 下 133 份 JSON 均已检查，其中 7 份为逐行日志，未发现 project.base_assets。
- 现有及候选目录已用 Rust layout 复算，字段补丁通过 schema 校验，目标已核对磁盘及索引。02、03、86 先归档组内占位者再 rekey。执行草稿前应重验摘要、引用和目录占用。

歌曲版本沿用 15 组同名歌审查的口径。同曲同剪辑可合并，歌手差异本身不拆歌；short/full 或相差超过 20 秒的剪辑分别保留。相同动作可以随不同音乐分发，B 类同时核查父歌自有文件、其他子素材、音轨和最大帧。

## 分类与待决定项


| 类 | 判断 | 组数 | 资产数 | 可自动成稿 | 人工确认 | 自动候选归档字节 |
|---|---|---|---|---|---|---|
| A | 同歌同部件 | 62 | 124 | 61 | 1 | 2,239,575,914 |
| B | 跨父歌的相同素材包 | 7 | 14 | 0 | 7 | 0 |
| C | 同一模型或场景的异名、异署名副本 | 21 | 43 | 17 | 4 | 524,333,357 |
| D | 名称或用途冲突 | 3 | 6 | 0 | 3 | 0 |
| E | 其他归属 | 1 | 2 | 0 | 1 | 0 |


| 组 | 类 | 资产或歌曲 | 需要确认 |
|---|---|---|---|
| [04](#g04) | B | nac_platinaDR / 『プラチナ』-shin'in future Mix- DANCEROIDver | 跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。 |
| [07](#g07) | D | 京都花火祭 / 红龙舟（有内室） | 需要人工确认红龙舟是否指整包的館船单独入口，以及有无真正独立的龙舟来源。 |
| [18](#g18) | B | Calabria2008~ / 女主播 | 现行父歌名 Calabria2008~ 和音频文件名 Calabria 2007 年份相差一年，需确认显示曲名；同一 WAV 与 VMD 已核实。 |
| [25](#g25) | A | Yume Yume Live | 提交草稿前人工确定该文件单值 role 采用 lip（依原文件名）还是 face（同时含眨眼），保留另一描述。 |
| [40](#g40) | B | いのちのいぶき / いのちのいぶきダンス | 跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。 |
| [44](#g44) | B | SexiMi / SexyMi | 跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。 |
| [57](#g57) | B | The Zombie Song / the_zombie_song___motion_dl | 跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。 |
| [63](#g63) | E | I won't say I'm in Love / _General | 需要核实原 _General 编目的用途，再将该素材归入已识别歌曲。 |
| [69](#g69) | C | TDA UK Lolita Dress Stardu (星尘模型) / TDA UK Lolita Dress Stardust Ver3.00[Sil | 核对整模限制和另行取得作者许可的情形，再决定修改字段；不自动沿用两个旧布尔值的共同结论。 |
| [71](#g71) | C | tdalukanude / Tda式ルカ モリガン風 Ver1.05 Nude | 确认 Nude 是否为独立第三方改版；本包 readme 对可再配布和使用另有限制，保留具体来源说明。 |
| [74](#g74) | D | TDA Electric Love Miku / 原画 | 确认“原画”目录原意及是否有缺失的独立插图。确认许可证涵盖成品后再考虑保留 Xoriu 的 Electric Love 记录。 |
| [79](#g79) | C | TDA Sailor Haku / TDA Sailor Haku (弱音模型) | 最终改模者没有署名，现有许可明确值并非来自该成品。 |
| [80](#g80) | C | Tda Sexy Bunny （短裤版） / Tda Sexy Bunny （短裤版） (初音模型) | 确认联合署名应如何记录、是否需要成品配布来源，再决定是否入草稿。 |
| [89](#g89) | D | 包厢舞台 / 酒馆 | 核对原压缩包标题和两方期望的主入口；不能从相同字节推断任一侧错放。 |
| [90](#g90) | B | As if its your last SHORT (Cookies X Cream / As if its your last | 跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。 |
| [93](#g93) | B | Freakum Dress / 洋楽モーション]ビヨンセFreakum Dress | 跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。 |


## 300 对严格子集

每对按子集 → 父集计一次，涉及 563 个不同资产。163 对（54.3%）同类且共享核心文件，父集独有内容只涉及预览、说明、视频或系统缓存，符合精简负载的解释。其中 133 对只差预览。另有 59 对可能同包但多了其他内容，54 对更像部件与合集，19 对证据不足，5 对是跨类型共享。符合精简版解释的只略过半数，原始发布历史仍需单独取证。

| 资产类型组合 | 对数 |
|---|---|
| model → model | 140 |
| motion_part → motion_part | 139 |
| stage → stage | 16 |
| song → motion_part | 5 |


| 子集文件数 ÷ 父集文件数 | 按不同内容数 | 按实际文件路径数 |
|---|---|---|
| [0,25%) | 44 | 45 |
| [25,50%) | 0 | 0 |
| [50,75%) | 0 | 0 |
| [75,90%) | 0 | 0 |
| [90%,100%) | 141 | 139 |
| >100% | 0 | 1 |

两种比率的中位数均为 83.33%。有一对的子集路径数多于父集，因为同一内容在子集有多个路径，按不同哈希数仍是严格子集。

### 前 20 对

优先展示证据明确的附件差异，再按未舍入的内容数比例降序、子集内容数降序、双方目录和 ID 升序。文件样例均为父集独有。双方 ID 可在 JSON 对应的 rank 查找。

| 序 | 子集目录 → 父集目录 | 不同内容数 | 实际文件数 | 额外内容示例 |
|---|---|---|---|---|
| 1 | `Models/Characters/InoriAruma On DeviantArt/Tda Luka Long Pajama Shirt` → `Models/Characters/InoriAruma On DeviantArt/Tda Luka Long Pajama Shirt (巡音模型)` | 105/106 (99.1%) | 110/111 | `预览.jpg` |
| 2 | `Models/Characters/aga/TDA Empress Luka` → `Models/Characters/aga/TDA Empress Luka (巡音模型)` | 105/106 (99.1%) | 114/115 | `预览.jpg` |
| 3 | `Models/Characters/Aさん/tda夏日装Neru` → `Models/Characters/Aさん/tda夏日装Neru (亚北模型)` | 96/97 (99.0%) | 106/107 | `预览.jpg` |
| 4 | `Models/Characters/_Unknown/TDA Nagisa Replica Luka` → `Models/Characters/_Unknown/TDA Nagisa Replica Luka (巡音模型)` | 89/90 (98.9%) | 89/90 | `预览.jpg` |
| 5 | `Models/Characters/_Unknown/時崎狂三_MMD` → `Models/Characters/_Unknown/時崎狂三_MMD (约会大作战模型)` | 84/85 (98.8%) | 91/92 | `预览.jpg` |
| 6 | `Models/Characters/Aさん/tda夏日装haku` → `Models/Characters/Aさん/tda夏日装haku (弱音模型)` | 79/80 (98.8%) | 93/94 | `预览.jpg` |
| 7 | `Models/Characters/やまもと/TDA Goth Haku ver1.2` → `Models/Characters/やまもと/TDA Goth Haku ver1.2 (弱音模型)` | 76/77 (98.7%) | 91/92 | `预览.jpg` |
| 8 | `Models/Characters/InoriAruma On DeviantArt/Tda Luka Maid` → `Models/Characters/InoriAruma On DeviantArt/Tda Luka Maid (巡音模型)` | 71/72 (98.6%) | 77/78 | `预览.jpg` |
| 9 | `Models/Characters/_Unknown/TDA赛车MIKU2012` → `Models/Characters/_Unknown/TDA赛车MIKU2012 (初音模型)` | 67/68 (98.5%) | 69/70 | `预览.jpg` |
| 10 | `Models/Characters/_Unknown/时尚露胸装弱音` → `Models/Characters/_Unknown/时尚露胸装弱音 (弱音模型)` | 67/68 (98.5%) | 76/77 | `预览.jpg` |
| 11 | `Models/Characters/やまもと/TDA Goth Luka ver1.1` → `Models/Characters/やまもと/TDA Goth Luka ver1.1 (巡音模型)` | 67/68 (98.5%) | 78/79 | `预览.jpg` |
| 12 | `Models/Characters/_Unknown/Darkness Helena` → `Models/Characters/_Unknown/Darkness Helena (其它模型)` | 66/67 (98.5%) | 66/67 | `预览.jpg` |
| 13 | `Models/Characters/_Unknown/tda夏日装luka` → `Models/Characters/_Unknown/tda夏日装luka (巡音模型)` | 66/67 (98.5%) | 75/76 | `预览.jpg` |
| 14 | `Models/Characters/Tda/TDA Elegant Miku` → `Models/Characters/aga/TDA Elegant Miku` | 65/66 (98.5%) | 65/66 | `预览.jpg` |
| 15 | `Models/Characters/InoriAruma On DeviantArt/Tda Rin School` → `Models/Characters/InoriAruma On DeviantArt/Tda Rin School (镜音铃模型)` | 64/65 (98.5%) | 67/68 | `预览.jpg` |
| 16 | `Models/Characters/InoriAruma On DeviantArt/Tda Rin Maid` → `Models/Characters/InoriAruma On DeviantArt/Tda Rin Maid (镜音铃模型)` | 61/62 (98.4%) | 66/67 | `预览.jpg` |
| 17 | `Models/Characters/aga/Tda 弱音 警察` → `Models/Characters/aga/Tda 弱音 警察 (弱音模型)` | 58/59 (98.3%) | 61/62 | `预览.jpg` |
| 18 | `Models/Characters/顾霏tadayo/TDA Elegant short skirt Haku （young Girl` → `Models/Characters/顾霏tadayo/TDA Elegant short skirt Ha (弱音模型)` | 58/59 (98.3%) | 59/60 | `预览.jpg` |
| 19 | `Models/Characters/_Unknown/Rin` → `Models/Characters/_Unknown/Rin (TDA Secret)` | 57/58 (98.3%) | 57/58 | `预览.jpg` |
| 20 | `Models/Characters/_Unknown/Tda Miku LUVORATORRRRY!` → `Models/Characters/InoriAruma On DeviantArt/Tda Miku LUVORATORRRRY!` | 56/57 (98.2%) | 57/58 | `预览.jpg` |


## 逐组记录

组号沿用整份重复报告。目录相对 G:\MMD，文件路径相对所属资产。JSON 的 actions 按顺序预演；quarantine 的目标以退役 ID 区分。D 类保留两边现状。

<a id="g01"></a>

### 01 · A · Dream of You

**自动草稿候选。** 两份都在歌曲「Dream of You」的同一个 song ID 下，part 都是 pack；全部 18 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。 Hat/Please Read before using.txt 的署名用于帽子模型。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92Z0QD62TPQSJB9B67VR` | motion_part / pack | `Library/Motions/Dream of You__Unknown/pack__Unknown` | 18 / 126,678,940 | 无 |
| `01M81T9JTWCXWDPF5DCTYPJ7ZB` | motion_part / pack | `Library/Motions/Dream of You__Unknown/pack__Unknown_CHUNG HA - Dream of You` | 18 / 126,678,940 | 无 |

**父歌**　`01M81T92Z0RVA0EA7PTXX9K4R7` `{"title":"Dream of You","composer":"_Unknown"}`。
**文件核对**　18 种不同内容，单份集合 126,678,940 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92Z0QD62TPQSJB9B67VR`，最终目录 `Library/Motions/Dream of You__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92Z0QD62TPQSJB9B67VR`，归并 `01M81T9JTWCXWDPF5DCTYPJ7ZB`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JTWCXWDPF5DCTYPJ7ZB` → `_Duplicates/kagura-review-20261124/01M81T9JTWCXWDPF5DCTYPJ7ZB/pack__Unknown_CHUNG HA - Dream of You`。
3. `rekey` `01M81T92Z0QD62TPQSJB9B67VR` → `Library/Motions/Dream of You__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g02"></a>

### 02 · C · Tda Haku Crazy

**自动草稿候选。** 黑、白两个变体及 56 个负载内容逐份对应，作者未知的副本保留了可读的日文说明文件名，另一份有乱码。两个 PMX 的贴图在两边均能解析。

**署名依据**　两个 PMX 的编辑署名和 Read_Me_Tda Haku Crazy.txt 都指向 InoriAruma；沿用库中已有作者目录 InoriAruma On DeviantArt。Tda 等 readme 涉及基础模型或配件。
**建议显示名**　`Tda Haku Crazy`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXDAQ2C6WGVBAKKPBSSP3` | model / character | `Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Crazy` | 56 / 75,237,262 | 明确许可 |
| `01M81T6RJ959EWAQWM3CZB3922` | model / character | `Library/Models/Characters/_Unknown/Tda Haku Crazy` | 56 / 75,237,262 | 明确许可 |

**文件核对**　51 种不同内容，单份集合 74,645,630 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T6RJ959EWAQWM3CZB3922`，最终目录 `Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Crazy`。

1. `merge` 保留 `01M81T6RJ959EWAQWM3CZB3922`，归并 `01M81TXDAQ2C6WGVBAKKPBSSP3`。沿用保留记录的其余字段。
2. `quarantine` `01M81TXDAQ2C6WGVBAKKPBSSP3` → `_Duplicates/kagura-review-20261124/01M81TXDAQ2C6WGVBAKKPBSSP3/Tda Haku Crazy`。
3. `rekey` `01M81T6RJ959EWAQWM3CZB3922` → `Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Crazy`。`/key` → `{"author":"InoriAruma On DeviantArt","name":"Tda Haku Crazy"}`。

**取舍**　merge 后 rekey；先将原占位的已署名副本归档，再把保留 ID 从 _Unknown 移到同名作者目录。保留其可读的 license.readme。

<a id="g03"></a>

### 03 · C · Tda Haku Long Pajama Shirt / Tda Haku Long Pajama Shirt (Tda Haku a) / Tda Haku Long Pajama Shirt (Tda inori)

**自动草稿候选。** 三份 PMX、预览和所有不同内容相同。有 (Tda Haku a) 后缀的一份有 37 条负载路径，另外两份各 36 条；它同时保留可读和乱码的左腕替换贴图目录，选择它可保留这两种路径。

**署名依据**　PMX 中署编辑者 InoriAruma On DeviantArt；coa、TDA、AGA、MikiMikuMMD 另列为协作者或素材提供者。模型名与无后缀包一致。
**建议显示名**　`Tda Haku Long Pajama Shirt`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXDB02VGE6R58FQNFHB76` | model / character | `Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Long Pajama Shirt` | 36 / 65,454,148 | 无 |
| `01M822G23DW74815TBNF9WHX6A` | model / character | `Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Long Pajama Shirt (Tda Haku a)` | 37 / 78,037,104 | 无 |
| `01M822G23MQHXPDVEGJJ01X1PM` | model / character | `Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Long Pajama Shirt (Tda inori)` | 36 / 65,454,148 | 无 |

**文件核对**　33 种不同内容，单份集合 64,864,162 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。同内容的路径数量也不同。

保留 `01M822G23DW74815TBNF9WHX6A`，最终目录 `Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Long Pajama Shirt`。

1. `merge` 保留 `01M822G23DW74815TBNF9WHX6A`，归并 `01M81TXDB02VGE6R58FQNFHB76`、`01M822G23MQHXPDVEGJJ01X1PM`。沿用保留记录的其余字段。
2. `quarantine` `01M81TXDB02VGE6R58FQNFHB76` → `_Duplicates/kagura-review-20261124/01M81TXDB02VGE6R58FQNFHB76/Tda Haku Long Pajama Shirt`。
3. `quarantine` `01M822G23MQHXPDVEGJJ01X1PM` → `_Duplicates/kagura-review-20261124/01M822G23MQHXPDVEGJJ01X1PM/Tda Haku Long Pajama Shirt (Tda inori)`。
4. `rekey` `01M822G23DW74815TBNF9WHX6A` → `Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Long Pajama Shirt`。`/key` → `{"author":"InoriAruma On DeviantArt","name":"Tda Haku Long Pajama Shirt"}`。

**取舍**　把另外两个 ID 纳入同一次 merge；无后缀目录归档后，再将保留副本 rekey 到无后缀名。

<a id="g04"></a>

### 04 · B · nac_platinaDR / 『プラチナ』-shin'in future Mix- DANCEROIDver

**需要人工确认。** 两份素材属于不同父歌，已核对两边父歌的自有负载、子素材、所有入边及对应 VMD。两边各只有这一个包，四个 VMD 均为 9,630 帧。相同的署名说明写明三位舞者、曲名和 DANCEROID 再编，nac_platinaDR 也是另一边的包装目录。保留已有 related 入边的具名父歌。

**署名依据**　読んでね.txt 落款为「なつき（～してもらいましたP）」，三位舞者为描摹对象。DANCEROID 为同一三人重编动作版本。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TAA643QGF712BJ94V8T0D` | motion_part / pack | `Library/Motions/nac_platinaDR__Unknown/pack__Unknown` | 5 / 59,526,412 | 无 |
| `01M81T9MSD68VEA7QXDGJ13K43` | motion_part / pack | `Library/Motions/『プラチナ』-shin'in future Mix- DANCEROIDver__Unknown/pack__Unknown` | 5 / 59,526,412 | 无 |

**父歌**　`01M81TAA64B1N1G0YP0MKRTZ3S` `{"title":"nac_platinaDR","composer":"_Unknown"}`；`01M81T9MSDGGQ3HGZQ5M9BGHFQ` `{"title":"『プラチナ』-shin'in future Mix- DANCEROIDver","composer":"_Unknown"}`。
**文件核对**　5 种不同内容，单份集合 59,526,412 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T9MSD68VEA7QXDGJ13K43`，最终目录 `Library/Motions/『プラチナ』-shin'in future Mix- DANCEROIDver__Unknown/pack_なつき`。

1. `merge` 保留 `01M81T9MSD68VEA7QXDGJ13K43`，归并 `01M81TAA643QGF712BJ94V8T0D`。沿用保留记录的其余字段。
2. `quarantine` `01M81TAA643QGF712BJ94V8T0D` → `_Duplicates/kagura-review-20261124/01M81TAA643QGF712BJ94V8T0D/pack__Unknown`。
3. `merge` 保留 `01M81T9MSDGGQ3HGZQ5M9BGHFQ`，归并 `01M81TAA64B1N1G0YP0MKRTZ3S`。沿用保留记录的其余字段。
4. `add_alias` `01M81T9MSDGGQ3HGZQ5M9BGHFQ` `/title_alias` → `["nac_platinaDR"]`。
5. `quarantine` `01M81TAA64B1N1G0YP0MKRTZ3S` → `_Duplicates/kagura-review-20261124/01M81TAA64B1N1G0YP0MKRTZ3S/nac_platinaDR__Unknown`。
6. `rekey` `01M81T9MSD68VEA7QXDGJ13K43` → `Library/Motions/『プラチナ』-shin'in future Mix- DANCEROIDver__Unknown/pack_なつき`。`/key` → `{"part":"pack","author":"なつき"}`。

父歌的入向引用如下。

- `01M81TAA643QGF712BJ94V8T0D` `song` → `01M81TAA64B1N1G0YP0MKRTZ3S`；来源素材随本组归档。
- `01M81T9MSD68VEA7QXDGJ13K43` `song` → `01M81T9MSDGGQ3HGZQ5M9BGHFQ`；保留原目标。
- `01M82BEPB915VES9SADW3GT18C` `related[0]` → `01M81T9MSDGGQ3HGZQ5M9BGHFQ`；保留原目标。

两首父歌均无自有负载，各只有表中这一份素材。

**人工确认**　跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。

<a id="g05"></a>

### 05 · C · CHIRNO_MMD / Montecore Cirno

**自动草稿候选。** 三种入口 cirno.pmx、cirno.pmd、nude.pmd 和整套负载均同路径同内容。保留可识别作者与角色的 Montecore Cirno，并补官方原名。

**署名依据**　Read_me.txt 及模型注释均署建模 モンテコア，原作权利署上海アリス幻樂団／ZUN，法线贴图署 Furia。标题为むちむちるの ver.1.1。
**建议显示名**　`Montecore Cirno`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXFD9QP1M8DC67QMZ3GES` | model / character | `Library/Models/Characters/モンテコア/CHIRNO_MMD` | 20 / 69,575,629 | 无 |
| `01M81TXFG9N6886TFHE4QKCKBD` | model / character | `Library/Models/Characters/モンテコア/Montecore Cirno` | 20 / 69,575,629 | 无 |

**文件核对**　19 种不同内容，单份集合 56,992,661 字节。所有相对路径与内容一致。

保留 `01M81TXFG9N6886TFHE4QKCKBD`，最终目录 `Library/Models/Characters/モンテコア/Montecore Cirno`。

1. `merge` 保留 `01M81TXFG9N6886TFHE4QKCKBD`，归并 `01M81TXFD9QP1M8DC67QMZ3GES`。字段 `/name_original` → `"むちむちるの ver.1.1"`。
2. `quarantine` `01M81TXFD9QP1M8DC67QMZ3GES` → `_Duplicates/kagura-review-20261124/01M81TXFD9QP1M8DC67QMZ3GES/CHIRNO_MMD`。

**取舍**　merge；沿用目录 Montecore Cirno，旧名 CHIRNO_MMD 可在本审查记录中检索。

<a id="g06"></a>

### 06 · A · Midas Touch

**自动草稿候选。** 两份都在歌曲「Midas Touch」的同一个 song ID 下，part 都是 pack；全部 12 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93EVFDZ5WHKF24VWFM62` | motion_part / pack | `Library/Motions/Midas Touch__Unknown/pack__Unknown` | 12 / 55,716,525 | 无 |
| `01M81T9JWN0ZFBBGF0YTGSS6CD` | motion_part / pack | `Library/Motions/Midas Touch__Unknown/pack__Unknown_KISS OF LIFE - Midas Tou` | 12 / 55,716,525 | 无 |

**父歌**　`01M81T93EV2RGVCV6Y55AAQQS4` `{"title":"Midas Touch","composer":"_Unknown"}`。
**文件核对**　12 种不同内容，单份集合 55,716,525 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93EVFDZ5WHKF24VWFM62`，最终目录 `Library/Motions/Midas Touch__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93EVFDZ5WHKF24VWFM62`，归并 `01M81T9JWN0ZFBBGF0YTGSS6CD`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JWN0ZFBBGF0YTGSS6CD` → `_Duplicates/kagura-review-20261124/01M81T9JWN0ZFBBGF0YTGSS6CD/pack__Unknown_KISS OF LIFE - Midas Tou`。
3. `rekey` `01M81T93EVFDZ5WHKF24VWFM62` → `Library/Motions/Midas Touch__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g07"></a>

### 07 · D · 京都花火祭 / 红龙舟（有内室）

**需要人工确认。** 两侧均是同一套京都花火祭舞台、竹、柱灯、酒樽和館船；红龙舟一侧在乱码父目录内重复整套内容。说明将館船列为随包可单独使用的部件，不能据此判断红龙舟是独立场景。更像按船部件另命名或误拷整包，无法区分原意。两侧无独立预览。按内容逐文件配对后，红龙舟侧去除一层乱码前缀，京都侧30/30条相对路径经CP932编码再按GBK解码，逐字符匹配红龙舟侧。

**署名依据**　两侧相同的舞台 PMX 注释与相同内容的 readme 均署建模 本生印。@宮大工P，原作出自戦国BASARA3宴，权利方 CAPCOM。京都记录仅保存 readme 路径，红龙舟记录未填 license；同一份说明列有改造许可和再配布/商业使用限制。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T0BZTCA79E6M80J5EM1Y0` | stage / stage | `Library/Stages/_Unknown/京都花火祭` | 30 / 55,437,566 | 简评 |
| `01M81T0ENB003DA5A0MV4Z9DEV` | stage / stage | `Library/Stages/_Unknown/红龙舟（有内室）` | 30 / 55,437,566 | 简评 |

**文件核对**　30 种不同内容，单份集合 55,437,566 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

两份均保留，未编列 merge 或 quarantine。

**`01M81T0BZTCA79E6M80J5EM1Y0`**

- 许可。`{"readme":"◆はじめにお読みください.txt"}`
- readme。`◆はじめにお読みください.txt`
- 预览路径。manifest 与常规预览路径均无
- 负载文件清单。`kff/01.tga`、`kff/02.tga`、`kff/03.tga`、`kff/04.tga`、`kff/05.tga`、`kff/06.tga`、`kff/07.tga`、`kff/08.tga`、`kff/09.tga`、`kff/10.tga`、`kff/11.tga`、`kff/12.tga`、`kff/13.tga`、`kff/14.tga`、`kff/15.tga`、`kff/16.tga`、`kff/17.tga`、`kff/18.tga`、`◆はじめにお読みください.txt`、`京都花火祭.pmx`、`京都花火祭_地上舞台.pmx`、`京都花火祭_水上舞台.pmx`、`曲がり竹.pmx`、`柱灯01.x`、`柱灯02.x`、`柱灯03.x`、`竹.pmx`、`花火祭.mqo`、`酒樽.x`、`館船.pmx`

**`01M81T0ENB003DA5A0MV4Z9DEV`**

- 许可。`null`
- readme。`嫗搒壴壩嵳/仧偼偠傔偵偍撉傒偔偩偝偄.txt`
- 预览路径。manifest 与常规预览路径均无
- 负载文件清单。`嫗搒壴壩嵳/kff/01.tga`、`嫗搒壴壩嵳/kff/02.tga`、`嫗搒壴壩嵳/kff/03.tga`、`嫗搒壴壩嵳/kff/04.tga`、`嫗搒壴壩嵳/kff/05.tga`、`嫗搒壴壩嵳/kff/06.tga`、`嫗搒壴壩嵳/kff/07.tga`、`嫗搒壴壩嵳/kff/08.tga`、`嫗搒壴壩嵳/kff/09.tga`、`嫗搒壴壩嵳/kff/10.tga`、`嫗搒壴壩嵳/kff/11.tga`、`嫗搒壴壩嵳/kff/12.tga`、`嫗搒壴壩嵳/kff/13.tga`、`嫗搒壴壩嵳/kff/14.tga`、`嫗搒壴壩嵳/kff/15.tga`、`嫗搒壴壩嵳/kff/16.tga`、`嫗搒壴壩嵳/kff/17.tga`、`嫗搒壴壩嵳/kff/18.tga`、`嫗搒壴壩嵳/仧偼偠傔偵偍撉傒偔偩偝偄.txt`、`嫗搒壴壩嵳/壴壩嵳.mqo`、`嫗搒壴壩嵳/娰慏.pmx`、`嫗搒壴壩嵳/嫗搒壴壩嵳.pmx`、`嫗搒壴壩嵳/嫗搒壴壩嵳_悈忋晳戜.pmx`、`嫗搒壴壩嵳/嫗搒壴壩嵳_抧忋晳戜.pmx`、`嫗搒壴壩嵳/嬋偑傝抾.pmx`、`嫗搒壴壩嵳/庰扢.x`、`嫗搒壴壩嵳/抾.pmx`、`嫗搒壴壩嵳/拰摂01.x`、`嫗搒壴壩嵳/拰摂02.x`、`嫗搒壴壩嵳/拰摂03.x`

**人工确认**　需要人工确认红龙舟是否指整包的館船单独入口，以及有无真正独立的龙舟来源。

<a id="g08"></a>

### 08 · A · Savage

**自动草稿候选。** 两份都在歌曲「Savage」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92G7D1C2Y9BT9C4WK8HB` | motion_part / pack | `Library/Motions/Savage__Unknown/pack__Unknown` | 7 / 50,970,375 | 无 |
| `01M81T9JDGEE7Z20KZT4S28DBZ` | motion_part / pack | `Library/Motions/Savage__Unknown/pack__Unknown_aespa - Savage` | 7 / 50,970,375 | 无 |

**父歌**　`01M81T92G71YDKC1BZWSKWQKXQ` `{"title":"Savage","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 50,970,375 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92G7D1C2Y9BT9C4WK8HB`，最终目录 `Library/Motions/Savage__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92G7D1C2Y9BT9C4WK8HB`，归并 `01M81T9JDGEE7Z20KZT4S28DBZ`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JDGEE7Z20KZT4S28DBZ` → `_Duplicates/kagura-review-20261124/01M81T9JDGEE7Z20KZT4S28DBZ/pack__Unknown_aespa - Savage`。
3. `rekey` `01M81T92G7D1C2Y9BT9C4WK8HB` → `Library/Motions/Savage__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g09"></a>

### 09 · A · Bon Bon Chocolat

**自动草稿候选。** 两份都在歌曲「Bon Bon Chocolat」的同一个 song ID 下，part 都是 pack；全部 5 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92ZZZZZ2X78E6YTFC6KX` | motion_part / pack | `Library/Motions/Bon Bon Chocolat__Unknown/pack__Unknown` | 5 / 48,365,860 | 无 |
| `01M81T9JVNHWDEPS4QN3AW5FNP` | motion_part / pack | `Library/Motions/Bon Bon Chocolat__Unknown/pack__Unknown_EVERGLOW - Bon Bon Choco` | 5 / 48,365,860 | 无 |

**父歌**　`01M81T92ZZ7Y437QNVPP7V1JXV` `{"title":"Bon Bon Chocolat","composer":"_Unknown"}`。
**文件核对**　5 种不同内容，单份集合 48,365,860 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92ZZZZZ2X78E6YTFC6KX`，最终目录 `Library/Motions/Bon Bon Chocolat__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92ZZZZZ2X78E6YTFC6KX`，归并 `01M81T9JVNHWDEPS4QN3AW5FNP`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JVNHWDEPS4QN3AW5FNP` → `_Duplicates/kagura-review-20261124/01M81T9JVNHWDEPS4QN3AW5FNP/pack__Unknown_EVERGLOW - Bon Bon Choco`。
3. `rekey` `01M81T92ZZZZZ2X78E6YTFC6KX` → `Library/Motions/Bon Bon Chocolat__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g10"></a>

### 10 · A · Next Level

**自动草稿候选。** 两份都在歌曲「Next Level」的同一个 song ID 下，part 都是 pack；全部 11 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92G30W0G41JVZJE139KN` | motion_part / pack | `Library/Motions/Next Level__Unknown/pack__Unknown` | 11 / 46,933,650 | 无 |
| `01M81T9JDCJ59KRGF7TA7QZMSG` | motion_part / pack | `Library/Motions/Next Level__Unknown/pack__Unknown_aespa - Next Level` | 11 / 46,933,650 | 无 |

**父歌**　`01M81T92G3QNN4WF62NR7BTXRP` `{"title":"Next Level","composer":"_Unknown"}`。
**文件核对**　11 种不同内容，单份集合 46,933,650 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92G30W0G41JVZJE139KN`，最终目录 `Library/Motions/Next Level__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92G30W0G41JVZJE139KN`，归并 `01M81T9JDCJ59KRGF7TA7QZMSG`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JDCJ59KRGF7TA7QZMSG` → `_Duplicates/kagura-review-20261124/01M81T9JDCJ59KRGF7TA7QZMSG/pack__Unknown_aespa - Next Level`。
3. `rekey` `01M81T92G30W0G41JVZJE139KN` → `Library/Motions/Next Level__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g11"></a>

### 11 · A · Snapping

**自动草稿候选。** 两份都在歌曲「Snapping」的同一个 song ID 下，part 都是 pack；全部 5 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92ZCSGTYNC7ZYJTBNJN2` | motion_part / pack | `Library/Motions/Snapping__Unknown/pack__Unknown` | 5 / 46,342,901 | 无 |
| `01M81T9JV4EMVTSNZ350RN2NMC` | motion_part / pack | `Library/Motions/Snapping__Unknown/pack__Unknown_CHUNG HA - Snapping` | 5 / 46,342,901 | 无 |

**父歌**　`01M81T92ZC8M92NDRNY50VQV9K` `{"title":"Snapping","composer":"_Unknown"}`。
**文件核对**　5 种不同内容，单份集合 46,342,901 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92ZCSGTYNC7ZYJTBNJN2`，最终目录 `Library/Motions/Snapping__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92ZCSGTYNC7ZYJTBNJN2`，归并 `01M81T9JV4EMVTSNZ350RN2NMC`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JV4EMVTSNZ350RN2NMC` → `_Duplicates/kagura-review-20261124/01M81T9JV4EMVTSNZ350RN2NMC/pack__Unknown_CHUNG HA - Snapping`。
3. `rekey` `01M81T92ZCSGTYNC7ZYJTBNJN2` → `Library/Motions/Snapping__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g12"></a>

### 12 · A · IDOL

**自动草稿候选。** 两份都在歌曲「IDOL」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93ZN0MD8MNY32YEVH8A4` | motion_part / pack | `Library/Motions/IDOL__Unknown/pack__Unknown` | 9 / 46,137,097 | 无 |
| `01M81T9KCF3VFHDYNFEMWRAC6Z` | motion_part / pack | `Library/Motions/IDOL__Unknown/pack__Unknown_YOASOBI - IDOL` | 9 / 46,137,097 | 无 |

**父歌**　`01M81T93ZN7NGAQZF3F4RQYVYC` `{"title":"IDOL","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 46,137,097 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93ZN0MD8MNY32YEVH8A4`，最终目录 `Library/Motions/IDOL__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93ZN0MD8MNY32YEVH8A4`，归并 `01M81T9KCF3VFHDYNFEMWRAC6Z`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KCF3VFHDYNFEMWRAC6Z` → `_Duplicates/kagura-review-20261124/01M81T9KCF3VFHDYNFEMWRAC6Z/pack__Unknown_YOASOBI - IDOL`。
3. `rekey` `01M81T93ZN0MD8MNY32YEVH8A4` → `Library/Motions/IDOL__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g13"></a>

### 13 · A · Spit it out

**自动草稿候选。** 两份都在歌曲「Spit it out」的同一个 song ID 下，part 都是 pack；全部 13 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93YJ14SHNBPT0321M94X` | motion_part / pack | `Library/Motions/Spit it out__Unknown/pack__Unknown` | 13 / 45,874,036 | 无 |
| `01M81T9KBKF0GZYWRCPJE842V2` | motion_part / pack | `Library/Motions/Spit it out__Unknown/pack__Unknown_SOLAR - Spit it out` | 13 / 45,874,036 | 无 |

**父歌**　`01M81T93YJHEM6Q86C4CDEEGW0` `{"title":"Spit it out","composer":"_Unknown"}`。
**文件核对**　13 种不同内容，单份集合 45,874,036 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93YJ14SHNBPT0321M94X`，最终目录 `Library/Motions/Spit it out__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93YJ14SHNBPT0321M94X`，归并 `01M81T9KBKF0GZYWRCPJE842V2`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KBKF0GZYWRCPJE842V2` → `_Duplicates/kagura-review-20261124/01M81T9KBKF0GZYWRCPJE842V2/pack__Unknown_SOLAR - Spit it out`。
3. `rekey` `01M81T93YJ14SHNBPT0321M94X` → `Library/Motions/Spit it out__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g14"></a>

### 14 · A · WHISTLE

**自动草稿候选。** 两份都在歌曲「WHISTLE」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92YG991B0KSR82AF1D11` | motion_part / pack | `Library/Motions/WHISTLE__Unknown/pack__Unknown` | 7 / 44,856,483 | 无 |
| `01M81T9JTKY21GG2M8HVJZPQQJ` | motion_part / pack | `Library/Motions/WHISTLE__Unknown/pack__Unknown_BLACKPINK - WHISTLE` | 7 / 44,856,483 | 无 |

**父歌**　`01M81T92YGF48P1D5CDS3W48HD` `{"title":"WHISTLE","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 44,856,483 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92YG991B0KSR82AF1D11`，最终目录 `Library/Motions/WHISTLE__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92YG991B0KSR82AF1D11`，归并 `01M81T9JTKY21GG2M8HVJZPQQJ`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JTKY21GG2M8HVJZPQQJ` → `_Duplicates/kagura-review-20261124/01M81T9JTKY21GG2M8HVJZPQQJ/pack__Unknown_BLACKPINK - WHISTLE`。
3. `rekey` `01M81T92YG991B0KSR82AF1D11` → `Library/Motions/WHISTLE__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g15"></a>

### 15 · A · I CAN'T STOP ME

**自动草稿候选。** 两份都在歌曲「I CAN'T STOP ME」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93Z6V901WFR88HHKFC8N` | motion_part / pack | `Library/Motions/I CAN'T STOP ME__Unknown/pack__Unknown` | 7 / 44,650,142 | 无 |
| `01M81T9KC19FVNDBWKY9AVGZNT` | motion_part / pack | `Library/Motions/I CAN'T STOP ME__Unknown/pack__Unknown_TWICE - I CAN'T STOP ME` | 7 / 44,650,142 | 无 |

**父歌**　`01M81T93Z6XZRXR7T3M1VY53M5` `{"title":"I CAN'T STOP ME","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 44,650,142 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93Z6V901WFR88HHKFC8N`，最终目录 `Library/Motions/I CAN'T STOP ME__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93Z6V901WFR88HHKFC8N`，归并 `01M81T9KC19FVNDBWKY9AVGZNT`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KC19FVNDBWKY9AVGZNT` → `_Duplicates/kagura-review-20261124/01M81T9KC19FVNDBWKY9AVGZNT/pack__Unknown_TWICE - I CAN'T STOP ME`。
3. `rekey` `01M81T93Z6V901WFR88HHKFC8N` → `Library/Motions/I CAN'T STOP ME__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g16"></a>

### 16 · A · SHAKE IT

**自动草稿候选。** 两份都在歌曲「SHAKE IT」的同一个 song ID 下，part 都是 pack；全部 5 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93YEH1CRT1QF65A5ASZ5` | motion_part / pack | `Library/Motions/SHAKE IT__Unknown/pack__Unknown` | 5 / 44,329,305 | 无 |
| `01M81T9KBGJDMKZD83SNDQ6P20` | motion_part / pack | `Library/Motions/SHAKE IT__Unknown/pack__Unknown_SISTAR - SHAKE IT` | 5 / 44,329,305 | 无 |

**父歌**　`01M81T93YETRX27XBZV2FZBEDT` `{"title":"SHAKE IT","composer":"_Unknown"}`。
**文件核对**　5 种不同内容，单份集合 44,329,305 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93YEH1CRT1QF65A5ASZ5`，最终目录 `Library/Motions/SHAKE IT__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93YEH1CRT1QF65A5ASZ5`，归并 `01M81T9KBGJDMKZD83SNDQ6P20`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KBGJDMKZD83SNDQ6P20` → `_Duplicates/kagura-review-20261124/01M81T9KBGJDMKZD83SNDQ6P20/pack__Unknown_SISTAR - SHAKE IT`。
3. `rekey` `01M81T93YEH1CRT1QF65A5ASZ5` → `Library/Motions/SHAKE IT__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g17"></a>

### 17 · A · SECRET NUMBER-Got That Boom

**自动草稿候选。** 两份都在歌曲「SECRET NUMBER-Got That Boom」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93YBCF81A92497KRAQMN` | motion_part / pack | `Library/Motions/SECRET NUMBER-Got That Boom__Unknown/pack__Unknown` | 7 / 44,296,506 | 无 |
| `01M81T9KBCN606CXS31T44VWN3` | motion_part / pack | `Library/Motions/SECRET NUMBER-Got That Boom__Unknown/pack__Unknown_SECRET NUMBER-Got That B` | 7 / 44,296,506 | 无 |

**父歌**　`01M81T93YBGEA75Z828QSY7YFX` `{"title":"SECRET NUMBER-Got That Boom","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 44,296,506 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93YBCF81A92497KRAQMN`，最终目录 `Library/Motions/SECRET NUMBER-Got That Boom__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93YBCF81A92497KRAQMN`，归并 `01M81T9KBCN606CXS31T44VWN3`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KBCN606CXS31T44VWN3` → `_Duplicates/kagura-review-20261124/01M81T9KBCN606CXS31T44VWN3/pack__Unknown_SECRET NUMBER-Got That B`。
3. `rekey` `01M81T93YBCF81A92497KRAQMN` → `Library/Motions/SECRET NUMBER-Got That Boom__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g18"></a>

### 18 · B · Calabria2008~ / 女主播

**需要人工确认。** 两份素材属于不同父歌，已核对两边父歌的自有负载、子素材、所有入边及对应 VMD。两份包的 WAV、镜头和动作逐内容一致，两个父歌各只有这一份素材，没有独立音频。WAV 名指向 Enur feat Natasja 的 Calabria 2007；女主播疑为用途或来源标签，Calabria2008~ 的年份也与文件名冲突。建议将这两个空壳记录合并到 Calabria 一侧，由人工核准显示曲名和年份；不把「女主播」写成正式别名。

**署名依据**　无动作署名文档。WAV 文件名标出的是歌曲和演唱者。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T96SKGKX94CJ42AR4G915` | motion_part / pack | `Library/Motions/Calabria2008~__Unknown/pack__Unknown` | 3 / 44,272,351 | 无 |
| `01M81TAS0PS24H5THE0NCHE0D1` | motion_part / pack | `Library/Motions/女主播__Unknown/pack__Unknown` | 3 / 44,272,351 | 无 |

**父歌**　`01M81T96SKHK9XFSX6QK3FFAWN` `{"title":"Calabria2008~","composer":"_Unknown"}`；`01M81TAS0PRKC4K7K6KZRYT14X` `{"title":"女主播","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 44,272,351 字节。所有相对路径与内容一致。

保留 `01M81T96SKGKX94CJ42AR4G915`，最终目录 `Library/Motions/Calabria 2007__Unknown/pack__Unknown`。

1. `merge` 保留 `01M81T96SKGKX94CJ42AR4G915`，归并 `01M81TAS0PS24H5THE0NCHE0D1`。沿用保留记录的其余字段。
2. `quarantine` `01M81TAS0PS24H5THE0NCHE0D1` → `_Duplicates/kagura-review-20261124/01M81TAS0PS24H5THE0NCHE0D1/pack__Unknown`。
3. `merge` 保留 `01M81T96SKHK9XFSX6QK3FFAWN`，归并 `01M81TAS0PRKC4K7K6KZRYT14X`。沿用保留记录的其余字段。
4. `add_alias` `01M81T96SKHK9XFSX6QK3FFAWN` `/title_alias` → `["Calabria2008~"]`。
5. `rekey` `01M81T96SKHK9XFSX6QK3FFAWN` → `Library/Motions/Calabria 2007__Unknown`。`/key` → `{"title":"Calabria 2007","composer":"_Unknown"}`。
6. `quarantine` `01M81TAS0PRKC4K7K6KZRYT14X` → `_Duplicates/kagura-review-20261124/01M81TAS0PRKC4K7K6KZRYT14X/女主播__Unknown`。

父歌的入向引用如下。

- `01M81T96SKGKX94CJ42AR4G915` `song` → `01M81T96SKHK9XFSX6QK3FFAWN`；保留原目标。
- `01M81TAS0PS24H5THE0NCHE0D1` `song` → `01M81TAS0PRKC4K7K6KZRYT14X`；来源素材随本组归档。

两首父歌均无自有负载，各只有表中这一份素材。

**人工确认**　现行父歌名 Calabria2008~ 和音频文件名 Calabria 2007 年份相差一年，需确认显示曲名；同一 WAV 与 VMD 已核实。

<a id="g19"></a>

### 19 · A · Good-night Kiss

**自动草稿候选。** 两份都在歌曲「Good-night Kiss」的同一个 song ID 下，part 都是 pack；全部 5 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93EH4337YG7CCWCP5M7K` | motion_part / pack | `Library/Motions/Good-night Kiss__Unknown/pack__Unknown` | 5 / 43,620,206 | 无 |
| `01M81T9JWBDD7TG2Z47JGAR5YK` | motion_part / pack | `Library/Motions/Good-night Kiss__Unknown/pack__Unknown_Jun Hyo Seong - Good-nig` | 5 / 43,620,206 | 无 |

**父歌**　`01M81T93EH33G8VJQBZFT5XX8A` `{"title":"Good-night Kiss","composer":"_Unknown"}`。
**文件核对**　5 种不同内容，单份集合 43,620,206 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93EH4337YG7CCWCP5M7K`，最终目录 `Library/Motions/Good-night Kiss__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93EH4337YG7CCWCP5M7K`，归并 `01M81T9JWBDD7TG2Z47JGAR5YK`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JWBDD7TG2Z47JGAR5YK` → `_Duplicates/kagura-review-20261124/01M81T9JWBDD7TG2Z47JGAR5YK/pack__Unknown_Jun Hyo Seong - Good-nig`。
3. `rekey` `01M81T93EH4337YG7CCWCP5M7K` → `Library/Motions/Good-night Kiss__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g20"></a>

### 20 · A · PUPPET SHOW

**自动草稿候选。** 两份都在歌曲「PUPPET SHOW」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93ZHB1VJTDW1AFWAW5AN` | motion_part / pack | `Library/Motions/PUPPET SHOW__Unknown/pack__Unknown` | 9 / 43,540,546 | 无 |
| `01M81T9KCBHFASSFCA7CAYM87M` | motion_part / pack | `Library/Motions/PUPPET SHOW__Unknown/pack__Unknown_XG - PUPPET SHOW` | 9 / 43,540,546 | 无 |

**父歌**　`01M81T93ZHR89316C21J3HZC78` `{"title":"PUPPET SHOW","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 43,540,546 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93ZHB1VJTDW1AFWAW5AN`，最终目录 `Library/Motions/PUPPET SHOW__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93ZHB1VJTDW1AFWAW5AN`，归并 `01M81T9KCBHFASSFCA7CAYM87M`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KCBHFASSFCA7CAYM87M` → `_Duplicates/kagura-review-20261124/01M81T9KCBHFASSFCA7CAYM87M/pack__Unknown_XG - PUPPET SHOW`。
3. `rekey` `01M81T93ZHB1VJTDW1AFWAW5AN` → `Library/Motions/PUPPET SHOW__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g21"></a>

### 21 · A · Miniskirt (Queendom ver.

**自动草稿候选。** 两份都在歌曲「Miniskirt (Queendom ver.」的同一个 song ID 下，part 都是 pack；全部 15 种文件内容相同。两边 tested_with 相同；保留有原地 Chair.vmd 路径的 note，另一侧相同说明采用外层包装目录。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92GNYQX08KR259R2NF10` | motion_part / pack | `Library/Motions/Miniskirt (Queendom ver__Unknown/pack__Unknown` | 15 / 43,515,282 | 简评 |
| `01M81T9JTD82SHG5FT6SGEM2QE` | motion_part / pack | `Library/Motions/Miniskirt (Queendom ver__Unknown/pack__Unknown_AOA - Miniskirt (Queendo` | 15 / 43,515,282 | 简评 |

**父歌**　`01M81T92GNVSRF4H8XCVXE211P` `{"title":"Miniskirt (Queendom ver.","composer":"_Unknown"}`。
**文件核对**　15 种不同内容，单份集合 43,515,282 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92GNYQX08KR259R2NF10`，最终目录 `Library/Motions/Miniskirt (Queendom ver__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92GNYQX08KR259R2NF10`，归并 `01M81T9JTD82SHG5FT6SGEM2QE`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JTD82SHG5FT6SGEM2QE` → `_Duplicates/kagura-review-20261124/01M81T9JTD82SHG5FT6SGEM2QE/pack__Unknown_AOA - Miniskirt (Queendo`。
3. `rekey` `01M81T92GNYQX08KR259R2NF10` → `Library/Motions/Miniskirt (Queendom ver__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g22"></a>

### 22 · A · A

**自动草稿候选。** 两份都在歌曲「A」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93XXHVS6HZZN84AN85X0` | motion_part / pack | `Library/Motions/A__Unknown/pack__Unknown` | 9 / 43,190,315 | 无 |
| `01M81T9KB2N4SPRY2N5ZR9102V` | motion_part / pack | `Library/Motions/A__Unknown/pack__Unknown_Rainbow - A` | 9 / 43,190,315 | 无 |

**父歌**　`01M81T93XX0T0HR63H1S08YJJQ` `{"title":"A","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 43,190,315 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93XXHVS6HZZN84AN85X0`，最终目录 `Library/Motions/A__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93XXHVS6HZZN84AN85X0`，归并 `01M81T9KB2N4SPRY2N5ZR9102V`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KB2N4SPRY2N5ZR9102V` → `_Duplicates/kagura-review-20261124/01M81T9KB2N4SPRY2N5ZR9102V/pack__Unknown_Rainbow - A`。
3. `rekey` `01M81T93XXHVS6HZZN84AN85X0` → `Library/Motions/A__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g23"></a>

### 23 · A · PLAY

**自动草稿候选。** 两份都在歌曲「PLAY」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92Z9W9FRA6Y8DTSEMMJP` | motion_part / pack | `Library/Motions/PLAY__Unknown/pack__Unknown` | 7 / 43,031,420 | 无 |
| `01M81T9JV10GQKMTQJW7794HHG` | motion_part / pack | `Library/Motions/PLAY__Unknown/pack__Unknown_CHUNG HA - PLAY` | 7 / 43,031,420 | 无 |

**父歌**　`01M81T92Z9D1HA7T7CVYV2K58P` `{"title":"PLAY","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 43,031,420 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92Z9W9FRA6Y8DTSEMMJP`，最终目录 `Library/Motions/PLAY__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92Z9W9FRA6Y8DTSEMMJP`，归并 `01M81T9JV10GQKMTQJW7794HHG`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JV10GQKMTQJW7794HHG` → `_Duplicates/kagura-review-20261124/01M81T9JV10GQKMTQJW7794HHG/pack__Unknown_CHUNG HA - PLAY`。
3. `rekey` `01M81T92Z9W9FRA6Y8DTSEMMJP` → `Library/Motions/PLAY__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g24"></a>

### 24 · A · PANDORA

**自动草稿候选。** 两份都在歌曲「PANDORA」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留有 10 个实际文件、8 条声明的那份。Camera2.vmd 是同内容另一文件名，对侧只有 9 个文件、7 条声明。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93FK1ATPJQQXQ7JN1J59` | motion_part / pack | `Library/Motions/PANDORA__Unknown/pack__Unknown` | 10 / 42,349,557 | 无 |
| `01M81T9JX7RE875STXH1BTSVZE` | motion_part / pack | `Library/Motions/PANDORA__Unknown/pack__Unknown_MAVE - PANDORA` | 9 / 42,333,196 | 无 |

**父歌**　`01M81T93FKNGCX6GQ0DZ47CNTY` `{"title":"PANDORA","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 42,333,196 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。同内容的路径数量也不同。

保留 `01M81T93FK1ATPJQQXQ7JN1J59`，最终目录 `Library/Motions/PANDORA__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93FK1ATPJQQXQ7JN1J59`，归并 `01M81T9JX7RE875STXH1BTSVZE`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JX7RE875STXH1BTSVZE` → `_Duplicates/kagura-review-20261124/01M81T9JX7RE875STXH1BTSVZE/pack__Unknown_MAVE - PANDORA`。
3. `rekey` `01M81T93FK1ATPJQQXQ7JN1J59` → `Library/Motions/PANDORA__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g25"></a>

### 25 · A · Yume Yume Live

**需要人工确认。** 两份都在歌曲「Yume Yume Live」的同一个 song ID 下，part 都是 pack；全部 3 种文件内容相同。同一哈希的文件在两侧分别标 face 和 lip。同一侧的 Lip Only.vmd 实测有 2,204 个表情关键帧：非零键中あ 617、お 245、い 193、う 120，另有まばたき 4，确实兼有口型与眨眼。每个 file 只有一个 role，保留两种角色证据，需人工审定后再合并。 建议保留原名 Lip Only.vmd 以及现有 lip 标注，补记另一侧的 face 标注和 4 个眨眼键；执行等待角色判定。

**署名依据**　两包均无 readme 或能归属给动作作者的文件名。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9EP1A4V1VCFRF7C1END8` | motion_part / pack | `Library/Motions/Yume Yume Live__Unknown/pack__Unknown` | 3 / 41,957,659 | 无 |
| `01M81TAD2HR0G99H6ASXVBT9ME` | motion_part / pack | `Library/Motions/Yume Yume Live__Unknown/pack__Unknown_Yume Yume Live Motion` | 3 / 41,957,659 | 无 |

**父歌**　`01M81T9EP1ZJ9E8W3XWB50PAAR` `{"title":"Yume Yume Live","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 41,957,659 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81TAD2HR0G99H6ASXVBT9ME`，最终目录 `Library/Motions/Yume Yume Live__Unknown/pack__Unknown_Yume Yume Live Motion`。

1. `merge` 保留 `01M81TAD2HR0G99H6ASXVBT9ME`，归并 `01M81T9EP1A4V1VCFRF7C1END8`。字段 `/note` → `"Lip Only.vmd 同时包含口型与眨眼；另一份记录将同一文件标为 face。"`。
2. `quarantine` `01M81T9EP1A4V1VCFRF7C1END8` → `_Duplicates/kagura-review-20261124/01M81T9EP1A4V1VCFRF7C1END8/pack__Unknown`。

**人工确认**　提交草稿前人工确定该文件单值 role 采用 lip（依原文件名）还是 face（同时含眨眼），保留另一描述。

<a id="g26"></a>

### 26 · A · PRIVACY

**自动草稿候选。** 两份都在歌曲「PRIVACY」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93Y88A16JJPQ6BH6N1WG` | motion_part / pack | `Library/Motions/PRIVACY__Unknown/pack__Unknown` | 7 / 41,426,158 | 无 |
| `01M81T9KB9G1X012VXKZRC9A7J` | motion_part / pack | `Library/Motions/PRIVACY__Unknown/pack__Unknown_SECRET NUMBER - PRIVACY` | 7 / 41,426,158 | 无 |

**父歌**　`01M81T93Y80SGKB52065134VSH` `{"title":"PRIVACY","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 41,426,158 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93Y88A16JJPQ6BH6N1WG`，最终目录 `Library/Motions/PRIVACY__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93Y88A16JJPQ6BH6N1WG`，归并 `01M81T9KB9G1X012VXKZRC9A7J`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KB9G1X012VXKZRC9A7J` → `_Duplicates/kagura-review-20261124/01M81T9KB9G1X012VXKZRC9A7J/pack__Unknown_SECRET NUMBER - PRIVACY`。
3. `rekey` `01M81T93Y88A16JJPQ6BH6N1WG` → `Library/Motions/PRIVACY__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g27"></a>

### 27 · A · Carousel

**自动草稿候选。** 两份都在歌曲「Carousel」的同一个 song ID 下，part 都是 pack；全部 3 种文件内容相同。保留音频曲名和表演者写在原文件名的一侧，两份的镜头和主体角色声明按哈希一致。

**署名依据**　具名 WAV 将 Melanie Martinez 和歌曲关联，不能据此填写动作作者。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T96T7M3219ZQWP8GA0ZP9` | motion_part / pack | `Library/Motions/Carousel__Unknown/pack__Unknown` | 3 / 41,059,817 | 无 |
| `01M81TA5XX0P8SBBXSGV03QXQ1` | motion_part / pack | `Library/Motions/Carousel__Unknown/pack__Unknown_Carousel` | 3 / 41,059,817 | 无 |

**父歌**　`01M81T96T7SXW04KVCZ5YB5CKA` `{"title":"Carousel","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 41,059,817 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81TA5XX0P8SBBXSGV03QXQ1`，最终目录 `Library/Motions/Carousel__Unknown/pack__Unknown_Carousel`。

1. `merge` 保留 `01M81TA5XX0P8SBBXSGV03QXQ1`，归并 `01M81T96T7M3219ZQWP8GA0ZP9`。沿用保留记录的其余字段。
2. `quarantine` `01M81T96T7M3219ZQWP8GA0ZP9` → `_Duplicates/kagura-review-20261124/01M81T96T7M3219ZQWP8GA0ZP9/pack__Unknown`。

<a id="g28"></a>

### 28 · A · Teddy Bear

**自动草稿候选。** 两份都在歌曲「Teddy Bear」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93YQ1SEJDKF2J2PFWTV8` | motion_part / pack | `Library/Motions/Teddy Bear__Unknown/pack__Unknown` | 9 / 40,946,925 | 无 |
| `01M81T9KBRGENEPAPCPWEWWY4S` | motion_part / pack | `Library/Motions/Teddy Bear__Unknown/pack__Unknown_STAYC - Teddy Bear` | 9 / 40,946,925 | 无 |

**父歌**　`01M81T93YQP5YMFZJXCC81TDTT` `{"title":"Teddy Bear","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 40,946,925 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93YQ1SEJDKF2J2PFWTV8`，最终目录 `Library/Motions/Teddy Bear__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93YQ1SEJDKF2J2PFWTV8`，归并 `01M81T9KBRGENEPAPCPWEWWY4S`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KBRGENEPAPCPWEWWY4S` → `_Duplicates/kagura-review-20261124/01M81T9KBRGENEPAPCPWEWWY4S/pack__Unknown_STAYC - Teddy Bear`。
3. `rekey` `01M81T93YQ1SEJDKF2J2PFWTV8` → `Library/Motions/Teddy Bear__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g29"></a>

### 29 · A · Eve, Psyche & The Bluebeard's wife

**自动草稿候选。** 两份都在歌曲「Eve, Psyche & The Bluebeard's wife」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93F4GJK6Y8FSE7Y06NSF` | motion_part / pack | `Library/Motions/Eve, Psyche & The Bluebeard's wife__Unknown/pack__Unknown` | 9 / 40,780,723 | 无 |
| `01M81T9JWSHK4C9FCNSPCA1YPZ` | motion_part / pack | `Library/Motions/Eve, Psyche & The Bluebeard's wife__Unknown/pack__Unknown_LE SSERAFIM - Eve, Psych` | 9 / 40,780,723 | 无 |

**父歌**　`01M81T93F4BWVNEVG6XBT708ZH` `{"title":"Eve, Psyche & The Bluebeard's wife","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 40,780,723 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93F4GJK6Y8FSE7Y06NSF`，最终目录 `Library/Motions/Eve, Psyche & The Bluebeard's wife__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93F4GJK6Y8FSE7Y06NSF`，归并 `01M81T9JWSHK4C9FCNSPCA1YPZ`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JWSHK4C9FCNSPCA1YPZ` → `_Duplicates/kagura-review-20261124/01M81T9JWSHK4C9FCNSPCA1YPZ/pack__Unknown_LE SSERAFIM - Eve, Psych`。
3. `rekey` `01M81T93F4GJK6Y8FSE7Y06NSF` → `Library/Motions/Eve, Psyche & The Bluebeard's wife__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g30"></a>

### 30 · A · Heart Attack

**自动草稿候选。** 两份都在歌曲「Heart Attack」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92GHKTC8K4GXFJCEGZDB` | motion_part / pack | `Library/Motions/Heart Attack__Unknown/pack__Unknown` | 7 / 40,659,455 | 无 |
| `01M81T9JDRTPHMKQ5PE0JNJS80` | motion_part / pack | `Library/Motions/Heart Attack__Unknown/pack__Unknown_AOA - Heart Attack` | 7 / 40,659,455 | 无 |

**父歌**　`01M81T92GHJ24D81B0CMNHAM30` `{"title":"Heart Attack","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 40,659,455 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92GHKTC8K4GXFJCEGZDB`，最终目录 `Library/Motions/Heart Attack__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92GHKTC8K4GXFJCEGZDB`，归并 `01M81T9JDRTPHMKQ5PE0JNJS80`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JDRTPHMKQ5PE0JNJS80` → `_Duplicates/kagura-review-20261124/01M81T9JDRTPHMKQ5PE0JNJS80/pack__Unknown_AOA - Heart Attack`。
3. `rekey` `01M81T92GHKTC8K4GXFJCEGZDB` → `Library/Motions/Heart Attack__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g31"></a>

### 31 · A · Sparkling

**自动草稿候选。** 两份都在歌曲「Sparkling」的同一个 song ID 下，part 都是 pack；全部 13 种文件内容相同。主包保留两条 tested_with 和与子轨对应的 note；key.author=Cloud9 与附件署名相符，而根目录动作说明指定 NatsumiSan。将主包 readme 指向 Readme-Rules.txt；在 note 留下 Cloud9 的附件署名和原附件说明位置。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。 Accessories/readme.txt 署名 Cloud9，范围是附件；保留该署名作为附件信息。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92ZFZTPMDJRNKJAQPA78` | motion_part / pack | `Library/Motions/Sparkling__Unknown/pack_Cloud9` | 13 / 40,356,660 | 简评 |
| `01M81T9JV7TV747RE4CHDADTD7` | motion_part / pack | `Library/Motions/Sparkling__Unknown/pack_Cloud9_CHUNG HA - Sparkling` | 13 / 40,356,660 | 简评 |

**父歌**　`01M81T92ZF12HPM3G25GJY00Z5` `{"title":"Sparkling","composer":"_Unknown"}`。
**文件核对**　13 种不同内容，单份集合 40,356,660 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92ZFZTPMDJRNKJAQPA78`，最终目录 `Library/Motions/Sparkling__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92ZFZTPMDJRNKJAQPA78`，归并 `01M81T9JV7TV747RE4CHDADTD7`。字段 `/license` → `{"readme":"Readme-Rules.txt"}`；`/note` → `"同版配套资产「くらげ（Sparkling）」ID 01M83V8TAY3SF03ZSD7S9E18R5。 对应原子轨：Jellyfish 1 Motion.vmd、Jellyfish 2 Motion.vmd、Jellyfish 3 Motion.vmd。\n同版配套资产「Ordinary Conch Shell」ID 01M83V8TC4PHT2NTRW6X02SEAH。 对应原子轨：Shell Motion.vmd。\n动作署名 NatsumiSan。随包附件署名 Cloud9，附件使用说明见 Accessories/readme.txt。"`。
2. `quarantine` `01M81T9JV7TV747RE4CHDADTD7` → `_Duplicates/kagura-review-20261124/01M81T9JV7TV747RE4CHDADTD7/pack_Cloud9_CHUNG HA - Sparkling`。
3. `rekey` `01M81T92ZFZTPMDJRNKJAQPA78` → `Library/Motions/Sparkling__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g32"></a>

### 32 · A · MISTER

**自动草稿候选。** 两份都在歌曲「MISTER」的同一个 song ID 下，part 都是 pack；全部 5 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93EMYE493T6WKJCQGF68` | motion_part / pack | `Library/Motions/MISTER__Unknown/pack__Unknown` | 5 / 40,126,766 | 无 |
| `01M81T9JWE8E7SGAJZE9FENBKC` | motion_part / pack | `Library/Motions/MISTER__Unknown/pack__Unknown_KARA - MISTER` | 5 / 40,126,766 | 无 |

**父歌**　`01M81T93EMSC7S9615W07G4SQP` `{"title":"MISTER","composer":"_Unknown"}`。
**文件核对**　5 种不同内容，单份集合 40,126,766 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93EMYE493T6WKJCQGF68`，最终目录 `Library/Motions/MISTER__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93EMYE493T6WKJCQGF68`，归并 `01M81T9JWE8E7SGAJZE9FENBKC`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JWE8E7SGAJZE9FENBKC` → `_Duplicates/kagura-review-20261124/01M81T9JWE8E7SGAJZE9FENBKC/pack__Unknown_KARA - MISTER`。
3. `rekey` `01M81T93EMYE493T6WKJCQGF68` → `Library/Motions/MISTER__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g33"></a>

### 33 · A · 蝶のように舞いーニョ EXTREME VS. FULL BBA

**自动草稿候选。** 两份都在歌曲「蝶のように舞いーニョ EXTREME VS. FULL BBA」的同一个 song ID 下，part 都是 pack；全部 12 种文件内容相同。保留日文文件名较可读的一侧；转入来源记录的 name 和原目录说明，其 added 仅作为来源日期记入 note，保持所留 ID 的建档字段原状。两侧声明的角色和目标模型可按哈希一一对上。

**署名依据**　两侧均无动作署名；后一份 note 是来源路径。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9HYN0S4EESRW3MQXMSB4` | motion_part / pack | `Library/Motions/蝶のように舞いーニョ EXTREME VS. FULL BBA__Unknown/pack__Unknown` | 12 / 39,776,230 | 无 |
| `01M829XS2DAKV0M9RFRP9WQ3YR` | motion_part / pack | `Library/Motions/蝶のように舞いーニョ EXTREME VS. FULL BBA__Unknown/pack__Unknown_蝶のように舞いーニョ EXTREME VS. F` | 12 / 39,776,230 | 简评 |

**父歌**　`01M81T9HYNQCMJFZXTCQTJCDZ5` `{"title":"蝶のように舞いーニョ EXTREME VS. FULL BBA","composer":"_Unknown"}`。
**文件核对**　12 种不同内容，单份集合 39,776,230 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T9HYN0S4EESRW3MQXMSB4`，最终目录 `Library/Motions/蝶のように舞いーニョ EXTREME VS. FULL BBA__Unknown/pack__Unknown`。

1. `merge` 保留 `01M81T9HYN0S4EESRW3MQXMSB4`，归并 `01M829XS2DAKV0M9RFRP9WQ3YR`。字段 `/name` → `"蝶のように舞いーニョ EXTREME VS. FULL BBA"`；`/note` → `"原目录 动作/#他人包/MMD/杂项VMD/[A Motion collection]/dance/蝶のように舞いーニョ EXTREME VS. FULL BBA。原来源记录的 added 为 2026-11-23。"`。
2. `quarantine` `01M829XS2DAKV0M9RFRP9WQ3YR` → `_Duplicates/kagura-review-20261124/01M829XS2DAKV0M9RFRP9WQ3YR/pack__Unknown_蝶のように舞いーニョ EXTREME VS. F`。

<a id="g34"></a>

### 34 · A · WA DA DA

**自动草稿候选。** 两份都在歌曲「WA DA DA」的同一个 song ID 下，part 都是 pack；全部 10 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93EQEC6W1Z136GSZE3A3` | motion_part / pack | `Library/Motions/WA DA DA__Unknown/pack__Unknown` | 10 / 39,761,013 | 无 |
| `01M81T9JWHFEBNWD6K6FQ493QX` | motion_part / pack | `Library/Motions/WA DA DA__Unknown/pack__Unknown_Kep1er - WA DA DA` | 10 / 39,761,013 | 无 |

**父歌**　`01M81T93EQY52GXPRY1YMEN7CJ` `{"title":"WA DA DA","composer":"_Unknown"}`。
**文件核对**　10 种不同内容，单份集合 39,761,013 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93EQEC6W1Z136GSZE3A3`，最终目录 `Library/Motions/WA DA DA__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93EQEC6W1Z136GSZE3A3`，归并 `01M81T9JWHFEBNWD6K6FQ493QX`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JWHFEBNWD6K6FQ493QX` → `_Duplicates/kagura-review-20261124/01M81T9JWHFEBNWD6K6FQ493QX/pack__Unknown_Kep1er - WA DA DA`。
3. `rekey` `01M81T93EQEC6W1Z136GSZE3A3` → `Library/Motions/WA DA DA__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g35"></a>

### 35 · A · MAISON

**自动草稿候选。** 两份都在歌曲「MAISON」的同一个 song ID 下，part 都是 pack；全部 11 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92ZQJ7F84G3WS2N0NHH2` | motion_part / pack | `Library/Motions/MAISON__Unknown/pack__Unknown` | 11 / 39,731,179 | 无 |
| `01M81T9JVFVPZVQP37DN135K4S` | motion_part / pack | `Library/Motions/MAISON__Unknown/pack__Unknown_Dreamcatcher - MAISON` | 11 / 39,731,179 | 无 |

**父歌**　`01M81T92ZQQMQ9Z8Q733SKNNBD` `{"title":"MAISON","composer":"_Unknown"}`。
**文件核对**　11 种不同内容，单份集合 39,731,179 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92ZQJ7F84G3WS2N0NHH2`，最终目录 `Library/Motions/MAISON__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92ZQJ7F84G3WS2N0NHH2`，归并 `01M81T9JVFVPZVQP37DN135K4S`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JVFVPZVQP37DN135K4S` → `_Duplicates/kagura-review-20261124/01M81T9JVFVPZVQP37DN135K4S/pack__Unknown_Dreamcatcher - MAISON`。
3. `rekey` `01M81T92ZQJ7F84G3WS2N0NHH2` → `Library/Motions/MAISON__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g36"></a>

### 36 · A · ILLELLA

**自动草稿候选。** 两份都在歌曲「ILLELLA」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93FFRYE6KC1DAFNPHYY4` | motion_part / pack | `Library/Motions/ILLELLA__Unknown/pack__Unknown` | 7 / 39,718,185 | 无 |
| `01M81T9JX2FBB9JYTKZSKCGDPG` | motion_part / pack | `Library/Motions/ILLELLA__Unknown/pack__Unknown_MAMAMOO - ILLELLA` | 7 / 39,718,185 | 无 |

**父歌**　`01M81T93FFAZ6GV0HZZKRK03N9` `{"title":"ILLELLA","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 39,718,185 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93FFRYE6KC1DAFNPHYY4`，最终目录 `Library/Motions/ILLELLA__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93FFRYE6KC1DAFNPHYY4`，归并 `01M81T9JX2FBB9JYTKZSKCGDPG`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JX2FBB9JYTKZSKCGDPG` → `_Duplicates/kagura-review-20261124/01M81T9JX2FBB9JYTKZSKCGDPG/pack__Unknown_MAMAMOO - ILLELLA`。
3. `rekey` `01M81T93FFRYE6KC1DAFNPHYY4` → `Library/Motions/ILLELLA__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g37"></a>

### 37 · A · Layin' Low

**自动草稿候选。** 两份都在歌曲「Layin' Low」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T930KF0RPC0FP0RJVSJ33` | motion_part / pack | `Library/Motions/Layin' Low__Unknown/pack__Unknown` | 9 / 39,468,045 | 无 |
| `01M81T9JW55NJY86JGT7HSAF95` | motion_part / pack | `Library/Motions/Layin' Low__Unknown/pack__Unknown_HYOLYN - Layin' Low` | 9 / 39,468,045 | 无 |

**父歌**　`01M81T930KQT8QQGFDS4D8TAM9` `{"title":"Layin' Low","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 39,468,045 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T930KF0RPC0FP0RJVSJ33`，最终目录 `Library/Motions/Layin' Low__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T930KF0RPC0FP0RJVSJ33`，归并 `01M81T9JW55NJY86JGT7HSAF95`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JW55NJY86JGT7HSAF95` → `_Duplicates/kagura-review-20261124/01M81T9JW55NJY86JGT7HSAF95/pack__Unknown_HYOLYN - Layin' Low`。
3. `rekey` `01M81T930KF0RPC0FP0RJVSJ33` → `Library/Motions/Layin' Low__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g38"></a>

### 38 · C · TDA式改变XXX - BY皮皮蜥 / TDA式改变XXX BY皮皮蜥

**自动草稿候选。** 两份 key 只差连字符和空格，31 条负载的相对路径和哈希逐条相同。主 PMX 文件名比旧目录占位词 XXX 更能识别这份模型。

**署名依据**　A神秘的文本2019.02.23.txt 与 PMX 均署模型编辑 皮皮蜥。TDA、EndiBell 等是分部件来源；PMX 内部名来自素体。
**建议显示名**　`TDA式改变弱音1`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXMDP1E0VH7QB5PYDDKT1` | model / character | `Library/Models/Characters/_Unknown/TDA式改变XXX - BY皮皮蜥` | 31 / 39,325,410 | 无 |
| `01M81TXMDWTB60GVWHGPW037P4` | model / character | `Library/Models/Characters/_Unknown/TDA式改变XXX BY皮皮蜥` | 31 / 39,325,410 | 无 |

**文件核对**　31 种不同内容，单份集合 39,325,410 字节。所有相对路径与内容一致。

保留 `01M81TXMDP1E0VH7QB5PYDDKT1`，最终目录 `Library/Models/Characters/皮皮蜥/TDA式改变弱音1`。

1. `merge` 保留 `01M81TXMDP1E0VH7QB5PYDDKT1`，归并 `01M81TXMDWTB60GVWHGPW037P4`。沿用保留记录的其余字段。
2. `quarantine` `01M81TXMDWTB60GVWHGPW037P4` → `_Duplicates/kagura-review-20261124/01M81TXMDWTB60GVWHGPW037P4/TDA式改变XXX BY皮皮蜥`。
3. `rekey` `01M81TXMDP1E0VH7QB5PYDDKT1` → `Library/Models/Characters/皮皮蜥/TDA式改变弱音1`。`/key` → `{"author":"皮皮蜥","name":"TDA式改变弱音1"}`。

**取舍**　merge 后 rekey 至最终编辑者与主 PMX 文件名。保留原预览。

<a id="g39"></a>

### 39 · A · Abracadabra

**自动草稿候选。** 两份都在歌曲「Abracadabra」的同一个 song ID 下，part 都是 pack；全部 8 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92YXJWWZK9H1XFWNG0GM` | motion_part / pack | `Library/Motions/Abracadabra__Unknown/pack__Unknown` | 8 / 39,184,064 | 无 |
| `01M81T9JTS28D2S6KH94MP45FA` | motion_part / pack | `Library/Motions/Abracadabra__Unknown/pack__Unknown_Brown Eyed Girls - Abrac` | 8 / 39,184,064 | 无 |

**父歌**　`01M81T92YXEJRM8CJSESFQNA6Y` `{"title":"Abracadabra","composer":"_Unknown"}`。
**文件核对**　8 种不同内容，单份集合 39,184,064 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92YXJWWZK9H1XFWNG0GM`，最终目录 `Library/Motions/Abracadabra__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92YXJWWZK9H1XFWNG0GM`，归并 `01M81T9JTS28D2S6KH94MP45FA`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JTS28D2S6KH94MP45FA` → `_Duplicates/kagura-review-20261124/01M81T9JTS28D2S6KH94MP45FA/pack__Unknown_Brown Eyed Girls - Abrac`。
3. `rekey` `01M81T92YXJWWZK9H1XFWNG0GM` → `Library/Motions/Abracadabra__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g40"></a>

### 40 · B · いのちのいぶき / いのちのいぶきダンス

**需要人工确认。** 两份素材属于不同父歌，已核对两边父歌的自有负载、子素材、所有入边及对应 VMD。两边的主动作均为 5,471 帧并同含短循环分轨，WAV 内容也相同。作者说明称 いのちのいぶきダンス，保留这一父歌和具有对应 WAV 原名的包。

**署名依据**　Read me ENG.txt 制作者自称 monotama_go，两份日英文说明均给出 @monotama_go。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T91WYERPHM5A7RFG3GM83` | motion_part / pack | `Library/Motions/いのちのいぶき__Unknown/pack__Unknown` | 8 / 39,182,182 | 无 |
| `01M81T91X225AX91261RYC453V` | motion_part / pack | `Library/Motions/いのちのいぶきダンス__Unknown/pack__Unknown` | 8 / 39,182,182 | 无 |

**父歌**　`01M81T91WYDFS5GBMEPSAGCN3V` `{"title":"いのちのいぶき","composer":"_Unknown"}`；`01M81T91X236YX26S2FYWVP9NX` `{"title":"いのちのいぶきダンス","composer":"_Unknown"}`。
**文件核对**　8 种不同内容，单份集合 39,182,182 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T91X225AX91261RYC453V`，最终目录 `Library/Motions/いのちのいぶきダンス__Unknown/pack_monotama_go`。

1. `merge` 保留 `01M81T91X225AX91261RYC453V`，归并 `01M81T91WYERPHM5A7RFG3GM83`。字段 `/source` → `{"url":"https://fantia.jp/posts/1606963"}`。
2. `quarantine` `01M81T91WYERPHM5A7RFG3GM83` → `_Duplicates/kagura-review-20261124/01M81T91WYERPHM5A7RFG3GM83/pack__Unknown`。
3. `merge` 保留 `01M81T91X236YX26S2FYWVP9NX`，归并 `01M81T91WYDFS5GBMEPSAGCN3V`。沿用保留记录的其余字段。
4. `add_alias` `01M81T91X236YX26S2FYWVP9NX` `/title_alias` → `["いのちのいぶき"]`。
5. `quarantine` `01M81T91WYDFS5GBMEPSAGCN3V` → `_Duplicates/kagura-review-20261124/01M81T91WYDFS5GBMEPSAGCN3V/いのちのいぶき__Unknown`。
6. `rekey` `01M81T91X225AX91261RYC453V` → `Library/Motions/いのちのいぶきダンス__Unknown/pack_monotama_go`。`/key` → `{"part":"pack","author":"monotama_go"}`。

父歌的入向引用如下。

- `01M81T91WYERPHM5A7RFG3GM83` `song` → `01M81T91WYDFS5GBMEPSAGCN3V`；来源素材随本组归档。
- `01M81T91X225AX91261RYC453V` `song` → `01M81T91X236YX26S2FYWVP9NX`；保留原目标。

两首父歌均无自有负载，各只有表中这一份素材。

**人工确认**　跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。

<a id="g41"></a>

### 41 · A · Devil

**自动草稿候选。** 两份都在歌曲「Devil」的同一个 song ID 下，part 都是 pack；全部 6 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92ZMWYTZ0AVMDEX892KW` | motion_part / pack | `Library/Motions/Devil__Unknown/pack__Unknown` | 6 / 38,518,255 | 无 |
| `01M81T9JVCS7YAJG8V3K3GCNYQ` | motion_part / pack | `Library/Motions/Devil__Unknown/pack__Unknown_CLC - Devil` | 6 / 38,518,255 | 无 |

**父歌**　`01M81T92ZM4V1BZ3XQT04XTG3M` `{"title":"Devil","composer":"_Unknown"}`。
**文件核对**　6 种不同内容，单份集合 38,518,255 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92ZMWYTZ0AVMDEX892KW`，最终目录 `Library/Motions/Devil__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92ZMWYTZ0AVMDEX892KW`，归并 `01M81T9JVCS7YAJG8V3K3GCNYQ`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JVCS7YAJG8V3K3GCNYQ` → `_Duplicates/kagura-review-20261124/01M81T9JVCS7YAJG8V3K3GCNYQ/pack__Unknown_CLC - Devil`。
3. `rekey` `01M81T92ZMWYTZ0AVMDEX892KW` → `Library/Motions/Devil__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g42"></a>

### 42 · A · Bunny Style

**自动草稿候选。** 两份都在歌曲「Bunny Style」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93YTDZWQXPZX37ZBSFJS` | motion_part / pack | `Library/Motions/Bunny Style__Unknown/pack__Unknown` | 7 / 38,412,003 | 无 |
| `01M81T9KBVRZEP8EC5KHDDHVG8` | motion_part / pack | `Library/Motions/Bunny Style__Unknown/pack__Unknown_T-ARA - Bunny Style` | 7 / 38,412,003 | 无 |

**父歌**　`01M81T93YTB0KZJ74RDTWGYJKR` `{"title":"Bunny Style","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 38,412,003 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93YTDZWQXPZX37ZBSFJS`，最终目录 `Library/Motions/Bunny Style__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93YTDZWQXPZX37ZBSFJS`，归并 `01M81T9KBVRZEP8EC5KHDDHVG8`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KBVRZEP8EC5KHDDHVG8` → `_Duplicates/kagura-review-20261124/01M81T9KBVRZEP8EC5KHDDHVG8/pack__Unknown_T-ARA - Bunny Style`。
3. `rekey` `01M81T93YTDZWQXPZX37ZBSFJS` → `Library/Motions/Bunny Style__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g43"></a>

### 43 · A · ALIEN

**自动草稿候选。** 两份都在歌曲「ALIEN」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93FBJXVVKX7B7R62V7HE` | motion_part / pack | `Library/Motions/ALIEN__Unknown/pack__Unknown` | 7 / 38,409,171 | 无 |
| `01M81T9JWZ1PS3B8136ZZWZW29` | motion_part / pack | `Library/Motions/ALIEN__Unknown/pack__Unknown_LEE SUHYUN - ALIEN` | 7 / 38,409,171 | 无 |

**父歌**　`01M81T93FB84T301KG9KT52H4N` `{"title":"ALIEN","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 38,409,171 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93FBJXVVKX7B7R62V7HE`，最终目录 `Library/Motions/ALIEN__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93FBJXVVKX7B7R62V7HE`，归并 `01M81T9JWZ1PS3B8136ZZWZW29`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JWZ1PS3B8136ZZWZW29` → `_Duplicates/kagura-review-20261124/01M81T9JWZ1PS3B8136ZZWZW29/pack__Unknown_LEE SUHYUN - ALIEN`。
3. `rekey` `01M81T93FBJXVVKX7B7R62V7HE` → `Library/Motions/ALIEN__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g44"></a>

### 44 · B · SexiMi / SexyMi

**需要人工确认。** 两份素材属于不同父歌，已核对两边父歌的自有负载、子素材、所有入边及对应 VMD。同一 WAV、同一 5,984 帧动作和 6,000 帧镜头，两首各只含该素材。SexyMi 出现在实际 WAV 文件名，SexiMi 保留作检索别名。

**署名依据**　无动作署名，WAV 文件名给出曲名拼写 SexyMi。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9CVEY0YX800EA0GFDGQQ` | motion_part / pack | `Library/Motions/SexiMi__Unknown/pack__Unknown` | 3 / 38,404,017 | 无 |
| `01M81TAQ4VR8HXSFDWH776HV25` | motion_part / pack | `Library/Motions/SexyMi__Unknown/pack__Unknown` | 3 / 38,404,017 | 无 |

**父歌**　`01M81T9CVEDRAR9D39YYRDD8Y8` `{"title":"SexiMi","composer":"_Unknown"}`；`01M81TAQ4V6W2XEG44M60DZ6VR` `{"title":"SexyMi","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 38,404,017 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81TAQ4VR8HXSFDWH776HV25`，最终目录 `Library/Motions/SexyMi__Unknown/pack__Unknown`。

1. `merge` 保留 `01M81TAQ4VR8HXSFDWH776HV25`，归并 `01M81T9CVEY0YX800EA0GFDGQQ`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9CVEY0YX800EA0GFDGQQ` → `_Duplicates/kagura-review-20261124/01M81T9CVEY0YX800EA0GFDGQQ/pack__Unknown`。
3. `merge` 保留 `01M81TAQ4V6W2XEG44M60DZ6VR`，归并 `01M81T9CVEDRAR9D39YYRDD8Y8`。沿用保留记录的其余字段。
4. `add_alias` `01M81TAQ4V6W2XEG44M60DZ6VR` `/title_alias` → `["SexiMi"]`。
5. `quarantine` `01M81T9CVEDRAR9D39YYRDD8Y8` → `_Duplicates/kagura-review-20261124/01M81T9CVEDRAR9D39YYRDD8Y8/SexiMi__Unknown`。

父歌的入向引用如下。

- `01M81T9CVEY0YX800EA0GFDGQQ` `song` → `01M81T9CVEDRAR9D39YYRDD8Y8`；来源素材随本组归档。
- `01M81TAQ4VR8HXSFDWH776HV25` `song` → `01M81TAQ4V6W2XEG44M60DZ6VR`；保留原目标。

两首父歌均无自有负载，各只有表中这一份素材。

**人工确认**　跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。

<a id="g45"></a>

### 45 · A · Hype Boy

**自动草稿候选。** 两份都在歌曲「Hype Boy」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93G2D539B4AZH7C9R7SM` | motion_part / pack | `Library/Motions/Hype Boy__Unknown/pack__Unknown` | 9 / 38,317,521 | 无 |
| `01M81T9JXK9GQNMT1GBRM36QF8` | motion_part / pack | `Library/Motions/Hype Boy__Unknown/pack__Unknown_NewJeans - Hype Boy` | 9 / 38,317,521 | 无 |

**父歌**　`01M81T93G256YYT47MTEAKWNMX` `{"title":"Hype Boy","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 38,317,521 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93G2D539B4AZH7C9R7SM`，最终目录 `Library/Motions/Hype Boy__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93G2D539B4AZH7C9R7SM`，归并 `01M81T9JXK9GQNMT1GBRM36QF8`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JXK9GQNMT1GBRM36QF8` → `_Duplicates/kagura-review-20261124/01M81T9JXK9GQNMT1GBRM36QF8/pack__Unknown_NewJeans - Hype Boy`。
3. `rekey` `01M81T93G2D539B4AZH7C9R7SM` → `Library/Motions/Hype Boy__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g46"></a>

### 46 · A · Queendom

**自动草稿候选。** 两份都在歌曲「Queendom」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93Y173PP1DZG6NCWX64C` | motion_part / pack | `Library/Motions/Queendom__Unknown/pack__Unknown` | 9 / 38,286,868 | 无 |
| `01M81T9KB5CYFSJEZ91P72MQ5D` | motion_part / pack | `Library/Motions/Queendom__Unknown/pack__Unknown_Red Velvet - Queendom` | 9 / 38,286,868 | 无 |

**父歌**　`01M81T93Y13Z9VEZ44FSAJGM6K` `{"title":"Queendom","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 38,286,868 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93Y173PP1DZG6NCWX64C`，最终目录 `Library/Motions/Queendom__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93Y173PP1DZG6NCWX64C`，归并 `01M81T9KB5CYFSJEZ91P72MQ5D`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KB5CYFSJEZ91P72MQ5D` → `_Duplicates/kagura-review-20261124/01M81T9KB5CYFSJEZ91P72MQ5D/pack__Unknown_Red Velvet - Queendom`。
3. `rekey` `01M81T93Y173PP1DZG6NCWX64C` → `Library/Motions/Queendom__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g47"></a>

### 47 · A · TOMBOY

**自动草稿候选。** 两份都在歌曲「TOMBOY」的同一个 song ID 下，part 都是 pack；全部 6 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92FRC5MB3PXF4CTJ39Y5` | motion_part / pack | `Library/Motions/TOMBOY__Unknown/pack__Unknown` | 6 / 36,990,730 | 无 |
| `01M81T9JD9WJKH74BRNS4RXBZN` | motion_part / pack | `Library/Motions/TOMBOY__Unknown/pack__Unknown_(G)I-DLE - TOMBOY` | 6 / 36,990,730 | 无 |

**父歌**　`01M81T92FRYD56CJQZFE4H9Y64` `{"title":"TOMBOY","composer":"_Unknown"}`。
**文件核对**　6 种不同内容，单份集合 36,990,730 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92FRC5MB3PXF4CTJ39Y5`，最终目录 `Library/Motions/TOMBOY__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92FRC5MB3PXF4CTJ39Y5`，归并 `01M81T9JD9WJKH74BRNS4RXBZN`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JD9WJKH74BRNS4RXBZN` → `_Duplicates/kagura-review-20261124/01M81T9JD9WJKH74BRNS4RXBZN/pack__Unknown_(G)I-DLE - TOMBOY`。
3. `rekey` `01M81T92FRC5MB3PXF4CTJ39Y5` → `Library/Motions/TOMBOY__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g48"></a>

### 48 · A · POP!

**自动草稿候选。** 两份都在歌曲「POP!」的同一个 song ID 下，part 都是 pack；全部 10 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93FQPYF2FRQ54P63TPNZ` | motion_part / pack | `Library/Motions/POP!__Unknown/pack__Unknown` | 10 / 36,431,386 | 无 |
| `01M81T9JXCN05Q45WHK5MKYA6X` | motion_part / pack | `Library/Motions/POP!__Unknown/pack__Unknown_NAYEON - POP!` | 10 / 36,431,386 | 无 |

**父歌**　`01M81T93FQ9ASVG1FCHT15Q45Q` `{"title":"POP!","composer":"_Unknown"}`。
**文件核对**　10 种不同内容，单份集合 36,431,386 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93FQPYF2FRQ54P63TPNZ`，最终目录 `Library/Motions/POP!__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93FQPYF2FRQ54P63TPNZ`，归并 `01M81T9JXCN05Q45WHK5MKYA6X`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JXCN05Q45WHK5MKYA6X` → `_Duplicates/kagura-review-20261124/01M81T9JXCN05Q45WHK5MKYA6X/pack__Unknown_NAYEON - POP!`。
3. `rekey` `01M81T93FQPYF2FRQ54P63TPNZ` → `Library/Motions/POP!__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g49"></a>

### 49 · A · Forgive Me

**自动草稿候选。** 两份都在歌曲「Forgive Me」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92YKHZW1SD11V9Z9FETC` | motion_part / pack | `Library/Motions/Forgive Me__Unknown/pack__Unknown` | 7 / 36,082,717 | 无 |
| `01M81T9JTPRRTE76QB6MVBPCBG` | motion_part / pack | `Library/Motions/Forgive Me__Unknown/pack__Unknown_BoA - Forgive Me` | 7 / 36,082,717 | 无 |

**父歌**　`01M81T92YKMMJGHXBZ7G12DJ1G` `{"title":"Forgive Me","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 36,082,717 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92YKHZW1SD11V9Z9FETC`，最终目录 `Library/Motions/Forgive Me__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92YKHZW1SD11V9Z9FETC`，归并 `01M81T9JTPRRTE76QB6MVBPCBG`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JTPRRTE76QB6MVBPCBG` → `_Duplicates/kagura-review-20261124/01M81T9JTPRRTE76QB6MVBPCBG/pack__Unknown_BoA - Forgive Me`。
3. `rekey` `01M81T92YKHZW1SD11V9Z9FETC` → `Library/Motions/Forgive Me__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g50"></a>

### 50 · A · Nice Body

**自动草稿候选。** 两份都在歌曲「Nice Body」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T930GW74EFFCANSF8HCC1` | motion_part / pack | `Library/Motions/Nice Body__Unknown/pack__Unknown` | 7 / 35,656,543 | 无 |
| `01M81T9JW1N22SNHDHAYV7SCZ7` | motion_part / pack | `Library/Motions/Nice Body__Unknown/pack__Unknown_HYO MIN - Nice Body` | 7 / 35,656,543 | 无 |

**父歌**　`01M81T930GMHK0BY20AA26AMGS` `{"title":"Nice Body","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 35,656,543 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T930GW74EFFCANSF8HCC1`，最终目录 `Library/Motions/Nice Body__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T930GW74EFFCANSF8HCC1`，归并 `01M81T9JW1N22SNHDHAYV7SCZ7`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JW1N22SNHDHAYV7SCZ7` → `_Duplicates/kagura-review-20261124/01M81T9JW1N22SNHDHAYV7SCZ7/pack__Unknown_HYO MIN - Nice Body`。
3. `rekey` `01M81T930GW74EFFCANSF8HCC1` → `Library/Motions/Nice Body__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g51"></a>

### 51 · A · Queencard

**自动草稿候选。** 两份都在歌曲「Queencard」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T92FNMVK7D8VES5V381V2` | motion_part / pack | `Library/Motions/Queencard__Unknown/pack__Unknown` | 7 / 35,082,448 | 无 |
| `01M81T9JD6NB3876214EXCJA31` | motion_part / pack | `Library/Motions/Queencard__Unknown/pack__Unknown_(G)I-DLE - Queencard` | 7 / 35,082,448 | 无 |

**父歌**　`01M81T92FNBDK2MKC998H2CQAD` `{"title":"Queencard","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 35,082,448 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T92FNMVK7D8VES5V381V2`，最终目录 `Library/Motions/Queencard__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T92FNMVK7D8VES5V381V2`，归并 `01M81T9JD6NB3876214EXCJA31`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JD6NB3876214EXCJA31` → `_Duplicates/kagura-review-20261124/01M81T9JD6NB3876214EXCJA31/pack__Unknown_(G)I-DLE - Queencard`。
3. `rekey` `01M81T92FNMVK7D8VES5V381V2` → `Library/Motions/Queencard__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g52"></a>

### 52 · A · Air Force One2023-11

**自动草稿候选。** 两份都在歌曲「Air Force One2023-11」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93G6QEFXZNE04H9AVEXN` | motion_part / pack | `Library/Motions/Air Force One2023-11__Unknown/pack__Unknown` | 9 / 34,910,294 | 无 |
| `01M81T9KAZ27B7Y9BSKGP7241D` | motion_part / pack | `Library/Motions/Air Force One2023-11__Unknown/pack__Unknown_ODD EYE CIRCLE - Air For` | 9 / 34,910,294 | 无 |

**父歌**　`01M81T93G65N0HJJESMSDR8G4H` `{"title":"Air Force One2023-11","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 34,910,294 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93G6QEFXZNE04H9AVEXN`，最终目录 `Library/Motions/Air Force One2023-11__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93G6QEFXZNE04H9AVEXN`，归并 `01M81T9KAZ27B7Y9BSKGP7241D`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KAZ27B7Y9BSKGP7241D` → `_Duplicates/kagura-review-20261124/01M81T9KAZ27B7Y9BSKGP7241D/pack__Unknown_ODD EYE CIRCLE - Air For`。
3. `rekey` `01M81T93G6QEFXZNE04H9AVEXN` → `Library/Motions/Air Force One2023-11__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g53"></a>

### 53 · C · TDA Monochrome Doll Haku / TDA Monochrome Doll Haku (Haku)

**自动草稿候选。** 两份均是 Monochrome Doll Haku.pmx，只有同一份 Tda 说明文件的路径编码不同。保留文件名可读且没有重复角色后缀的一份。

**署名依据**　本包 README.txt 落款 HarukaSakurai，并明确由她负责服装转换、绑定和物理；使用时要求署 TDA 和 HarukaSakurai。PMX 的 aga 署名与附带 aga_readme.txt 属于泳装素体改造。
**建议显示名**　`TDA Monochrome Doll Haku`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T6S5WHQZGD8BX1WM0SM7G` | model / character | `Library/Models/Characters/aga/TDA Monochrome Doll Haku` | 35 / 33,550,176 | 明确许可 |
| `01M822G21ZEB0KSSW23FN78ZX5` | model / character | `Library/Models/Characters/aga/TDA Monochrome Doll Haku (Haku)` | 35 / 33,550,176 | 明确许可 |

**文件核对**　35 种不同内容，单份集合 33,550,176 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T6S5WHQZGD8BX1WM0SM7G`，最终目录 `Library/Models/Characters/HarukaSakurai/TDA Monochrome Doll Haku`。

1. `merge` 保留 `01M81T6S5WHQZGD8BX1WM0SM7G`，归并 `01M822G21ZEB0KSSW23FN78ZX5`。沿用保留记录的其余字段。
2. `quarantine` `01M822G21ZEB0KSSW23FN78ZX5` → `_Duplicates/kagura-review-20261124/01M822G21ZEB0KSSW23FN78ZX5/TDA Monochrome Doll Haku (Haku)`。
3. `rekey` `01M81T6S5WHQZGD8BX1WM0SM7G` → `Library/Models/Characters/HarukaSakurai/TDA Monochrome Doll Haku`。`/key` → `{"author":"HarukaSakurai","name":"TDA Monochrome Doll Haku"}`。

**取舍**　merge 后 rekey 至最终制作者；保留 README.txt 和所有基础模型说明，保留现有许可字段。

<a id="g54"></a>

### 54 · A · ETA

**自动草稿候选。** 两份都在歌曲「ETA」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93FY1TJ50G83TZ65KG9K` | motion_part / pack | `Library/Motions/ETA__Unknown/pack__Unknown` | 9 / 32,873,850 | 无 |
| `01M81T9JXFBCRDAVYYD7668FVS` | motion_part / pack | `Library/Motions/ETA__Unknown/pack__Unknown_NewJeans - ETA` | 9 / 32,873,850 | 无 |

**父歌**　`01M81T93FYH7Z24PVJ8G6DN5AV` `{"title":"ETA","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 32,873,850 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93FY1TJ50G83TZ65KG9K`，最终目录 `Library/Motions/ETA__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93FY1TJ50G83TZ65KG9K`，归并 `01M81T9JXFBCRDAVYYD7668FVS`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JXFBCRDAVYYD7668FVS` → `_Duplicates/kagura-review-20261124/01M81T9JXFBCRDAVYYD7668FVS/pack__Unknown_NewJeans - ETA`。
3. `rekey` `01M81T93FY1TJ50G83TZ65KG9K` → `Library/Motions/ETA__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g55"></a>

### 55 · A · NUMBER NINE

**自动草稿候选。** 两份都在歌曲「NUMBER NINE」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93YYS58V2EX696DY1XSX` | motion_part / pack | `Library/Motions/NUMBER NINE__Unknown/pack__Unknown` | 7 / 32,703,735 | 无 |
| `01M81T9KBYN4DHSMPRGDDW3XPT` | motion_part / pack | `Library/Motions/NUMBER NINE__Unknown/pack__Unknown_T-ARA - NUMBER NINE` | 7 / 32,703,735 | 无 |

**父歌**　`01M81T93YYS7VEEFF8VDHVZVA0` `{"title":"NUMBER NINE","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 32,703,735 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93YYS58V2EX696DY1XSX`，最终目录 `Library/Motions/NUMBER NINE__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93YYS58V2EX696DY1XSX`，归并 `01M81T9KBYN4DHSMPRGDDW3XPT`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KBYN4DHSMPRGDDW3XPT` → `_Duplicates/kagura-review-20261124/01M81T9KBYN4DHSMPRGDDW3XPT/pack__Unknown_T-ARA - NUMBER NINE`。
3. `rekey` `01M81T93YYS58V2EX696DY1XSX` → `Library/Motions/NUMBER NINE__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g56"></a>

### 56 · A · 《四合庄上》Ace组合

**自动草稿候选。** 两份都在歌曲「《四合庄上》Ace组合」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留含 tested_with 及配件轨道对应位置的较完整记录，另一份的说明没有提供新的轨道。两条来源不同的表情文件保留各自署名。 保留记录已保有更全的配件引用。

**署名依据**　Read Me.txt 的制作者自称 Yudou，人物动作、镜头、配饰及一条表情文件署名 Yudou；另一条表情署名伝偈津，原分轨署名随包保留。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9KW2HP8NWN9XREHJ22AZ` | motion_part / pack | `Library/Motions/《四合庄上》Ace组合__Unknown/pack__Unknown` | 7 / 32,342,498 | 简评 |
| `01M81T9MVYGG9SJQKC3HJ0HFGC` | motion_part / pack | `Library/Motions/《四合庄上》Ace组合__Unknown/pack__Unknown_《四合庄上》Ace组合` | 7 / 32,342,498 | 简评 |

**父歌**　`01M81T9KW2HR3KZGFJ7ACQBTRD` `{"title":"《四合庄上》Ace组合","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 32,342,498 字节。所有相对路径与内容一致。

保留 `01M81T9KW2HP8NWN9XREHJ22AZ`，最终目录 `Library/Motions/《四合庄上》Ace组合__Unknown/pack_Yudou`。

1. `merge` 保留 `01M81T9KW2HP8NWN9XREHJ22AZ`，归并 `01M81T9MVYGG9SJQKC3HJ0HFGC`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9MVYGG9SJQKC3HJ0HFGC` → `_Duplicates/kagura-review-20261124/01M81T9MVYGG9SJQKC3HJ0HFGC/pack__Unknown_《四合庄上》Ace组合`。
3. `rekey` `01M81T9KW2HP8NWN9XREHJ22AZ` → `Library/Motions/《四合庄上》Ace组合__Unknown/pack_Yudou`。`/key` → `{"part":"pack","author":"Yudou"}`。

<a id="g57"></a>

### 57 · B · The Zombie Song / the_zombie_song___motion_dl

**需要人工确认。** 两份素材属于不同父歌，已核对两边父歌的自有负载、子素材、所有入边及对应 VMD。WAV、3 个角色动作、镜头与署名说明相同。说明写明歌曲 The Zombie Song、Stephanie Mabey，后者未单独列出作曲分工。the_zombie_song___motion_dl 是下载名称，原 name_original 作为历史检索名保留。

**署名依据**　The Zombie Song/Readme.txt 明示 Motion By Sweet B.Girl；shira-flame 是同页联络账号，davyujd 为目录标识，Stephanie Mabey 出现在歌曲署名中。 保留 readme 中的历史联络信息，可按作品署名更正 key；不为编目方便将歌曲署名改填为作曲。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9KVV94DFQJTDHV8TDRZ7` | motion_part / pack | `Library/Motions/The Zombie Song__Unknown/pack_shira_flame-davyujd` | 6 / 31,563,203 | 无 |
| `01M81TAKBNT4RS1SF428E8HCXY` | motion_part / pack | `Library/Motions/the_zombie_song___motion_dl__Unknown/pack_shira_flame-davyujd` | 6 / 31,563,203 | 无 |

**父歌**　`01M81T9KVV0WW6QWHJYRJDWJ80` `{"title":"The Zombie Song","composer":"_Unknown"}`；`01M81TAKBN99BR8GKX2EGG0B56` `{"title":"the_zombie_song___motion_dl","composer":"_Unknown"}`。
**文件核对**　6 种不同内容，单份集合 31,563,203 字节。所有相对路径与内容一致。

保留 `01M81T9KVV94DFQJTDHV8TDRZ7`，最终目录 `Library/Motions/The Zombie Song__Unknown/pack_Sweet B.Girl`。

1. `merge` 保留 `01M81T9KVV94DFQJTDHV8TDRZ7`，归并 `01M81TAKBNT4RS1SF428E8HCXY`。沿用保留记录的其余字段。
2. `quarantine` `01M81TAKBNT4RS1SF428E8HCXY` → `_Duplicates/kagura-review-20261124/01M81TAKBNT4RS1SF428E8HCXY/pack_shira_flame-davyujd`。
3. `merge` 保留 `01M81T9KVV0WW6QWHJYRJDWJ80`，归并 `01M81TAKBN99BR8GKX2EGG0B56`。沿用保留记录的其余字段。
4. `add_alias` `01M81T9KVV0WW6QWHJYRJDWJ80` `/title_alias` → `["the_zombie_song___motion_dl","mmd__the_zombie_song___motion_dl"]`。
5. `quarantine` `01M81TAKBN99BR8GKX2EGG0B56` → `_Duplicates/kagura-review-20261124/01M81TAKBN99BR8GKX2EGG0B56/the_zombie_song___motion_dl__Unknown`。
6. `rekey` `01M81T9KVV94DFQJTDHV8TDRZ7` → `Library/Motions/The Zombie Song__Unknown/pack_Sweet B.Girl`。`/key` → `{"part":"pack","author":"Sweet B.Girl"}`。

父歌的入向引用如下。

- `01M81T9KVV94DFQJTDHV8TDRZ7` `song` → `01M81T9KVV0WW6QWHJYRJDWJ80`；保留原目标。
- `01M81TAKBNT4RS1SF428E8HCXY` `song` → `01M81TAKBN99BR8GKX2EGG0B56`；来源素材随本组归档。

两首父歌均无自有负载，各只有表中这一份素材。

**人工确认**　跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。

<a id="g58"></a>

### 58 · A · SOMETHING

**自动草稿候选。** 两份都在歌曲「SOMETHING」的同一个 song ID 下，part 都是 pack；全部 5 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9305JHTFDRVWPC4VA2JD` | motion_part / pack | `Library/Motions/SOMETHING__Unknown/pack__Unknown` | 5 / 30,724,491 | 无 |
| `01M81T9JVRH5F8C27ZTS1JQZ1S` | motion_part / pack | `Library/Motions/SOMETHING__Unknown/pack__Unknown_GIRL'S DAY - SOMETHING` | 5 / 30,724,491 | 无 |

**父歌**　`01M81T93055F4YNTA5APNMR02K` `{"title":"SOMETHING","composer":"_Unknown"}`。
**文件核对**　5 种不同内容，单份集合 30,724,491 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T9305JHTFDRVWPC4VA2JD`，最终目录 `Library/Motions/SOMETHING__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T9305JHTFDRVWPC4VA2JD`，归并 `01M81T9JVRH5F8C27ZTS1JQZ1S`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JVRH5F8C27ZTS1JQZ1S` → `_Duplicates/kagura-review-20261124/01M81T9JVRH5F8C27ZTS1JQZ1S/pack__Unknown_GIRL'S DAY - SOMETHING`。
3. `rekey` `01M81T9305JHTFDRVWPC4VA2JD` → `Library/Motions/SOMETHING__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g59"></a>

### 59 · A · 『プラチナ』-shin'in future Mix- いとくとら様Ver

**自动草稿候选。** 两份都在歌曲「『プラチナ』-shin'in future Mix- いとくとら様Ver」的同一个 song ID 下，part 都是 pack；全部 4 种文件内容相同。保留可独立使用的包及其内部文件路径。

**署名依据**　nac_platinai/読んでね.txt 落款「なつき（～してもらいましたP）」。いとくとら 是舞者。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9MSG50KFZC1N4HCZVEXH` | motion_part / pack | `Library/Motions/『プラチナ』-shin'in future Mix- いとくとら様Ver__Unknown/pack__Unknown` | 4 / 29,902,047 | 无 |
| `01M81T9NQE680T35N4SJ25VN28` | motion_part / pack | `Library/Motions/『プラチナ』-shin'in future Mix- いとくとら様Ver__Unknown/pack__Unknown_『プラチナ』-shin'in future Mi` | 4 / 29,902,047 | 无 |

**父歌**　`01M81T9MSG3JW0HDDSMV61NR48` `{"title":"『プラチナ』-shin'in future Mix- いとくとら様Ver","composer":"_Unknown"}`。
**文件核对**　4 种不同内容，单份集合 29,902,047 字节。所有相对路径与内容一致。

保留 `01M81T9MSG50KFZC1N4HCZVEXH`，最终目录 `Library/Motions/『プラチナ』-shin'in future Mix- いとくとら様Ver__Unknown/pack_なつき`。

1. `merge` 保留 `01M81T9MSG50KFZC1N4HCZVEXH`，归并 `01M81T9NQE680T35N4SJ25VN28`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9NQE680T35N4SJ25VN28` → `_Duplicates/kagura-review-20261124/01M81T9NQE680T35N4SJ25VN28/pack__Unknown_『プラチナ』-shin'in future Mi`。
3. `rekey` `01M81T9MSG50KFZC1N4HCZVEXH` → `Library/Motions/『プラチナ』-shin'in future Mix- いとくとら様Ver__Unknown/pack_なつき`。`/key` → `{"part":"pack","author":"なつき"}`。

<a id="g60"></a>

### 60 · A · Genie

**自动草稿候选。** 两份都在歌曲「Genie」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T930817CNVRC344216HDY` | motion_part / pack | `Library/Motions/Genie__Unknown/pack__Unknown` | 7 / 29,321,698 | 无 |
| `01M81T9JVV7032WYE44MG8DPHS` | motion_part / pack | `Library/Motions/Genie__Unknown/pack__Unknown_Girls' Generation - Geni` | 7 / 29,321,698 | 无 |

**父歌**　`01M81T9308GW1H7E3GQT59PX1P` `{"title":"Genie","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 29,321,698 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T930817CNVRC344216HDY`，最终目录 `Library/Motions/Genie__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T930817CNVRC344216HDY`，归并 `01M81T9JVV7032WYE44MG8DPHS`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JVV7032WYE44MG8DPHS` → `_Duplicates/kagura-review-20261124/01M81T9JVV7032WYE44MG8DPHS/pack__Unknown_Girls' Generation - Geni`。
3. `rekey` `01M81T930817CNVRC344216HDY` → `Library/Motions/Genie__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g61"></a>

### 61 · A · Bboom Bboom

**自动草稿候选。** 两份都在歌曲「Bboom Bboom」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留含具名 WAV 和编号独舞分轨的一侧，按哈希逐一核对声明角色。

**署名依据**　没有署名说明；音频名可以辨认歌手，未据此补写动作作者。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T95X33S7SDRXBSCTCP01G` | motion_part / pack | `Library/Motions/Bboom Bboom__Unknown/pack__Unknown` | 9 / 28,107,162 | 无 |
| `01M81TA50W1M9A8ADNFPY7X76Y` | motion_part / pack | `Library/Motions/Bboom Bboom__Unknown/pack__Unknown_Bboom Bboom` | 9 / 28,107,162 | 无 |

**父歌**　`01M81T95X3T3B7QHDG2YAPG1PV` `{"title":"Bboom Bboom","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 28,107,162 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81TA50W1M9A8ADNFPY7X76Y`，最终目录 `Library/Motions/Bboom Bboom__Unknown/pack__Unknown_Bboom Bboom`。

1. `merge` 保留 `01M81TA50W1M9A8ADNFPY7X76Y`，归并 `01M81T95X33S7SDRXBSCTCP01G`。沿用保留记录的其余字段。
2. `quarantine` `01M81T95X33S7SDRXBSCTCP01G` → `_Duplicates/kagura-review-20261124/01M81T95X33S7SDRXBSCTCP01G/pack__Unknown`。

<a id="g62"></a>

### 62 · C · TDA Animal Miku

**自动草稿候选。** 28 个文件逐路径一致，PMX 完整标题明确 Hoodie 款式。保留有作者及本模型 readme 的 ID，从 scanner 副本采纳 name_original。

**署名依据**　READ ME!.txt 和 PMX 都署 Model by FlyingTunas；KandaYuu42 是基础模型来源，其他署名各自对应鞋、毛衣与打底裤。
**建议显示名**　`TDA Animal Miku`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXK072RJNSAWPHC00D8DZ` | model / character | `Library/Models/Characters/FlyingTunas/TDA Animal Miku` | 28 / 34,827,875 | 无 |
| `01M81RK1YNCHGBMX75YX8HWRDR` | model / character | `Library/Models/Characters/_Unknown/TDA Animal Miku` | 28 / 34,827,875 | 无 |

**文件核对**　22 种不同内容，单份集合 26,050,609 字节。所有相对路径与内容一致。

保留 `01M81TXK072RJNSAWPHC00D8DZ`，最终目录 `Library/Models/Characters/FlyingTunas/TDA Animal Miku`。

1. `merge` 保留 `01M81TXK072RJNSAWPHC00D8DZ`，归并 `01M81RK1YNCHGBMX75YX8HWRDR`。字段 `/name_original` → `"TDA Animal Hoodie Miku"`。
2. `quarantine` `01M81RK1YNCHGBMX75YX8HWRDR` → `_Duplicates/kagura-review-20261124/01M81RK1YNCHGBMX75YX8HWRDR/TDA Animal Miku`。

**取舍**　merge；保留既有 key，补入另一份已有的 name_original。

<a id="g63"></a>

### 63 · E · I won't say I'm in Love / _General

**需要人工确认。** 一侧关联歌曲，另一侧位于 _General。两份 WAV 及全部 7 个 VMD 的哈希一致，组内无需合并两首歌。保留 _General 中记录原目录、含更清楚的道具姿态和音轨文件名的一份，将 song 改为已识别的父歌。

**署名依据**　两边无署名，原包保留来源目录记录以及可读的音频、道具姿势文件名。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T99KQ57KXSBG1ZVR0E183` | motion_part / pack | `Library/Motions/I won't say I'm in Love__Unknown/pack__Unknown` | 8 / 25,088,779 | 无 |
| `01M829XRVT0MP0KQDXYK4W97YX` | motion_part / pack | `Library/Motions/_General/pack__Unknown_I won't Say I'm in Love` | 8 / 25,088,779 | 简评 |

**父歌**　`01M81T99KQH969X1S7XMAKRQFS` `{"title":"I won't say I'm in Love","composer":"_Unknown"}`；`_General`（song=null）。
**文件核对**　8 种不同内容，单份集合 25,088,779 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M829XRVT0MP0KQDXYK4W97YX`，最终目录 `Library/Motions/I won't say I'm in Love__Unknown/pack__Unknown_I won't Say I'm in Love`。

1. `merge` 保留 `01M829XRVT0MP0KQDXYK4W97YX`，归并 `01M81T99KQ57KXSBG1ZVR0E183`。沿用保留记录的其余字段。
2. `quarantine` `01M81T99KQ57KXSBG1ZVR0E183` → `_Duplicates/kagura-review-20261124/01M81T99KQ57KXSBG1ZVR0E183/pack__Unknown`。
3. `move_part` `01M829XRVT0MP0KQDXYK4W97YX` → `Library/Motions/I won't say I'm in Love__Unknown/pack__Unknown_I won't Say I'm in Love`。`/song` → `"01M81T99KQH969X1S7XMAKRQFS"`。

**人工确认**　需要核实原 _General 编目的用途，再将该素材归入已识别歌曲。

<a id="g64"></a>

### 64 · C · Haku Wedding Dress / Haku Wedding Dress (弱音模型)

**自动草稿候选。** 主 PMX 是 TDA Haku Wedding Dress(Beta 1.0)，78 条负载的内容及出现次数相同。选已有显式预览的一侧；另一份的 01.png 与该预览同哈希。

**署名依据**　根目录读我.txt 落款 samsink，PMX 署 Samsink（機動戰士牛肉）；Luka 说明及 Tda、aga 文档是基础部件来源。作者目录采用既存 SAMSINK 的大小写。
**建议显示名**　`Haku Wedding Dress`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXD9VBBQC7W1ZY0N8X627` | model / character | `Library/Models/Characters/_Unknown/Haku Wedding Dress` | 78 / 25,438,886 | 明确许可 |
| `01M822G3DKSG1NXA1QQ0NBNE8J` | model / character | `Library/Models/Characters/_Unknown/Haku Wedding Dress (弱音模型)` | 78 / 25,438,886 | 明确许可 |

**文件核对**　68 种不同内容，单份集合 24,447,908 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M822G3DKSG1NXA1QQ0NBNE8J`，最终目录 `Library/Models/Characters/SAMSINK/Haku Wedding Dress`。

1. `merge` 保留 `01M822G3DKSG1NXA1QQ0NBNE8J`，归并 `01M81TXD9VBBQC7W1ZY0N8X627`。字段 `/name_original` → `"TDA Haku Wedding Dress(Beta 1.0)"`；`/license` → `{"readme":"读我.txt","redistribute":false}`。
2. `quarantine` `01M81TXD9VBBQC7W1ZY0N8X627` → `_Duplicates/kagura-review-20261124/01M81TXD9VBBQC7W1ZY0N8X627/Haku Wedding Dress`。
3. `rekey` `01M822G3DKSG1NXA1QQ0NBNE8J` → `Library/Models/Characters/SAMSINK/Haku Wedding Dress`。`/key` → `{"author":"SAMSINK","name":"Haku Wedding Dress"}`。

**取舍**　merge 后 rekey；保留预览，将 license.readme 指向婚纱成品说明，延续已有的禁止再配布值。

<a id="g65"></a>

### 65 · A · ME!ME!ME!

**自动草稿候选。** 两份都在歌曲「ME!ME!ME!」的同一个 song ID 下，part 都是 pack；全部 3 种文件内容相同。保留包含历史目录线索的 suffix；该歌另有第三份无后缀 pack 和独立 motion。两者均未列入本组，不改其 ID、目录或归属。

**署名依据**　旧迁移清单来源目录 Vmd/DataforScan/•ᗰEI•@_mei_/ME!ME!ME! 和现有 suffix 记录账号线索，包内无署名；保留原后缀，不把账号推定为作者。
**来源记录**　`G:/MMD/_design/migration_runs/first/plan.jsonl`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9AZHG43XX4FYDSQR2V1N` | motion_part / pack | `Library/Motions/ME!ME!ME!__Unknown/pack__Unknown_ME!ME!ME!` | 3 / 23,724,148 | 无 |
| `01M81T9MAV19ZC78NK0ZDT2CK1` | motion_part / pack | `Library/Motions/ME!ME!ME!__Unknown/pack__Unknown_•ᗰEI•@_mei_ ME!ME!ME!` | 3 / 23,724,148 | 无 |

**父歌**　`01M81T94FPRZ2BS6HYMGNECWW2` `{"title":"ME!ME!ME!","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 23,724,148 字节。所有相对路径与内容一致。

保留 `01M81T9MAV19ZC78NK0ZDT2CK1`，最终目录 `Library/Motions/ME!ME!ME!__Unknown/pack__Unknown_•ᗰEI•@_mei_ ME!ME!ME!`。

1. `merge` 保留 `01M81T9MAV19ZC78NK0ZDT2CK1`，归并 `01M81T9AZHG43XX4FYDSQR2V1N`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9AZHG43XX4FYDSQR2V1N` → `_Duplicates/kagura-review-20261124/01M81T9AZHG43XX4FYDSQR2V1N/pack__Unknown_ME!ME!ME!`。

<a id="g66"></a>

### 66 · A · U-Go-Girl (YUNA ver.

**自动草稿候选。** 两份都在歌曲「U-Go-Girl (YUNA ver.」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93F8ERFVQ1D9Z8C8FWZ0` | motion_part / pack | `Library/Motions/U-Go-Girl (YUNA ver__Unknown/pack__Unknown` | 7 / 22,828,207 | 无 |
| `01M81T9JWWSX4YES1W3AE0KXP2` | motion_part / pack | `Library/Motions/U-Go-Girl (YUNA ver__Unknown/pack__Unknown_Lee Hyo Ri - U-Go-Girl (` | 7 / 22,828,207 | 无 |

**父歌**　`01M81T93F8GB5W6PZ365N3Q862` `{"title":"U-Go-Girl (YUNA ver.","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 22,828,207 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93F8ERFVQ1D9Z8C8FWZ0`，最终目录 `Library/Motions/U-Go-Girl (YUNA ver__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93F8ERFVQ1D9Z8C8FWZ0`，归并 `01M81T9JWWSX4YES1W3AE0KXP2`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JWWSX4YES1W3AE0KXP2` → `_Duplicates/kagura-review-20261124/01M81T9JWWSX4YES1W3AE0KXP2/pack__Unknown_Lee Hyo Ri - U-Go-Girl (`。
3. `rekey` `01M81T93F8ERFVQ1D9Z8C8FWZ0` → `Library/Motions/U-Go-Girl (YUNA ver__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g67"></a>

### 67 · A · Diamond Sky

**自动草稿候选。** 两份都在歌曲「Diamond Sky」的同一个 song ID 下，part 都是 pack；全部 3 种文件内容相同。保留现有具名作者记录与可辨识的音频文件名。

**署名依据**　原迁移清单 source「动作/合集1/Diamond Sky [motion by ureshiiiiii]」明确标为动作作者，与现有具名包相符。包内无 readme。
**来源记录**　`G:/MMD/_design/migration_runs/first/plan.jsonl`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T979DYVWBH5ATT7PKJ6DD` | motion_part / pack | `Library/Motions/Diamond Sky__Unknown/pack__Unknown` | 3 / 22,049,967 | 无 |
| `01M81TA6W90ZMRP5ZMXXX9EGVG` | motion_part / pack | `Library/Motions/Diamond Sky__Unknown/pack_ureshiiiiii` | 3 / 22,049,967 | 无 |

**父歌**　`01M81T979DZ9GT0WHRTJ60C30M` `{"title":"Diamond Sky","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 22,049,967 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81TA6W90ZMRP5ZMXXX9EGVG`，最终目录 `Library/Motions/Diamond Sky__Unknown/pack_ureshiiiiii`。

1. `merge` 保留 `01M81TA6W90ZMRP5ZMXXX9EGVG`，归并 `01M81T979DYVWBH5ATT7PKJ6DD`。沿用保留记录的其余字段。
2. `quarantine` `01M81T979DYVWBH5ATT7PKJ6DD` → `_Duplicates/kagura-review-20261124/01M81T979DYVWBH5ATT7PKJ6DD/pack__Unknown`。

<a id="g68"></a>

### 68 · A · BOP BOP!

**自动草稿候选。** 两份都在歌曲「BOP BOP!」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93ZASB7B9WV735TPM5RX` | motion_part / pack | `Library/Motions/BOP BOP!__Unknown/pack__Unknown` | 7 / 20,694,019 | 无 |
| `01M81T9KC4YCG9V5SB6EAR4R4A` | motion_part / pack | `Library/Motions/BOP BOP!__Unknown/pack__Unknown_VIVIZ - BOP BOP!` | 7 / 20,694,019 | 无 |

**父歌**　`01M81T93ZAKYZ02G49BRDM52X6` `{"title":"BOP BOP!","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 20,694,019 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93ZASB7B9WV735TPM5RX`，最终目录 `Library/Motions/BOP BOP!__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93ZASB7B9WV735TPM5RX`，归并 `01M81T9KC4YCG9V5SB6EAR4R4A`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KC4YCG9V5SB6EAR4R4A` → `_Duplicates/kagura-review-20261124/01M81T9KC4YCG9V5SB6EAR4R4A/pack__Unknown_VIVIZ - BOP BOP!`。
3. `rekey` `01M81T93ZASB7B9WV735TPM5RX` → `Library/Motions/BOP BOP!__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g69"></a>

### 69 · C · TDA UK Lolita Dress Stardu (星尘模型) / TDA UK Lolita Dress Stardust Ver3.00[Sil

**需要人工确认。** Silver v3.00 模型、同附银色伞与各文件哈希一致。第二个 key 来自完整标题的截断，name_original 已保留全名。两份均将服装素材作者 HarukaSakurai 的说明当成整模许可。

**署名依据**　主 PMX 和根目录 Silver 版本 readme 都将本体编辑者署为维他ViTA Shiro；Crystallyna 署伞，其他署名对应部件。成品说明限制修改、再配布和商用，遇需修改须联系作者，和记录中的 modify:true 冲突。
**建议显示名**　`TDA UK Lolita Dress Stardust V3 Silver`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M822G43H9VEX6W3TC1PFMWPG` | model / character | `Library/Models/Characters/维他ViTA Shiro/TDA UK Lolita Dress Stardu (星尘模型)` | 46 / 20,062,452 | 简评、明确许可 |
| `01M81T6S8R5H8C0CM7V8QYWVA6` | model / character | `Library/Models/Characters/维他ViTA Shiro/TDA UK Lolita Dress Stardust Ver3.00[Sil` | 46 / 20,062,452 | 简评、明确许可 |

**文件核对**　46 种不同内容，单份集合 20,062,452 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T6S8R5H8C0CM7V8QYWVA6`，最终目录 `Library/Models/Characters/维他ViTA Shiro/TDA UK Lolita Dress Stardust V3 Silver`。

1. `merge` 保留 `01M81T6S8R5H8C0CM7V8QYWVA6`，归并 `01M822G43H9VEX6W3TC1PFMWPG`。字段 `/license` → `{"readme":"TDA UK Lolita Dress Stardust Ver3.00[Silver]Readme(维他ViTA Shiro).txt","modify":false,"redistribute":false,"commercial":false}`。
2. `quarantine` `01M822G43H9VEX6W3TC1PFMWPG` → `_Duplicates/kagura-review-20261124/01M822G43H9VEX6W3TC1PFMWPG/TDA UK Lolita Dress Stardu (星尘模型)`。
3. `rekey` `01M81T6S8R5H8C0CM7V8QYWVA6` → `Library/Models/Characters/维他ViTA Shiro/TDA UK Lolita Dress Stardust V3 Silver`。`/key` → `{"author":"维他ViTA Shiro","name":"TDA UK Lolita Dress Stardust V3 Silver"}`。

**取舍**　仅作为确认规约之后的候选：merge、rekey 以保留版本和配色；授权布尔值按未取得额外许可的默认状况填写。

**人工确认**　核对整模限制和另行取得作者许可的情形，再决定修改字段；不自动沿用两个旧布尔值的共同结论。

<a id="g70"></a>

### 70 · A · MANIAC

**自动草稿候选。** 两份都在歌曲「MANIAC」的同一个 song ID 下，part 都是 pack；全部 9 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T93ZD6ZMVH09WXFQ2K142` | motion_part / pack | `Library/Motions/MANIAC__Unknown/pack__Unknown` | 9 / 20,032,368 | 无 |
| `01M81T9KC7G8B9PN03CWXGMVCG` | motion_part / pack | `Library/Motions/MANIAC__Unknown/pack__Unknown_VIVIZ - MANIAC` | 9 / 20,032,368 | 无 |

**父歌**　`01M81T93ZDBHD28ZB98KH1JM7M` `{"title":"MANIAC","composer":"_Unknown"}`。
**文件核对**　9 种不同内容，单份集合 20,032,368 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T93ZD6ZMVH09WXFQ2K142`，最终目录 `Library/Motions/MANIAC__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T93ZD6ZMVH09WXFQ2K142`，归并 `01M81T9KC7G8B9PN03CWXGMVCG`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9KC7G8B9PN03CWXGMVCG` → `_Duplicates/kagura-review-20261124/01M81T9KC7G8B9PN03CWXGMVCG/pack__Unknown_VIVIZ - MANIAC`。
3. `rekey` `01M81T93ZD6ZMVH09WXFQ2K142` → `Library/Motions/MANIAC__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g71"></a>

### 71 · C · tdalukanude / Tda式ルカ モリガン風 Ver1.05 Nude

**需要人工确认。** 同名的 TdaLukanude.pmx 及全部负载一致，一份多两层目录。保留已有模型来源说明、原包结构和正式改模作者记录的一份作为候选。

**署名依据**　readme 列举リリス、モリガン、黒ワンピ等原版，署じーん，PMX 注释也署他负责モリガン風 Ver1.05；均未单独署名 Nude 变体制作者。Tda、aga、モンテコア为更早来源。
**建议显示名**　`Tda式ルカ モリガン風 Ver1.05 Nude`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T6SV4HYX1MWC38HQZHMQN` | model / character | `Library/Models/Characters/_Unknown/tdalukanude` | 49 / 19,452,467 | 明确许可 |
| `01M83SR42GSZSAG7WQFQ1SYEGJ` | model / character | `Library/Models/Characters/じーん/Tda式ルカ モリガン風 Ver1.05 Nude` | 49 / 19,452,467 | 简评、明确许可 |

**文件核对**　49 种不同内容，单份集合 19,452,467 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M83SR42GSZSAG7WQFQ1SYEGJ`，最终目录 `Library/Models/Characters/じーん/Tda式ルカ モリガン風 Ver1.05 Nude`。

1. `merge` 保留 `01M83SR42GSZSAG7WQFQ1SYEGJ`，归并 `01M81T6SV4HYX1MWC38HQZHMQN`。沿用保留记录的其余字段。
2. `quarantine` `01M81T6SV4HYX1MWC38HQZHMQN` → `_Duplicates/kagura-review-20261124/01M81T6SV4HYX1MWC38HQZHMQN/tdalukanude`。

**取舍**　确认 Nude 版本的出处后 merge；候选保留现有 key 和源目录说明，未经核实不为该衍生版本追加作者或放宽许可。

**人工确认**　确认 Nude 是否为独立第三方改版；本包 readme 对可再配布和使用另有限制，保留具体来源说明。

<a id="g72"></a>

### 72 · A · Base_Down_Low

**自动草稿候选。** 两份都在歌曲「Base_Down_Low」的同一个 song ID 下，part 都是 pack；全部 3 种文件内容相同。两侧相对路径相同，保留同歌下无冗余后缀的 key。

**署名依据**　文件名可以确认歌曲，未发现动作署名。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T95WTGB0READXDEPMF9GB` | motion_part / pack | `Library/Motions/Base_Down_Low__Unknown/pack__Unknown` | 3 / 19,312,050 | 无 |
| `01M81TA50SAEY31XA57W92YBAK` | motion_part / pack | `Library/Motions/Base_Down_Low__Unknown/pack__Unknown_Base_Down_Low` | 3 / 19,312,050 | 无 |

**父歌**　`01M81T95WTEJ18YBYNGW8MAXZR` `{"title":"Base_Down_Low","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 19,312,050 字节。所有相对路径与内容一致。

保留 `01M81T95WTGB0READXDEPMF9GB`，最终目录 `Library/Motions/Base_Down_Low__Unknown/pack__Unknown`。

1. `merge` 保留 `01M81T95WTGB0READXDEPMF9GB`，归并 `01M81TA50SAEY31XA57W92YBAK`。沿用保留记录的其余字段。
2. `quarantine` `01M81TA50SAEY31XA57W92YBAK` → `_Duplicates/kagura-review-20261124/01M81TA50SAEY31XA57W92YBAK/pack__Unknown_Base_Down_Low`。

<a id="g73"></a>

### 73 · A · 『プラチナ』-shin'in future Mix- 愛川こずえ様Ver

**自动草稿候选。** 两份都在歌曲「『プラチナ』-shin'in future Mix- 愛川こずえ様Ver」的同一个 song ID 下，part 都是 pack；全部 3 种文件内容相同。保留可独立使用的包及其内部文件路径。

**署名依据**　nac_platinak/読んでね.txt 落款「なつき（～してもらいましたP）」。愛川こずえ 是舞者。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9MSPP3693HPD7BYXZA59` | motion_part / pack | `Library/Motions/『プラチナ』-shin'in future Mix- 愛川こずえ様Ver__Unknown/pack__Unknown` | 3 / 18,570,668 | 无 |
| `01M81T9NQJEAAT0BSGZ0BK1XK3` | motion_part / pack | `Library/Motions/『プラチナ』-shin'in future Mix- 愛川こずえ様Ver__Unknown/pack__Unknown_『プラチナ』-shin'in future Mi` | 3 / 18,570,668 | 无 |

**父歌**　`01M81T9MSP4JNZW77XKK235J47` `{"title":"『プラチナ』-shin'in future Mix- 愛川こずえ様Ver","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 18,570,668 字节。所有相对路径与内容一致。

保留 `01M81T9MSPP3693HPD7BYXZA59`，最终目录 `Library/Motions/『プラチナ』-shin'in future Mix- 愛川こずえ様Ver__Unknown/pack_なつき`。

1. `merge` 保留 `01M81T9MSPP3693HPD7BYXZA59`，归并 `01M81T9NQJEAAT0BSGZ0BK1XK3`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9NQJEAAT0BSGZ0BK1XK3` → `_Duplicates/kagura-review-20261124/01M81T9NQJEAAT0BSGZ0BK1XK3/pack__Unknown_『プラチナ』-shin'in future Mi`。
3. `rekey` `01M81T9MSPP3693HPD7BYXZA59` → `Library/Motions/『プラチナ』-shin'in future Mix- 愛川こずえ様Ver__Unknown/pack_なつき`。`/key` → `{"part":"pack","author":"なつき"}`。

<a id="g74"></a>

### 74 · D · TDA Electric Love Miku / 原画

**需要人工确认。** 两套负载都含可加载的 TDA Electric Love Miku.pmx、相同材质和同一份模型编辑说明。“原画”一侧没有 manifest 预览、约定预览名或能区分为单独原画的证据。倾向整模包错标或误拷，不能仅从名字确认它原本该存哪幅原画。

**署名依据**　两侧相同内容的 Readme (Xoriu).txt 与 PMX 都署改模 Xoriu，设计源为 Project Mirai 2，TDA 等列入素材署名。Electric Love 记录未填 license，“原画”的 modify:true 和引用来自 Tda 的基础说明。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81RK1YWZP0W8GANRYST8DQS` | model / character | `Library/Models/Characters/_Unknown/TDA Electric Love Miku` | 26 / 18,375,900 | 无 |
| `01M81T6V2S287XP92TWBN93R7P` | model / character | `Library/Models/Characters/_Unknown/原画` | 26 / 18,375,900 | 明确许可 |

**文件核对**　26 种不同内容，单份集合 18,375,900 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

两份均保留，未编列 merge 或 quarantine。

**`01M81RK1YWZP0W8GANRYST8DQS`**

- 许可。`null`
- readme。`ReadMe_Tda.txt`、`ReadMe_Tda_English.txt`、`ReadMe_金子卵黄.txt`、`Readme (Xoriu).txt`
- 预览路径。manifest 与常规预览路径均无
- 负载文件清单。`CC.spa`、`ReadMe_Tda.txt`、`ReadMe_Tda_English.txt`、`ReadMe_金子卵黄.txt`、`Readme (Xoriu).txt`、`TDA Electric Love Miku.pmx`、`body00_MikuAp_b.tga`、`body00_s.bmp`、`body01_s.bmp`、`body02_MikuAp.tga`、`boots.png`、`fff3.tga`、`h2.png`、`hair_s.bmp`、`hika2.png`、`hsp3.png`、`k_Hada.png`、`k_hika3.png`、`k_huku5.png`、`skin_s.bmp`、`t007.bmp`、`toon_defo.bmp`、`toon_hair.bmp`、`toon_skin.bmp`、`top5.png`、`top5e.png`

**`01M81T6V2S287XP92TWBN93R7P`**

- 许可。`{"modify":true,"readme":"ReadMe_Tda.txt"}`
- readme。`ReadMe_Tda.txt`、`ReadMe_Tda_English.txt`、`ReadMe_嬥巕棏墿.txt`、`Readme (Xoriu).txt`
- 预览路径。manifest 与常规预览路径均无
- 负载文件清单。`CC.spa`、`ReadMe_Tda.txt`、`ReadMe_Tda_English.txt`、`ReadMe_嬥巕棏墿.txt`、`Readme (Xoriu).txt`、`TDA Electric Love Miku.pmx`、`body00_MikuAp_b.tga`、`body00_s.bmp`、`body01_s.bmp`、`body02_MikuAp.tga`、`boots.png`、`fff3.tga`、`h2.png`、`hair_s.bmp`、`hika2.png`、`hsp3.png`、`k_Hada.png`、`k_hika3.png`、`k_huku5.png`、`skin_s.bmp`、`t007.bmp`、`toon_defo.bmp`、`toon_hair.bmp`、`toon_skin.bmp`、`top5.png`、`top5e.png`

**人工确认**　确认“原画”目录原意及是否有缺失的独立插图。确认许可证涵盖成品后再考虑保留 Xoriu 的 Electric Love 记录。

<a id="g75"></a>

### 75 · A · Domineer

**自动草稿候选。** 两份都在歌曲「Domineer」的同一个 song ID 下，part 都是 pack；全部 7 种文件内容相同。保留无冗余包装的主包，保留其 files、license.readme 以及匹配保留侧目录的内部路径。

**署名依据**　主包 Readme-Rules.txt 指名 NatsumiSan 为动作制作者；两侧对应说明的 BLAKE3 相同。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T930C04GHM9Q5CDRJZZ2K` | motion_part / pack | `Library/Motions/Domineer__Unknown/pack__Unknown` | 7 / 17,324,192 | 无 |
| `01M81T9JVYKRDWC36K4F46DWKX` | motion_part / pack | `Library/Motions/Domineer__Unknown/pack__Unknown_Herrscher of Sentience -` | 7 / 17,324,192 | 无 |

**父歌**　`01M81T930C448ADSJ60DA4SFD7` `{"title":"Domineer","composer":"_Unknown"}`。
**文件核对**　7 种不同内容，单份集合 17,324,192 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T930C04GHM9Q5CDRJZZ2K`，最终目录 `Library/Motions/Domineer__Unknown/pack_NatsumiSan`。

1. `merge` 保留 `01M81T930C04GHM9Q5CDRJZZ2K`，归并 `01M81T9JVYKRDWC36K4F46DWKX`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9JVYKRDWC36K4F46DWKX` → `_Duplicates/kagura-review-20261124/01M81T9JVYKRDWC36K4F46DWKX/pack__Unknown_Herrscher of Sentience -`。
3. `rekey` `01M81T930C04GHM9Q5CDRJZZ2K` → `Library/Motions/Domineer__Unknown/pack_NatsumiSan`。`/key` → `{"part":"pack","author":"NatsumiSan"}`。

<a id="g76"></a>

### 76 · C · Lat初音Ver2.3单马尾 / Lat初音Ver2.3单马尾 人物模型

**自动草稿候选。** 相同单马尾 PMD 和贴图，第二份仅多一层同名目录和“人物模型”后缀，优先保留简洁入口。

**署名依据**　PMD 署 Lat 的基础模型及转换。随附傟偁偳傔.txt 中，まお署名说明自己移动双马尾、移植发带，接下来才是 Lat 的原说明；最终单马尾版本应归まお。
**建议显示名**　`Lat初音Ver2.3单马尾`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81VNKR884HCN2PY5R9J9KRE` | model / character | `Library/Models/Characters/_Unknown/Lat初音Ver2.3单马尾` | 12 / 16,384,560 | 无 |
| `01M81TXDZ0HGKC96SF9S8QG64D` | model / character | `Library/Models/Characters/_Unknown/Lat初音Ver2.3单马尾 人物模型` | 12 / 16,384,560 | 无 |

**文件核对**　12 种不同内容，单份集合 16,384,560 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81VNKR884HCN2PY5R9J9KRE`，最终目录 `Library/Models/Characters/まお/Lat初音Ver2.3单马尾`。

1. `merge` 保留 `01M81VNKR884HCN2PY5R9J9KRE`，归并 `01M81TXDZ0HGKC96SF9S8QG64D`。沿用保留记录的其余字段。
2. `quarantine` `01M81TXDZ0HGKC96SF9S8QG64D` → `_Duplicates/kagura-review-20261124/01M81TXDZ0HGKC96SF9S8QG64D/Lat初音Ver2.3单马尾 人物模型`。
3. `rekey` `01M81VNKR884HCN2PY5R9J9KRE` → `Library/Models/Characters/まお/Lat初音Ver2.3单马尾`。`/key` → `{"author":"まお","name":"Lat初音Ver2.3单马尾"}`。

**取舍**　merge 后 rekey 作者；保留名称中的 Lat 作为基础模型系列。

<a id="g77"></a>

### 77 · C · TDA luka Cat Bikin / TDA luka Cat Bikin(1)

**自动草稿候选。** 两份相对路径和哈希均一致。(1) 是副本标记，当前两个 key 将 Bikini 截成 Bikin；主模型及内部名可还原全称。

**署名依据**　rules (MisuTochi).txt 与 PMX 都指定使用模型时须署 MisuTochi。PMX 同列 TDA 等协作者，并未把所有素材归为 MisuTochi。
**建议显示名**　`TDA Luka Cat Bikini`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T6RKEJSS9Z3VDH6ZTR763` | model / character | `Library/Models/Characters/_Unknown/TDA luka Cat Bikin` | 22 / 14,605,403 | 无 |
| `01M81T6RKKFH5S75YP56H1H6F2` | model / character | `Library/Models/Characters/_Unknown/TDA luka Cat Bikin(1)` | 22 / 14,605,403 | 无 |

**文件核对**　22 种不同内容，单份集合 14,605,403 字节。所有相对路径与内容一致。

保留 `01M81T6RKEJSS9Z3VDH6ZTR763`，最终目录 `Library/Models/Characters/MisuTochi/TDA Luka Cat Bikini`。

1. `merge` 保留 `01M81T6RKEJSS9Z3VDH6ZTR763`，归并 `01M81T6RKKFH5S75YP56H1H6F2`。字段 `/license` → `{"readme":"TDA luka Cat Bikini/rules (MisuTochi).txt","modify":false,"redistribute":false}`。
2. `quarantine` `01M81T6RKKFH5S75YP56H1H6F2` → `_Duplicates/kagura-review-20261124/01M81T6RKKFH5S75YP56H1H6F2/TDA luka Cat Bikin(1)`。
3. `rekey` `01M81T6RKEJSS9Z3VDH6ZTR763` → `Library/Models/Characters/MisuTochi/TDA Luka Cat Bikini`。`/key` → `{"author":"MisuTochi","name":"TDA Luka Cat Bikini"}`。

**取舍**　merge 后 rekey，以成品 rules 替换素体说明作为 license.readme；其中明确禁止编辑和再配布。

<a id="g78"></a>

### 78 · C · Tda+改变式+Lolita+女仆+Miku+V1(1).0 / Tda+改变式+Lolita+女仆+Miku+V1.0

**自动草稿候选。** 两份均包含相同的公配版 V1.0 主模型，路径与哈希相同；保留没有 (1) 的键。

**署名依据**　模型注释署编辑 Yudou；Read Me.txt 将改造者署 Yudou，结尾落款 Yudou优斗。WYKP、TDA 等为基础部件来源。
**建议显示名**　`Tda+改变式+Lolita+女仆+Miku+V1.0`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T6ST81S4A8JJ88R6021PN` | model / character | `Library/Models/Characters/Yudou优斗/Tda+改变式+Lolita+女仆+Miku+V1(1).0` | 14 / 14,543,843 | 无 |
| `01M81T6STBNE8AE4FPYWTV0833` | model / character | `Library/Models/Characters/Yudou优斗/Tda+改变式+Lolita+女仆+Miku+V1.0` | 14 / 14,543,843 | 无 |

**文件核对**　14 种不同内容，单份集合 14,543,843 字节。所有相对路径与内容一致。

保留 `01M81T6STBNE8AE4FPYWTV0833`，最终目录 `Library/Models/Characters/Yudou优斗/Tda+改变式+Lolita+女仆+Miku+V1.0`。

1. `merge` 保留 `01M81T6STBNE8AE4FPYWTV0833`，归并 `01M81T6ST81S4A8JJ88R6021PN`。沿用保留记录的其余字段。
2. `quarantine` `01M81T6ST81S4A8JJ88R6021PN` → `_Duplicates/kagura-review-20261124/01M81T6ST81S4A8JJ88R6021PN/Tda+改变式+Lolita+女仆+Miku+V1(1).0`。

**取舍**　merge；沿用无 (1) 的 key、已有 readme 和主变体。

<a id="g79"></a>

### 79 · C · TDA Sailor Haku / TDA Sailor Haku (弱音模型)

**需要人工确认。** 主文件均为 TDA Sailor Haku.pmx，第二个 key 的弱音模型仅为分类后缀；其预览与第一份 haku.png 相同。两份 modify:true 都来自 Tda 基础模型文档。

**署名依据**　主模型注释为空。配套说明所署 Tda、金子卵黄对应初音素体或服装，无法据此判断水手服弱音的最终改模者。
**建议显示名**　`TDA Sailor Haku`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T6S7MJMJ4C5ZGWV6RG3CP` | model / character | `Library/Models/Characters/_Unknown/TDA Sailor Haku` | 29 / 14,129,359 | 明确许可 |
| `01M822G41EEX0WFKT7G2GYEHAJ` | model / character | `Library/Models/Characters/_Unknown/TDA Sailor Haku (弱音模型)` | 29 / 14,129,359 | 明确许可 |

**文件核对**　29 种不同内容，单份集合 14,129,359 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T6S7MJMJ4C5ZGWV6RG3CP`，最终目录 `Library/Models/Characters/_Unknown/TDA Sailor Haku`。

1. `merge` 保留 `01M81T6S7MJMJ4C5ZGWV6RG3CP`，归并 `01M822G41EEX0WFKT7G2GYEHAJ`。字段 `/preview` → `["haku.png"]`。
2. `quarantine` `01M822G41EEX0WFKT7G2GYEHAJ` → `_Duplicates/kagura-review-20261124/01M822G41EEX0WFKT7G2GYEHAJ/TDA Sailor Haku (弱音模型)`。

**取舍**　候选 merge 至无后缀记录并按哈希映射补入预览，保留 _Unknown；关于成品 modify 的现有值，须核对后再确认。

**人工确认**　最终改模者没有署名，现有许可明确值并非来自该成品。

<a id="g80"></a>

### 80 · C · Tda Sexy Bunny （短裤版） / Tda Sexy Bunny （短裤版） (初音模型)

**需要人工确认。** 短裤版主 PMX 名相同，两份都已声明预览：英文文件名与中文文件名内容相同。第二份 key 加的是角色类别说明。

**署名依据**　PMX 联合列出 Tda、WYKP、花僧逗奶、大狗头、iRon0129、DOA5，未列分工、唯一作者或最终编辑；无本包 readme。
**建议显示名**　`Tda Sexy Bunny （短裤版）`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T6S84PF9ZCB2ZRF28PW5J` | model / character | `Library/Models/Characters/_Unknown/Tda Sexy Bunny （短裤版）` | 18 / 13,792,218 | 无 |
| `01M822G2S64KBD0D5XDTNDN7KJ` | model / character | `Library/Models/Characters/_Unknown/Tda Sexy Bunny （短裤版） (初音模型)` | 18 / 13,792,218 | 无 |

**文件核对**　18 种不同内容，单份集合 13,792,218 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T6S84PF9ZCB2ZRF28PW5J`，最终目录 `Library/Models/Characters/_Unknown/Tda Sexy Bunny （短裤版）`。

1. `merge` 保留 `01M81T6S84PF9ZCB2ZRF28PW5J`，归并 `01M822G2S64KBD0D5XDTNDN7KJ`。沿用保留记录的其余字段。
2. `quarantine` `01M822G2S64KBD0D5XDTNDN7KJ` → `_Duplicates/kagura-review-20261124/01M822G2S64KBD0D5XDTNDN7KJ/Tda Sexy Bunny （短裤版） (初音模型)`。

**取舍**　候选 merge 至保留 Preview.png 的无后缀 ID；key.author 暂留 _Unknown，不把第一位署名者当作唯一作者。

**人工确认**　确认联合署名应如何记录、是否需要成品配布来源，再决定是否入草稿。

<a id="g81"></a>

### 81 · C · DT Winter Miku

**自动草稿候选。** DT Winter Miku 的两份 PMD 与素材逐路径一致；保留有编辑者和源 readme 的记录。PMD 内部名只有初音ミク，季节仍由文件包/记录区分；不与其他季节跨组合并。

**署名依据**　readme_Miku DT.txt 与 PMD 的英文注释均将模型提取署 Hanaminasho、编辑署 Jomomonogm；双马尾由 sakuraxangel31 提供。
**建议显示名**　`DT Winter Miku`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXKR6VHWHE9VHK4Q5EE1D` | model / character | `Library/Models/Characters/Jomomonogm/DT Winter Miku` | 39 / 12,960,133 | 明确许可 |
| `01M81RK1Y34X1P9DQF06M3C5RZ` | model / character | `Library/Models/Characters/_Unknown/DT Winter Miku` | 39 / 12,960,133 | 无 |

**文件核对**　39 种不同内容，单份集合 12,960,133 字节。所有相对路径与内容一致。

保留 `01M81TXKR6VHWHE9VHK4Q5EE1D`，最终目录 `Library/Models/Characters/Jomomonogm/DT Winter Miku`。

1. `merge` 保留 `01M81TXKR6VHWHE9VHK4Q5EE1D`，归并 `01M81RK1Y34X1P9DQF06M3C5RZ`。字段 `/name_original` → `"初音ミク"`。
2. `quarantine` `01M81RK1Y34X1P9DQF06M3C5RZ` → `_Duplicates/kagura-review-20261124/01M81RK1Y34X1P9DQF06M3C5RZ/DT Winter Miku`。

**取舍**　merge 到已署 Jomomonogm 的 ID，保留季节 key、许可及变体，并纳入原 scanner 记录的 name_original。

<a id="g82"></a>

### 82 · C · DT Summer Miku

**自动草稿候选。** DT Summer Miku 的两份 PMD 与素材逐路径一致；保留有编辑者和源 readme 的记录。PMD 内部名只有初音ミク，季节仍由文件包/记录区分；不与其他季节跨组合并。

**署名依据**　readme_Miku DT.txt 与 PMD 的英文注释均将模型提取署 Hanaminasho、编辑署 Jomomonogm；双马尾由 sakuraxangel31 提供。
**建议显示名**　`DT Summer Miku`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXKR0KWBH6BS2HZ0Y9KKZ` | model / character | `Library/Models/Characters/Jomomonogm/DT Summer Miku` | 39 / 12,725,799 | 明确许可 |
| `01M81RK1Y0TY2SGXMS3XQETHM9` | model / character | `Library/Models/Characters/_Unknown/DT Summer Miku` | 39 / 12,725,799 | 无 |

**文件核对**　39 种不同内容，单份集合 12,725,799 字节。所有相对路径与内容一致。

保留 `01M81TXKR0KWBH6BS2HZ0Y9KKZ`，最终目录 `Library/Models/Characters/Jomomonogm/DT Summer Miku`。

1. `merge` 保留 `01M81TXKR0KWBH6BS2HZ0Y9KKZ`，归并 `01M81RK1Y0TY2SGXMS3XQETHM9`。字段 `/name_original` → `"初音ミク"`。
2. `quarantine` `01M81RK1Y0TY2SGXMS3XQETHM9` → `_Duplicates/kagura-review-20261124/01M81RK1Y0TY2SGXMS3XQETHM9/DT Summer Miku`。

**取舍**　merge 到已署 Jomomonogm 的 ID，保留季节 key、许可及变体，并纳入原 scanner 记录的 name_original。

<a id="g83"></a>

### 83 · C · DT Autumn Miku

**自动草稿候选。** DT Autumn Miku 的两份 PMD 与素材逐路径一致；保留有编辑者和源 readme 的记录。PMD 内部名只有初音ミク，季节仍由文件包/记录区分；不与其他季节跨组合并。

**署名依据**　readme_Miku DT.txt 与 PMD 的英文注释均将模型提取署 Hanaminasho、编辑署 Jomomonogm；双马尾由 sakuraxangel31 提供。
**建议显示名**　`DT Autumn Miku`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TXKQTWHMN4EPJEX49Z2QC` | model / character | `Library/Models/Characters/Jomomonogm/DT Autumn Miku` | 39 / 12,641,875 | 明确许可 |
| `01M81RK1XT4CJEXA3A3PWW5WZP` | model / character | `Library/Models/Characters/_Unknown/DT Autumn Miku` | 39 / 12,641,875 | 无 |

**文件核对**　39 种不同内容，单份集合 12,641,875 字节。所有相对路径与内容一致。

保留 `01M81TXKQTWHMN4EPJEX49Z2QC`，最终目录 `Library/Models/Characters/Jomomonogm/DT Autumn Miku`。

1. `merge` 保留 `01M81TXKQTWHMN4EPJEX49Z2QC`，归并 `01M81RK1XT4CJEXA3A3PWW5WZP`。字段 `/name_original` → `"初音ミク"`。
2. `quarantine` `01M81RK1XT4CJEXA3A3PWW5WZP` → `_Duplicates/kagura-review-20261124/01M81RK1XT4CJEXA3A3PWW5WZP/DT Autumn Miku`。

**取舍**　merge 到已署 Jomomonogm 的 ID，保留季节 key、许可及变体，并纳入原 scanner 记录的 name_original。

<a id="g84"></a>

### 84 · A · Very Nice

**自动草稿候选。** 两份都在歌曲「Very Nice」的同一个 song ID 下，part 都是 pack；全部 3 种文件内容相同。保留现有具名作者记录与可辨识的 SEVENTEEN 音频文件名。

**署名依据**　原迁移清单 source「动作/合集1/Very Nice [motion by ureshiiiiii]」明确标为动作作者，与现有具名包相符。包内无 readme。
**来源记录**　`G:/MMD/_design/migration_runs/first/plan.jsonl`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9E7H7A2MRCWM9VD1TZEG` | motion_part / pack | `Library/Motions/Very Nice__Unknown/pack__Unknown` | 3 / 11,966,681 | 无 |
| `01M81TACFVFDPQZ482JDQHJME3` | motion_part / pack | `Library/Motions/Very Nice__Unknown/pack_ureshiiiiii` | 3 / 11,966,681 | 无 |

**父歌**　`01M81T9E7H4CS8JJ8W2W0Q71CF` `{"title":"Very Nice","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 11,966,681 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81TACFVFDPQZ482JDQHJME3`，最终目录 `Library/Motions/Very Nice__Unknown/pack_ureshiiiiii`。

1. `merge` 保留 `01M81TACFVFDPQZ482JDQHJME3`，归并 `01M81T9E7H7A2MRCWM9VD1TZEG`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9E7H7A2MRCWM9VD1TZEG` → `_Duplicates/kagura-review-20261124/01M81T9E7H7A2MRCWM9VD1TZEG/pack__Unknown`。

<a id="g85"></a>

### 85 · A · 『プラチナ』-shin'in future Mix- ミンカ・リー様Ver

**自动草稿候选。** 两份都在歌曲「『プラチナ』-shin'in future Mix- ミンカ・リー様Ver」的同一个 song ID 下，part 都是 pack；全部 3 种文件内容相同。保留可独立使用的包及其内部文件路径。

**署名依据**　nac_platinam/読んでね.txt 落款「なつき（～してもらいましたP）」。ミンカ・リー 是舞者。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9MSKZG242BM6VFRE3D28` | motion_part / pack | `Library/Motions/『プラチナ』-shin'in future Mix- ミンカ・リー様Ver__Unknown/pack__Unknown` | 3 / 10,968,022 | 无 |
| `01M81T9NQGZET3BFJYHHDC0MYN` | motion_part / pack | `Library/Motions/『プラチナ』-shin'in future Mix- ミンカ・リー様Ver__Unknown/pack__Unknown_『プラチナ』-shin'in future Mi` | 3 / 10,968,022 | 无 |

**父歌**　`01M81T9MSKWXEG3R1NS55WF4FW` `{"title":"『プラチナ』-shin'in future Mix- ミンカ・リー様Ver","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 10,968,022 字节。所有相对路径与内容一致。

保留 `01M81T9MSKZG242BM6VFRE3D28`，最终目录 `Library/Motions/『プラチナ』-shin'in future Mix- ミンカ・リー様Ver__Unknown/pack_なつき`。

1. `merge` 保留 `01M81T9MSKZG242BM6VFRE3D28`，归并 `01M81T9NQGZET3BFJYHHDC0MYN`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9NQGZET3BFJYHHDC0MYN` → `_Duplicates/kagura-review-20261124/01M81T9NQGZET3BFJYHHDC0MYN/pack__Unknown_『プラチナ』-shin'in future Mi`。
3. `rekey` `01M81T9MSKZG242BM6VFRE3D28` → `Library/Motions/『プラチナ』-shin'in future Mix- ミンカ・リー様Ver__Unknown/pack_なつき`。`/key` → `{"part":"pack","author":"なつき"}`。

<a id="g86"></a>

### 86 · C · Stage 38 / Stage 38 (圣诞绷带服弱音)

**自动草稿候选。** 13 个文件均同路径、同哈希。第二份 name 已写 Stage 38，note 指明“圣诞绷带服弱音”是原模型包的上级目录，并解释场景与配套角色名的关系。保留有原出处 note 的 ID。

**署名依据**　Stage.pmd 的编辑者和 MOD 作成者均为占位符；Tso2Pmd 是转换工具，TechArts3D 是版权标识。无独立 readme，沿用 _Unknown。
**建议显示名**　`Stage 38`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81RJTW5EZ8Y7T9FH6AG2TV5` | stage / stage | `Library/Stages/_Unknown/Stage 38` | 13 / 9,993,664 | 无 |
| `01M825AYE05A00JG11CA2FCGYH` | stage / stage | `Library/Stages/_Unknown/Stage 38 (圣诞绷带服弱音)` | 13 / 9,993,664 | 简评 |

**文件核对**　13 种不同内容，单份集合 9,993,664 字节。所有相对路径与内容一致。

保留 `01M825AYE05A00JG11CA2FCGYH`，最终目录 `Library/Stages/_Unknown/Stage 38`。

1. `merge` 保留 `01M825AYE05A00JG11CA2FCGYH`，归并 `01M81RJTW5EZ8Y7T9FH6AG2TV5`。字段 `/bundled_effects` → `[]`。
2. `quarantine` `01M81RJTW5EZ8Y7T9FH6AG2TV5` → `_Duplicates/kagura-review-20261124/01M81RJTW5EZ8Y7T9FH6AG2TV5/Stage 38`。
3. `rekey` `01M825AYE05A00JG11CA2FCGYH` → `Library/Stages/_Unknown/Stage 38`。`/key` → `{"author":"_Unknown","name":"Stage 38"}`。

**取舍**　merge 后 rekey 至 Stage 38；原占位目录须先归档。保留 note，从 scanner 记录沿用空的 bundled_effects。

<a id="g87"></a>

### 87 · C · Doflamingo / 海贼王模型

**自动草稿候选。** 第二份用“海贼王模型”作上级分类，包内目录为堂吉诃德·多佛朗明哥。两边 PMX 和角色内部名均为 Doflamingo；保留按角色命名的 ID。另一份预览图可按哈希映射到本侧 prev.png。

**署名依据**　Doflamingo.pmx 的名称与注释都只给角色名，没署模型作者，也无可用 readme。保留 _Unknown。
**建议显示名**　`Doflamingo`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T6N8HJK6GCSMB9Q4ZVKPR` | model / character | `Library/Models/Characters/_Unknown/Doflamingo` | 16 / 9,387,706 | 无 |
| `01M81T6W8GA1V8YCZWVZY9HJ5E` | model / character | `Library/Models/Characters/_Unknown/海贼王模型` | 16 / 9,387,706 | 无 |

**文件核对**　16 种不同内容，单份集合 9,387,706 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81T6N8HJK6GCSMB9Q4ZVKPR`，最终目录 `Library/Models/Characters/_Unknown/Doflamingo`。

1. `merge` 保留 `01M81T6N8HJK6GCSMB9Q4ZVKPR`，归并 `01M81T6W8GA1V8YCZWVZY9HJ5E`。字段 `/preview` → `["prev.png"]`。
2. `quarantine` `01M81T6W8GA1V8YCZWVZY9HJ5E` → `_Duplicates/kagura-review-20261124/01M81T6W8GA1V8YCZWVZY9HJ5E/海贼王模型`。

**取舍**　merge；把另一份的预览语义保留到现有相对路径 prev.png。

<a id="g88"></a>

### 88 · C · romantic hotel / 场景奢华典雅

**自动草稿候选。** “场景奢华典雅”的实际内层目录叫 romantic hotel，完整路径去掉这层后与另一份对应。材质说明列有床具、墙、灯、桌和酒具；有依据保留直接对应发行目录的 romantic hotel。

**署名依据**　PMD 中作者和编辑者为占位符，TechArts3D 是版权标记；カスタム少女.txt 是材质清单，未署成品作者。保留 _Unknown。
**建议显示名**　`romantic hotel`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81RJTT7GNS760RP8BDMSSJF` | stage / stage | `Library/Stages/_Unknown/romantic hotel` | 25 / 8,491,337 | 无 |
| `01M81T0CKKA557QXZ1C6C80BT3` | stage / stage | `Library/Stages/_Unknown/场景奢华典雅` | 25 / 8,491,337 | 无 |

**文件核对**　25 种不同内容，单份集合 8,491,337 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81RJTT7GNS760RP8BDMSSJF`，最终目录 `Library/Stages/_Unknown/romantic hotel`。

1. `merge` 保留 `01M81RJTT7GNS760RP8BDMSSJF`，归并 `01M81T0CKKA557QXZ1C6C80BT3`。沿用保留记录的其余字段。
2. `quarantine` `01M81T0CKKA557QXZ1C6C80BT3` → `_Duplicates/kagura-review-20261124/01M81T0CKKA557QXZ1C6C80BT3/场景奢华典雅`。

**取舍**　merge，保留根目录入口与清晰的主变体标签。

<a id="g89"></a>

### 89 · D · 包厢舞台 / 酒馆

**需要人工确认。** 23 条路径和内容逐条相同，含一套 STAGE21 场景和一套 drinks 模型。两份 manifest 均把 drinks 标为 main。包厢和酒馆可以是同一环境的描述，也可能是对主舞台和饮品配件的不同归档意图；没有配布标题或独立预览可区分。

**署名依据**　两份各有 STAGE21/カスタム少女.txt 和 drinks/カスタム少女.txt，内容是材质/项目列表；两套 PMD 署名占位，仅可识别 Tso2Pmd 转换工具，许可均未录入。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T0C17GQTNP3F1AHB4EDAB` | stage / stage | `Library/Stages/_Unknown/包厢舞台` | 23 / 7,927,409 | 无 |
| `01M8200KJDRDTDM5XN2SFN61EX` | stage / stage | `Library/Stages/_Unknown/酒馆` | 23 / 7,927,409 | 无 |

**文件核对**　22 种不同内容，单份集合 7,924,941 字节。所有相对路径与内容一致。

两份均保留，未编列 merge 或 quarantine。

**`01M81T0C17GQTNP3F1AHB4EDAB`**

- 许可。`null`
- readme。`STAGE21/カスタム少女.txt`、`drinks/カスタム少女.txt`
- 预览路径。manifest 与常规预览路径均无
- 负载文件清单。`STAGE21/TPOConfig.xml`、`STAGE21/t000.bmp`、`STAGE21/t001.bmp`、`STAGE21/t002.bmp`、`STAGE21/t003.bmp`、`STAGE21/t004.bmp`、`STAGE21/t005.bmp`、`STAGE21/t006.bmp`、`STAGE21/t007.bmp`、`STAGE21/t008.bmp`、`STAGE21/t009.bmp`、`STAGE21/t010.bmp`、`STAGE21/t011.bmp`、`STAGE21/t012.bmp`、`STAGE21/カスタム少女.pmd`、`STAGE21/カスタム少女.txt`、`drinks/TPOConfig.xml`、`drinks/t000.bmp`、`drinks/t001.bmp`、`drinks/t002.bmp`、`drinks/t003.bmp`、`drinks/カスタム少女.pmd`、`drinks/カスタム少女.txt`

**`01M8200KJDRDTDM5XN2SFN61EX`**

- 许可。`null`
- readme。`STAGE21/カスタム少女.txt`、`drinks/カスタム少女.txt`
- 预览路径。manifest 与常规预览路径均无
- 负载文件清单。`STAGE21/TPOConfig.xml`、`STAGE21/t000.bmp`、`STAGE21/t001.bmp`、`STAGE21/t002.bmp`、`STAGE21/t003.bmp`、`STAGE21/t004.bmp`、`STAGE21/t005.bmp`、`STAGE21/t006.bmp`、`STAGE21/t007.bmp`、`STAGE21/t008.bmp`、`STAGE21/t009.bmp`、`STAGE21/t010.bmp`、`STAGE21/t011.bmp`、`STAGE21/t012.bmp`、`STAGE21/カスタム少女.pmd`、`STAGE21/カスタム少女.txt`、`drinks/TPOConfig.xml`、`drinks/t000.bmp`、`drinks/t001.bmp`、`drinks/t002.bmp`、`drinks/t003.bmp`、`drinks/カスタム少女.pmd`、`drinks/カスタム少女.txt`

**人工确认**　核对原压缩包标题和两方期望的主入口；不能从相同字节推断任一侧错放。

<a id="g90"></a>

### 90 · B · As if its your last SHORT (Cookies X Cream / As if its your last

**需要人工确认。** 两份素材属于不同父歌，已核对两边父歌的自有负载、子素材、所有入边及对应 VMD。两个父歌各只有该包；WAV 哈希、主体 2,452 帧（81.73 秒）和镜头 645 帧完全一致，指向同一 SHORT 剪辑。以有 Cookies X Cream 标注的父歌为保留项，把剪辑信息放入 edition。库内另两份曲名相近的歌含 2,838/2,954 帧包，超出本组，未核对到与此包整份相同；不给它们合并建议。

**署名依据**　SHORT (Cookies X Cream) 为版本来源线索，音频名的 ROSEXLISA 描述所用音轨；无动作作者签名。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TA5Z87D65R425MRPSBH4Y` | motion_part / pack | `Library/Motions/As if its your last SHORT (Cookies X Cre__Unknown/pack__Unknown` | 3 / 4,116,336 | 无 |
| `01M81T95E7P81K7Y63KH6ZGPF0` | motion_part / pack | `Library/Motions/As if its your last__Unknown/pack__Unknown` | 3 / 4,116,336 | 无 |

**父歌**　`01M81TA5Z8EJTYVV1H5SWACAFJ` `{"title":"As if its your last SHORT (Cookies X Cream","composer":"_Unknown"}`；`01M81T95E78BF4R294TJVA2ZK2` `{"title":"As if its your last","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 4,116,336 字节。相对路径有差异，各路径对应的 BLAKE3 已核对。

保留 `01M81TA5Z87D65R425MRPSBH4Y`，最终目录 `Library/Motions/AS IF IT'S YOUR LAST(SHORT (Cookies X Cream))__Unknown/pack__Unknown`。

1. `merge` 保留 `01M81TA5Z87D65R425MRPSBH4Y`，归并 `01M81T95E7P81K7Y63KH6ZGPF0`。沿用保留记录的其余字段。
2. `quarantine` `01M81T95E7P81K7Y63KH6ZGPF0` → `_Duplicates/kagura-review-20261124/01M81T95E7P81K7Y63KH6ZGPF0/pack__Unknown`。
3. `merge` 保留 `01M81TA5Z8EJTYVV1H5SWACAFJ`，归并 `01M81T95E78BF4R294TJVA2ZK2`。沿用保留记录的其余字段。
4. `add_alias` `01M81TA5Z8EJTYVV1H5SWACAFJ` `/title_alias` → `["As if its your last SHORT (Cookies X Cream","As if its your last"]`。
5. `rekey` `01M81TA5Z8EJTYVV1H5SWACAFJ` → `Library/Motions/AS IF IT'S YOUR LAST(SHORT (Cookies X Cream))__Unknown`。`/key` → `{"title":"AS IF IT'S YOUR LAST","composer":"_Unknown","edition":"SHORT (Cookies X Cream)"}`。
6. `quarantine` `01M81T95E78BF4R294TJVA2ZK2` → `_Duplicates/kagura-review-20261124/01M81T95E78BF4R294TJVA2ZK2/As if its your last__Unknown`。

父歌的入向引用如下。

- `01M81TA5Z87D65R425MRPSBH4Y` `song` → `01M81TA5Z8EJTYVV1H5SWACAFJ`；保留原目标。
- `01M81T95E7P81K7Y63KH6ZGPF0` `song` → `01M81T95E78BF4R294TJVA2ZK2`；来源素材随本组归档。

两首父歌均无自有负载，各只有表中这一份素材。

**人工确认**　跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。

<a id="g91"></a>

### 91 · C · map_jp_garden__pmx__by_arisumatio / map_jp_garden__pmx__by_arisumatio-d9kahl

**自动草稿候选。** 文件清单完全相同。两个目录名分别是发布路径式的短名及带额外编号的版本；PMX 中有完整英文名称 Japanese Garden at Night，选择已有 ReadMe.txt 路径的记录。

**署名依据**　Japanese Garden.pmx 英文注释明确 PMX 转换由 Arisumatio 完成并列原始资源链接。ReadMe.txt 将权利归 Ilussion（保留文内拼写）；本建议将作者键用于这份 PMX 转换，不将 Arisumatio 认作原场景权利人。
**建议显示名**　`Japanese Garden at Night`

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81TX9YW3K9Y172ENJ1DN9QV` | stage / stage | `Library/Stages/_Unknown/map_jp_garden__pmx__by_arisumatio` | 22 / 3,735,503 | 无 |
| `01M81RJTS7NW62MWJ1631PC1JM` | stage / stage | `Library/Stages/_Unknown/map_jp_garden__pmx__by_arisumatio-d9kahl` | 22 / 3,735,503 | 无 |

**文件核对**　21 种不同内容，单份集合 3,733,085 字节。所有相对路径与内容一致。

保留 `01M81TX9YW3K9Y172ENJ1DN9QV`，最终目录 `Library/Stages/Arisumatio/Japanese Garden at Night`。

1. `merge` 保留 `01M81TX9YW3K9Y172ENJ1DN9QV`，归并 `01M81RJTS7NW62MWJ1631PC1JM`。字段 `/name_original` → `"Japanese Garden at Night"`；`/note` → `"PMX 转换：Arisumatio。原场景资源与权利署名见 ReadMe.txt。"`。
2. `quarantine` `01M81RJTS7NW62MWJ1631PC1JM` → `_Duplicates/kagura-review-20261124/01M81RJTS7NW62MWJ1631PC1JM/map_jp_garden__pmx__by_arisumatio-d9kahl`。
3. `rekey` `01M81TX9YW3K9Y172ENJ1DN9QV` → `Library/Stages/Arisumatio/Japanese Garden at Night`。`/key` → `{"author":"Arisumatio","name":"Japanese Garden at Night"}`。

**取舍**　merge、rekey；保留含 readme 的记录，记录转换角色，保留原许可引用。

<a id="g92"></a>

### 92 · A · 「ダメよ」PVキット一式

**自动草稿候选。** 两份都在歌曲「「ダメよ」PVキット一式」的同一个 song ID 下，part 都是 pack；全部 8 种文件内容相同。保留 PV 套件各组件、工程与 readme；包与上游动作/镜头的不同署名分别记录。

**署名依据**　Readme.txt 直接署名「トレース制作：盗賊つばき」，另列じゅんこだ的原动作和だんしゃくいも的一人镜头；这些分工不合并为别名。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9MVG7RNZ3RPG2X6W4ECJ` | motion_part / pack | `Library/Motions/「ダメよ」PVキット一式__Unknown/pack__Unknown` | 8 / 1,510,478 | 无 |
| `01M81T9NQBW6F3K9P45H5Z5J9E` | motion_part / pack | `Library/Motions/「ダメよ」PVキット一式__Unknown/pack__Unknown_「ダメよ」PVキット一式` | 8 / 1,510,478 | 无 |

**父歌**　`01M81T9MVG2GTPHW349HMVPRJX` `{"title":"「ダメよ」PVキット一式","composer":"_Unknown"}`。
**文件核对**　8 种不同内容，单份集合 1,510,478 字节。所有相对路径与内容一致。

保留 `01M81T9MVG7RNZ3RPG2X6W4ECJ`，最终目录 `Library/Motions/「ダメよ」PVキット一式__Unknown/pack_盗賊つばき`。

1. `merge` 保留 `01M81T9MVG7RNZ3RPG2X6W4ECJ`，归并 `01M81T9NQBW6F3K9P45H5Z5J9E`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9NQBW6F3K9P45H5Z5J9E` → `_Duplicates/kagura-review-20261124/01M81T9NQBW6F3K9P45H5Z5J9E/pack__Unknown_「ダメよ」PVキット一式`。
3. `rekey` `01M81T9MVG7RNZ3RPG2X6W4ECJ` → `Library/Motions/「ダメよ」PVキット一式__Unknown/pack_盗賊つばき`。`/key` → `{"part":"pack","author":"盗賊つばき"}`。

<a id="g93"></a>

### 93 · B · Freakum Dress / 洋楽モーション]ビヨンセFreakum Dress

**需要人工确认。** 两份素材属于不同父歌，已核对两边父歌的自有负载、子素材、所有入边及对应 VMD。两包均只有相同的主体、镜头及 readme；读出的最大帧号分别为 1,449 和 1,447。readme 指定 Beyoncé Freakum Dress，长标题是带歌手和「洋楽モーション」的原包装名。保留已有署名 nn 的包和简明父歌。

**署名依据**　Freakum Dress_readme.txt 落款 nn，注明所用的 Beyoncé 曲目、动作来源及单人版本。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81RK3KWZS25JEXSX78WXNN4` | motion_part / pack | `Library/Motions/Freakum Dress__Unknown/pack_nn` | 3 / 810,454 | 无 |
| `01M81TAKBJGAA2B97B720N9QB0` | motion_part / pack | `Library/Motions/洋楽モーション]ビヨンセFreakum Dress__Unknown/pack__Unknown` | 3 / 810,454 | 无 |

**父歌**　`01M81RK3KW4DG212W1RJ4AA0NY` `{"title":"Freakum Dress","composer":"_Unknown"}`；`01M81TAKBJN2A5XKCWT0DXBP1H` `{"title":"洋楽モーション]ビヨンセFreakum Dress","composer":"_Unknown"}`。
**文件核对**　3 种不同内容，单份集合 810,454 字节。所有相对路径与内容一致。

保留 `01M81RK3KWZS25JEXSX78WXNN4`，最终目录 `Library/Motions/Freakum Dress__Unknown/pack_nn`。

1. `merge` 保留 `01M81RK3KWZS25JEXSX78WXNN4`，归并 `01M81TAKBJGAA2B97B720N9QB0`。字段 `/license` → `{"readme":"Freakum Dress_readme.txt"}`。
2. `quarantine` `01M81TAKBJGAA2B97B720N9QB0` → `_Duplicates/kagura-review-20261124/01M81TAKBJGAA2B97B720N9QB0/pack__Unknown`。
3. `merge` 保留 `01M81RK3KW4DG212W1RJ4AA0NY`，归并 `01M81TAKBJN2A5XKCWT0DXBP1H`。沿用保留记录的其余字段。
4. `add_alias` `01M81RK3KW4DG212W1RJ4AA0NY` `/title_alias` → `["洋楽モーション]ビヨンセFreakum Dress"]`。
5. `quarantine` `01M81TAKBJN2A5XKCWT0DXBP1H` → `_Duplicates/kagura-review-20261124/01M81TAKBJN2A5XKCWT0DXBP1H/洋楽モーション]ビヨンセFreakum Dress__Unknown`。

父歌的入向引用如下。

- `01M81RK3KWZS25JEXSX78WXNN4` `song` → `01M81RK3KW4DG212W1RJ4AA0NY`；保留原目标。
- `01M81TAKBJGAA2B97B720N9QB0` `song` → `01M81TAKBJN2A5XKCWT0DXBP1H`；来源素材随本组归档。

两首父歌均无自有负载，各只有表中这一份素材。

**人工确认**　跨父歌操作需复核父歌标题、别名及保留目录，批准后把两首歌和两份素材作为同一草稿处理。

<a id="g94"></a>

### 94 · A · 「お茶碗にご飯が無い」

**自动草稿候选。** 两份都在歌曲「「お茶碗にご飯が無い」」的同一个 song ID 下，part 都是 pack；全部 4 种文件内容相同。两包指向同一作品 sm13719003，负载和路径一致；不因缺作者名而新造署名。

**署名依据**　一応お読み下さい。.txt 标出作品来源 sm13719003，但没有可核实的个人署名。

| 资产 ID | 类型 / kind 或 part | 库内目录 | 负载路径 / 字节 | 人工信息 |
|---|---|---|---|---|
| `01M81T9MVDN4GZJT9WAQAG3WPR` | motion_part / pack | `Library/Motions/「お茶碗にご飯が無い」__Unknown/pack__Unknown` | 4 / 143,692 | 无 |
| `01M81T9NQ8NJ3PPKFFBH7NS3ZH` | motion_part / pack | `Library/Motions/「お茶碗にご飯が無い」__Unknown/pack__Unknown_「お茶碗にご飯が無い」动作` | 4 / 143,692 | 无 |

**父歌**　`01M81T9MVD6DM30XSCAPPCSSF0` `{"title":"「お茶碗にご飯が無い」","composer":"_Unknown"}`。
**文件核对**　4 种不同内容，单份集合 143,692 字节。所有相对路径与内容一致。

保留 `01M81T9MVDN4GZJT9WAQAG3WPR`，最终目录 `Library/Motions/「お茶碗にご飯が無い」__Unknown/pack__Unknown`。

1. `merge` 保留 `01M81T9MVDN4GZJT9WAQAG3WPR`，归并 `01M81T9NQ8NJ3PPKFFBH7NS3ZH`。沿用保留记录的其余字段。
2. `quarantine` `01M81T9NQ8NJ3PPKFFBH7NS3ZH` → `_Duplicates/kagura-review-20261124/01M81T9NQ8NJ3PPKFFBH7NS3ZH/pack__Unknown_「お茶碗にご飯が無い」动作`。
