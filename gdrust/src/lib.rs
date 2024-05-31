use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;

struct GdExtension;

#[gdextension()]
unsafe impl ExtensionLibrary for GdExtension {}

#[derive(GodotClass)]
#[class(base = Sprite2D)]
struct Player {
    base: Base<Sprite2D>,
}

#[godot_api()]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Player {
        godot_print!("Player created from Godot Rust");
        Self { base }
    }

    fn process(&mut self, delta: f64) {
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
    }
}

#[godot_api]
impl Player {
    #[constant]
    const SPEED:i32 = 50;
}
