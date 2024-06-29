# Build Guide

| dependencies      |
| :---------------- |
| Godot 4.3         |
| Gettext           |
| Protobuf Compiler |
| Rust Compiler     |

Regarding the Protobuf Compiler, we first check for the existence of a local protoc executable. If a usable compiler is found, it will be utilized directly. For certain special requirements, such as when the protoc version is too low, we also provide an option to
automatically compile a bundled protoc. This option is turned off by default and can be enabled by using cargo build --package proto --features protobuf_feature. Note that this option may significantly increase your build directory size and extend the build time by
several times. It is still recommended to install protoc in advance.

Then you can run `cd script && python build.py` to build the extension (both Debug and Release)

Finally,you can start godot editor and develop or export the game.
