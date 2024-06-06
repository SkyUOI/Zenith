use godot::engine::{Area2D, IArea2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct StarWrath {
    base: Base<Area2D>,
}

#[godot_api()]
impl IArea2D for StarWrath {
    fn init(base: Base<Area2D>) -> Self {
        Self { base }
    }

    fn process(&mut self, delta: f64) {}
}

#[godot_api()]
impl StarWrath {
    fn init() {}
}
