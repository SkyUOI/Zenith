use godot::engine::{Area2D, CollisionPolygon2D, INode2D, Node2D};
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

const WIDTH: f32 = 10.0;

fn get_block_color() -> Color {
    Color::from_rgba8(255, 255, 255, 255)
}

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

    fn ready(&mut self) {
        let mut collision_obj = self
            .base_mut()
            .get_node_as::<CollisionPolygon2D>("collision/collision");
        let mut points = PackedVector2Array::new();
        points.push(Vector2::new(self.x, self.y));
        points.push(Vector2::new(self.get_x_len(), self.y));
        points.push(Vector2::new(
            self.get_x_len(),
            self.y + Self::Y_SIZE_DEFAULT,
        ));
        points.push(Vector2::new(self.x, self.y + Self::Y_SIZE_DEFAULT));
        collision_obj.set_polygon(points);
        collision_obj.set_disabled(false);
        godot_print!("nihao")
    }

    fn process(&mut self, delta: f64) {}

    fn draw(&mut self) {
        let xsize = self.get_x_len();

        godot_print!("enter");
        let tmp = Vector2::new(self.x, self.y);
        self.base_mut()
            .draw_rect_ex(
                Rect2::new(tmp, Vector2::new(xsize, Self::Y_SIZE_DEFAULT)),
                get_block_color(),
            )
            .width(WIDTH)
            .filled(false)
            .done();
    }
}

impl BlockDrawer {
    fn get_x_len(&self) -> f32 {
        let tmp = self.x;
        let xsize = self.base().get_viewport_rect().size.x - tmp * 2.0;
        xsize
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
