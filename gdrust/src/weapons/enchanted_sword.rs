use godot::{
    classes::{ISprite2D, Sprite2D},
    prelude::*,
};
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
struct EnchantedSword {
    properties: HashMap<String, Callable>,
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for EnchantedSword {
    fn get_property(&self, name: StringName) -> Option<Variant> {
        match self.properties.get(&name.to_string()) {
            None => {}
            Some(val) => {
                val.callv((&[self.base().to_variant()]).into());
            }
        }
        None
    }
}

#[godot_api]
impl EnchantedSword {
    #[func]
    /// 设置要检测的属性
    fn set_monitor(&mut self, property_name: String, func: Callable) {
        self.properties.insert(property_name, func);
    }
}
