use derive::gen_debug;
use godot::engine::{AcceptDialog, Button, IButton};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::debug_check;

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

    fn ready(&mut self) {
        debug_check!(self)
    }
}

#[godot_api]
#[gen_debug]
impl MultiEnter {
    #[func]
    fn connect_to_server() {}

    #[debug]
    fn get_dialog(&self) -> Gd<AcceptDialog> {
        self.base().get_node_as("WrongDialog")
    }
}
