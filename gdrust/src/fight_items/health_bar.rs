use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct HealthBar {
    base: Base<Node2D>,
}

#[godot_api()]
impl INode2D for HealthBar {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }

    fn draw(&mut self) {}
}
