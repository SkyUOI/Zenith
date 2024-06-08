use godot::engine::{Control, IControl, Sprite2D};
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
        let mut EnchantedSword = self.base().get_node_as::<Sprite2D>("Enchanted_Sword");
        EnchantedSword.call("fightStart".into(), &[]);
    }
}
