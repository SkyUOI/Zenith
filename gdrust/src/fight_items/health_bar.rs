use godot::engine::{INode2D, Node2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct HealthBar {
    base: Base<Node2D>,
    health: i32,
}

fn get_health_color() -> Color {
    Color::from_rgba8(255, 254, 71, 255)
}

#[godot_api()]
impl INode2D for HealthBar {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            health: Self::INIT_HEALTH,
        }
    }

    fn draw(&mut self) {
        let health = self.health;
        self.base_mut()
            .draw_line_ex(
                Vector2::new(Self::START_POSITION_X, Self::START_POSITION_Y),
                Vector2::new(
                    Self::LEN * (health as f32 / Self::INIT_HEALTH as f32) + Self::START_POSITION_X,
                    Self::START_POSITION_Y,
                ),
                get_health_color(),
            )
            .width(Self::WIDTH)
            .done();
    }
}

#[godot_api()]
impl HealthBar {
    const START_POSITION_X: f32 = 530.0;
    const START_POSITION_Y: f32 = 530.0;
    const WIDTH: f32 = 25.0;
    const LEN: f32 = 150.0;
    const INIT_HEALTH: i32 = 100;
}
