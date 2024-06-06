use godot::engine::{INode2D, Node2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node2D)]
struct BlockDrawer {
    base: Base<Node2D>,
    #[var]
    x: f32,
    #[var]
    y: f32,
}

const BLOCK_COLOR: Color = Color::from_rgb(10.0, 10.0, 10.0);
const WIDTH: f32 = 12.0;

#[godot_api]
impl INode2D for BlockDrawer {
    fn init(base: Base<Node2D>) -> BlockDrawer {
        // godot_print!("BlockDrawer created from Godot Rust");
        Self {
            x: Self::BOX_START_POS_X,
            y: Self::BOX_START_POS_Y,
            base,
        }
    }

    fn process(&mut self, delta: f64) {}

    fn draw(&mut self) {
        let tmp = self.x;
        let xsize = self.base_mut().get_viewport_rect().size.x - tmp * 2.0;

        godot_print!("enter");
        let tmp = Vector2::new(self.x, self.y);
        self.base_mut()
            .draw_rect_ex(
                Rect2::new(tmp, Vector2::new(xsize, Self::Y_SIZE_DEFAULT)),
                BLOCK_COLOR,
            )
            .width(WIDTH)
            .filled(false)
            .done();
    }
}

#[godot_api]
impl BlockDrawer {
    const BOX_START_POS_X: f32 = 400.0;
    const BOX_START_POS_Y: f32 = 300.0;
    const Y_SIZE_DEFAULT: f32 = 200.0;

    #[func]
    fn change_block_immediate(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    #[func]
    fn change_block_gently(&mut self, x: i32, y: i32) {}

    #[func]
    fn get_y_min(&mut self) -> i32 {
        // self.base().
        (self.base().get_viewport_rect().size.y - self.y) as i32
    }

    #[func]
    fn get_x_min(&mut self) -> i32 {
        // self.base().
        (self.base().get_viewport_rect().size.x - self.x) as i32
    }
}
