use godot::engine::{Control, IControl};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct Bar {
    base: Base<Control>,
}

#[godot_api()]
impl IControl for Bar {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }
}
