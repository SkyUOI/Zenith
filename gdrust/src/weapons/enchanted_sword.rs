use crate::get_multi_single;
use godot::{
    classes::{ISprite2D, Sprite2D},
    prelude::*,
};
use proto::connect;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base = Sprite2D)]
struct EnchantedSword {
    properties: HashMap<String, Box<dyn Fn()>>,
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for EnchantedSword {
    fn init(base: Base<Sprite2D>) -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            "position".to_string(),
            Box::new(|| {
                // let data = connect::UpdateObj {};
                let data = todo!();
                get_multi_single().lock().unwrap().send_data(data)
            }),
        );
        Self {
            base,
            properties: HashMap::new(),
        }
    }

    fn get_property(&self, name: StringName) -> Option<Variant> {
        match self.properties.get(&name.to_string()) {
            None => {}
            Some(val) => val(),
        }
        None
    }
}

#[godot_api]
impl EnchantedSword {
    /// 设置要检测的属性
    fn set_monitor(&mut self, property_name: String, func: Box<dyn Fn()>) {
        self.properties.insert(property_name, func);
    }
}
