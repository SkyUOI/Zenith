# 构建指南

| 依赖              |
| :---------------- |
| Godot 4.3         |
| Gettext           |
| Protobuf Compiler |
| Rust Compiler     |

关于Protobuf Compiler，我们首先会在本地查找是否存在protoc，如果存在可用的编译器，我们会直接采用，对于某些特殊需求，例如protoc版本过低等，我们也提供了自动编译protoc的选项，该选项默认关闭，可以通过`cargo build --package proto --features protobuf_feature`来启用捆绑的protoc，
注意，该选项可能使你的构建目录增大，构建时间增长若干倍，仍然建议提前安装好protoc

接下来，你可以运行`cd script && python build.py`来构建extension(Debug和Release)

最终，你可以启动godot编辑器来开发或导出游戏.
