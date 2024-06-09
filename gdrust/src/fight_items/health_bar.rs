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

fn get_health_background_color() -> Color {
    Color::from_rgba8(154, 53, 29, 255)
}

#[godot_api()]
impl INode2D for HealthBar {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            health: Self::INIT_HEALTH,
        }
    }

    fn ready(&mut self) {}

    fn draw(&mut self) {
        // godot_print!("again！");
        let start_pos = Vector2::new(Self::START_POSITION_X, Self::START_POSITION_Y);
        let end_pos = Vector2::new(
            Self::LEN * (self.health as f32 / Self::INIT_HEALTH as f32) + Self::START_POSITION_X,
            Self::START_POSITION_Y,
        );
        let end_pos_back = Vector2::new(Self::LEN + Self::START_POSITION_X, Self::START_POSITION_Y);
        // background
        self.base_mut()
            .draw_line_ex(start_pos, end_pos_back, get_health_background_color())
            .width(Self::WIDTH)
            .done();
        // health
        self.base_mut()
            .draw_line_ex(start_pos, end_pos, get_health_color())
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

    #[func]
    fn attack(&mut self, harm: i32) {
        // godot_print!("received!{}", self.health);
        self.base_mut().queue_redraw();
        if harm > self.health {
            self.health = 0;
        } else {
            self.health -= harm;
        }
    }
}
