use crate::fight_items::sword::{SwordManager, START};
use crate::{debug_check, get_global};
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
        get_global!(self, screen_effects).bind_mut().shake(3.0, 1.0);
        self.start_fight();
    }
}

#[godot_api()]
#[derive::gen_debug]
impl Fight {
    #[debug]
    fn get_end_fight(&self) -> Callable {
        self.base().callable("end_fight")
    }

    #[debug]
    fn get_sword_manager(&self) -> Gd<SwordManager> {
        self.base().get_node_as("SwordManager")
    }

    #[func]
    fn start_fight(&mut self) {
        let mut sword = self.get_sword_manager().bind_mut().get_and_next_sword();
        sword.connect("attack_finished".into(), self.get_end_fight());
        sword.call(START.into(), &[]);
    }
}
