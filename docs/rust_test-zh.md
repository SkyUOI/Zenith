# Rust测试指引

## 1.正确的依赖

 对于Rust代码，首先对于严重依赖Godot场景树和对象函数的方法，由于使用字符串调用，你需要对于Debug模式下，在`ready`函数中尝试`get`这些函数和对象，以此实现部分的自动测试

然而，由于该部分较为重复，我们提供了```#[derive::gen_debug]```和```debug_check!()```宏来简化。

例子:

```Rust
#[godot_api]
impl INode2D for xxx {
    fn ready(&mut self) {
        debug_check!(self);
    }
}

#[derive::gen_debug]
#[godot_api]
impl xxx {
    #[debug]
    fn get_staticbody(&self) -> Gd<StaticBody2D> {
        self.base().get_node_as::<StaticBody2D>("Collision")
    }
}
```

这样，被```#[debug]```标记的函数就会在Debug模式下于```debug_check!()```宏处运行了
