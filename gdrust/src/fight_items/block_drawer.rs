use godot::classes::{Area2D, CollisionPolygon2D, INode2D, Node2D, StaticBody2D};
use godot::obj::{NewAlloc, WithBaseField};
use godot::prelude::*;

use crate::debug_check;

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

const COLLISION_WIDTH: f32 = 20.0;

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
        debug_check!(self);
        let points = [
            Vector2::new(self.x, self.y),
            Vector2::new(self.x + self.get_x_len(), self.y),
            Vector2::new(self.x + self.get_x_len(), self.y + Self::Y_SIZE_DEFAULT),
            Vector2::new(self.x, self.y + Self::Y_SIZE_DEFAULT),
        ];
        let offset = [
            COLLISION_WIDTH - 15.0,
            -COLLISION_WIDTH + 15.0,
            -COLLISION_WIDTH + 15.0,
            COLLISION_WIDTH - 15.0,
        ];
        let mut staticbody = self.get_staticbody();
        for i in 0..4 {
            let mut colliison_obj = CollisionPolygon2D::new_alloc();
            let mut line = PackedVector2Array::new();
            let start_point = points[i % 4];
            let end_point = points[(i + 1) % 4];
            let offset_vec = if start_point.x == end_point.x {
                // 竖线
                Vector2::new(offset[i], 0.0)
            } else {
                // 横线
                Vector2::new(0.0, offset[i])
            };
            line.push(start_point);
            let start_near = start_point + offset_vec;
            line.push(start_near);
            let end_near = end_point + offset_vec;
            line.push(end_near);
            line.push(end_point);
            colliison_obj.set_polygon(line);
            staticbody.add_child(colliison_obj.upcast());
        }
    }

    fn process(&mut self, delta: f64) {}

    fn draw(&mut self) {
        let xsize = self.get_x_len();

        // godot_print!("enter");
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

#[derive::gen_debug]
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

    #[func]
    fn collision(&mut self, obj: Gd<Area2D>) {}

    #[debug]
    fn get_staticbody(&self) -> Gd<StaticBody2D> {
        self.base().get_node_as("Collision")
    }
}
