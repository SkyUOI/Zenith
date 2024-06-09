# Code Style

## 1.工具辅助

- 你需要使用[gdtoolkit](https://github.com/Scony/godot-gdscript-toolkit)和rustfmt进行格式化，你可以通过将`script/pre-commit`复制到`.git/hooks`来自动执行这个过程

## 2.命名规范

- 对于所有节点的名称，我们规定采用**大驼峰**格式

## 3.正确性保证

- 对于Rust代码，首先对于严重依赖Godot场景树和对象函数的方法，由于使用字符串调用，你需要对于Debug模式下，在`init`函数中尝试`get`这些函数和对象，以此实现部分的自动测试
