use godot::engine::{INode2D, Node2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node2D)]
struct BlockDrawer {
    base: Base<Node2D>,
    #[var]
    // x方向上的block数量
    x: i32,
    #[var]
    // y方向上的block数量
    y: i32,
}

#[godot_api]
impl INode2D for BlockDrawer {
    fn init(base: Base<Node2D>) -> BlockDrawer {
        // godot_print!("BlockDrawer created from Godot Rust");
        Self {
            x: Self::X_BLOCKS,
            y: Self::Y_BLOCKS,
            base,
        }
    }

    fn process(&mut self, delta: f64) {}

    fn draw(&mut self) {}
}

#[godot_api]
impl BlockDrawer {
    #[constant]
    const X_BLOCKS: i32 = 10;
    #[constant]
    const Y_BLOCKS: i32 = 10;

    #[func]
    fn change_block_immediate(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    #[func]
    fn change_block_gently(&mut self, x: i32, y: i32) {}

    #[func]
    fn get_y_min(&mut self) -> i32 {
        // self.base().
        (self.base().get_viewport_rect().size.y - self.y as f32) as i32
    }

    #[func]
    fn get_x_min(&mut self) -> i32 {
        // self.base().
        (self.base().get_viewport_rect().size.x - self.x as f32) as i32
    }
}
