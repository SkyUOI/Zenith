# Zenith

![总代码行数](https://tokei.rs/github/skyuoi/zenith)
|平台|状态|
|:--|:--|
|Linux|![linux test](https://img.shields.io/github/actions/workflow/status/skyuoi/zenith/rust_linux.yml)|
|Windows|![windows test](https://img.shields.io/github/actions/workflow/status/skyuoi/zenith/rust_windows.yml)|
|Macos|![macos test](https://img.shields.io/github/actions/workflow/status/skyuoi/zenith/rust_macos.yml)|

一个关于传说之下和泰拉瑞亚的同人游戏，使用Godot引擎开发.

## 构建指南

| 依赖              |
| :---------------- |
| Godot 4.3         |
| Gettext           |
| Protobuf Compiler |
| Rust Compiler     |

接下来，你可以运行`cd script && python build.py`来构建extension(Debug和Release)

最终，你可以启动godot编辑器来开发或导出游戏.

## 贡献

如果你想做出任何贡献，请看[CONTRIBUTING.md](./CONTRIBUTING_zh.md)
