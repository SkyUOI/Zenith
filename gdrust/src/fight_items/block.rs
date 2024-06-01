use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node2D)]
struct BlockDrawer {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for BlockDrawer {
    fn init(base: Base<Node2D>) -> BlockDrawer {
        // godot_print!("BlockDrawer created from Godot Rust");
        Self { base }
    }
}