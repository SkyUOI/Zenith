use crate::fight_items::sword::SwordManager;
use crate::{debug_check, get_global, get_global_screen_effects};
use godot::classes::{Control, IControl};
use godot::obj::WithBaseField;
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

    fn ready(&mut self) {
        debug_check!(self);
        get_global_screen_effects!(self).bind_mut().shake(3.0, 1.0);
        self.base_mut().call("start_fight".into(), &[]);
    }
}

#[godot_api()]
#[derive::gen_debug]
impl Fight {
    #[func]
    #[debug]
    fn get_sword_manager(&self) -> Gd<SwordManager> {
        self.base().get_node_as("SwordManager")
    }
}
