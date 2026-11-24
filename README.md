# Kagura（神楽）

本地 MMD / 3D 资产的整理、浏览与简单预览工具。一个文件夹 = 一份资产，记录写在 `_asset.manifest.json`。

- 计划：[PLAN.md](PLAN.md)
- 核心库设计：[docs/core-design.md](docs/core-design.md)
- MCP 设计：[docs/mcp-design.md](docs/mcp-design.md)

## 结构

```
crates/kagura-core   核心库（命名、路径、manifest、schema 校验）
src-tauri            Tauri 2 桌面应用
ui                   React + TypeScript 资产浏览界面
schemas              JSON Schema 2020-12（编译进 kagura-core）
archive/VmdManager   旧工具的代码，仅留档
```

## 开发

需要 Rust 1.90+（锁定的 Tauri 2 要求 1.90，`rusqlite` 要求 1.89），以及 Tauri 2 的 Windows 依赖（WebView2、MSVC 生成工具）。

```powershell
cargo test -p kagura-core            # 核心库测试，含与 Python 版的 schema 对照
cargo install tauri-cli --version "^2" --locked
npm --prefix ui ci                   # 安装锁定的前端依赖
cargo tauri dev                      # 在 src-tauri 上层目录运行
```

首次启动、搜索筛选和资产详情见[界面说明](docs/ui.md)。

`schemas/` 从 `G:\MMD\_design\schemas` 复制，以后以仓库里的为准。
