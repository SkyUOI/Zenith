use godot::engine::{Control, IControl};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Control)]
struct Fight {
    base: Base<Control>,
}

#[godot_api]
impl IControl for Fight {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }
}
