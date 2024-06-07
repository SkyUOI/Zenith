use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
struct Player {
    #[var]
    health: i32,
    base: Base<CharacterBody2D>,
}

#[godot_api()]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Player {
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
        let mut speed = Self::SPEED;
        if input_obj.is_action_pressed("slow_down".into()) {
            speed /= 2;
        }
        let res = self
            .base_mut()
            .move_and_collide(vel.normalized() * speed as f32 * delta as f32);
    }
}

#[godot_api]
impl Player {
    #[constant]
    const SPEED: i32 = 500;
    #[constant]
    const MAX_HEALTH: i32 = 100;
}
