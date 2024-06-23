# Rust Test Guide

## 1. Correctness Assurance

For Rust code, first of all, for methods that heavily rely on Godot's scene tree and object functions, since string calling is used, you need to attempt to `get` these functions and objects in the `ready` function during Debug mode, thereby implementing some automatic testing.

However,because this part is repetitive,we provide `#[derive::gen_debug]` and `debug_check!()` macros to simplify this process.

For example:

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
        self.base().get_node_as("Collision")
    }
}
```

Like this,the function marked by `#[debug]` will be running in debug mode at the place of `debug_check!()` macro.
