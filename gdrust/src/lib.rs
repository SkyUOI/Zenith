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
        // godot_print!("Player created");
        Self { base }
    }
}
