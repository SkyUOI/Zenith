use godot::engine::{Button, IButton};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Button)]
struct MultiEnter {
    base: Base<Button>,
}

#[godot_api()]
impl IButton for MultiEnter {
    fn init(base: Base<Button>) -> Self {
        Self { base }
    }
}
