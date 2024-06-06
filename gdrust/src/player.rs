use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Area2D)]
struct Player {
    #[var]
    health: i32,
    base: Base<Area2D>,
}

#[godot_api()]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Player {
        // godot_print!("Player created from Godot Rust");
        Self {
            base,
            health: Self::MAX_HEALTH,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut vel = Vector2::ZERO;
        let input_obj = Input::singleton();
        if input_obj.is_action_pressed("move_left".into()) {
            vel.x -= 1.0;
        }
        if input_obj.is_action_pressed("move_right".into()) {
            vel.x += 1.0;
        }
        if input_obj.is_action_pressed("move_up".into()) {
            vel.y -= 1.0;
        }
        if input_obj.is_action_pressed("move_down".into()) {
            vel.y += 1.0;
        }
        // if vel != Vector2::ZERO {
        //     godot_error!("fuck you")
        // }
        let pos = self.base().get_position();
        self.base_mut()
            .set_position(pos + vel.normalized() * Self::SPEED as f32 * delta as f32);
        // godot_print!("position move")
    }
}

#[godot_api]
impl Player {
    #[constant]
    const SPEED: i32 = 500;
    #[constant]
    const MAX_HEALTH: i32 = 100;
}
