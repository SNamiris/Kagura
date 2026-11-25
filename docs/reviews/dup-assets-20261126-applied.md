# 2026-11-26 整份重复资产合并执行记录

已执行 78 组自动候选，归档 79 份资产。剩余 16 组整份重复与原报告的人工确认名单逐 ID 一致。[逐组审查](dup-assets-20261124.md) · [原报告 JSON](dup-assets-20261124.json)

## 执行前复核

- 关闭 Kagura 后，通过 SQLite 的立即事务阻止桌面索引被并发写入。另在系统临时目录重建索引，共 8,824 份资产；库内一致性检查为 0 个问题。
- 按 ID 和目录核对全部 157 份候选资产。11 月 24 日的报告保存的是 manifest SHA-256，因此用它核对原始字节，并现场计算 BLAKE3 供移动、写回和撤回检查。负载则逐路径从磁盘重算 BLAKE3，同时核对数量、大小和组内不同哈希集合。新增入向引用为 0。
- 虚拟预检核对来源、目标、路径大小写、240 个 UTF-16 单元的长度限制、manifest schema、文件声明及引用。报告中 02、03、86 的目标先由待归档记录占用，依照报告顺序先归档再改目录。
- 写回沿用原 BOM 和换行，设置 updated 为 2026-11-26，在原 note 后添加归档 ID。78 份保留记录均保持 LF、无 BOM。78 组均无须改写父歌 manifest。

| 阶段 | 组数 | 移动 | manifest 编辑 | 创建目录 | 复核跳过 |
|---|---:|---:|---:|---:|---:|
| 全量 dry-run | 78 | 141 | 78 | 77 | 0 |
| 实际试跑 01、03、27 | 3 | 6 | 3 | 7 | 0 |
| 实际余下批次 | 75 | 135 | 75 | 70 | 0 |
| 实际合计 | 78 | 141 | 78 | 77 | 0 |

试跑覆盖目录不变的 A 组 Carousel、改键的 A 组 Dream of You，以及三份模型合一的 C 组 Tda Haku Long Pajama Shirt。完成后有 8,820 份记录、0 个问题。索引中三个保留 ID 的路径正确，四个退役 ID 已移出索引。四份归档共 93 个负载文件、298,647,053 字节，归档 manifest 原始字节与修改时间均吻合。撤回预检可逆转试跑的 16 步。

## 执行范围

执行组号为 01、02、03、05、06、08、09、10、11、12、13、14、15、16、17、19、20、21、22、23、24、26、27、28、29、30、31、32、33、34、35、36、37、38、39、41、42、43、45、46、47、48、49、50、51、52、53、54、55、56、58、59、60、61、62、64、65、66、67、68、70、72、73、75、76、77、78、81、82、83、84、85、86、87、88、91、92、94。

排除的人工确认组为 04、07、18、25、40、44、57、63、69、71、74、79、80、89、90、93。自动候选无失配，也无执行失败。

## 执行后验证

- check_library 扫描 8,745 份记录，9 项检查均为 0。分类型为模型 2,021、场景 1,355、歌曲 2,220、动作素材 3,149。
- 索引中 101,408 个负载路径均有 BLAKE3。整份重复由 94 组降至 16 组、共 32 份资产，与人工确认名单逐 ID 匹配。严格子集剩 294 对。默认筛选的文件级重复为 8,645 组。
- 全部 78 份保留 manifest 的字段和值均与报告补丁及执行附注匹配。核对了 61 份动作素材的父歌和索引入边。除 78 份编辑和 79 份归档外，8,667 份 manifest 的路径、内容 SHA-256 和修改时间与迁移前一致，包括所有父歌和人工确认组。
- 79 份原目录按 Library 内相对路径收在 G:\MMD\_Duplicates\dup-assets-20261126\。归档清单为 1,023 个负载文件、2,763,909,271 字节，与报告逐路径吻合，另有 79 份逐字节保留的 manifest。同卷重命名保留原资产字节，归档仍占用同卷磁盘空间。

| 抽查组 | 保留 ID | 最终目录 | 核对项 |
|---|---|---|---|
| 01 Dream of You | 01M81T92Z0QD62TPQSJB9B67VR | Library/Motions/Dream of You__Unknown/pack_NatsumiSan | key.author 为 NatsumiSan，父歌及 note 正确 |
| 03 Tda Haku Long Pajama Shirt | 01M822G23DW74815TBNF9WHX6A | Library/Models/Characters/InoriAruma On DeviantArt/Tda Haku Long Pajama Shirt | 无后缀目录，作者正确，note 含两个归档 ID |
| 27 Carousel | 01M81TA5XX0P8SBBXSGV03QXQ1 | Library/Motions/Carousel__Unknown/pack__Unknown_Carousel | 作者仍为 _Unknown，suffix、父歌及 note 正确 |
| 31 Sparkling | 01M81T92ZFZTPMDJRNKJAQPA78 | Library/Motions/Sparkling__Unknown/pack_NatsumiSan | 主动作作者 NatsumiSan，Cloud9 附件署名与配件引用保留 |
| 53 TDA Monochrome Doll Haku | 01M81T6S5WHQZGD8BX1WM0SM7G | Library/Models/Characters/HarukaSakurai/TDA Monochrome Doll Haku | 更正为最终改模者 HarukaSakurai，key 和 note 正确 |

## 执行工具与撤回

日志位于 G:\MMD\_design\migration_runs\dup-assets-20261126\log.jsonl。共 452 行，包含 78 条组开始、296 条操作及 78 条组提交记录。操作先落日志并同步，再重命名目录或原子替换 manifest。移动记录源、目标和 manifest BLAKE3；编辑记录完整的旧、新 UTF-8 字节及其 BLAKE3。撤回按日志倒序还原，并检查路径占用、目录是否为空及 manifest 摘要。

~~~powershell
cargo run -p kagura-core --release --example apply_dup_merge -- docs/reviews/dup-assets-20261124.json --dry-run
cargo run -p kagura-core --release --example apply_dup_merge -- --undo --dry-run
~~~

实际撤回时从第二条命令移除 --dry-run。全量只读预演已通过，296 步包含 141 次反向移动、78 次原字节写回和 77 次撤回时清理空目录。mini_library 上已实测完整撤回，包括 BOM、CRLF、父歌引用和原始字节恢复。

真实库测试以 11 月 24 日的 Python golden 为基础，只应用已执行 78 组的 ID、目录和统计差量。对原始快照可设置 KAGURA_LIBRARY_SNAPSHOT=20261124。

最终通过 cargo fmt --all --check、全 workspace Clippy -D warnings 及含真实库的 70 项测试。再次传入原报告执行 dry-run 时，78 组均已完成，待执行操作为 0。
