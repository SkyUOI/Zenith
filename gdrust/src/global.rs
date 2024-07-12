use crate::multi::MultiManager;
use crate::utils::screen_effects::ScreenEffects;
use godot::classes::{INode, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GlobalClass {
    #[var]
    connected_ip: GString,
    #[var]
    player_name: GString,
    #[var]
    multi_manager: Option<Gd<MultiManager>>,
    #[var]
    pub screen_effects: Gd<ScreenEffects>,
    base: Base<Node>,
}

#[godot_api()]
impl INode for GlobalClass {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            screen_effects: ScreenEffects::new_alloc(),
            connected_ip: GString::default(),
            multi_manager: None,
            player_name: GString::default(),
        }
    }

    fn ready(&mut self) {
        let tmp = self
            .get_screen_effects_scene()
            .instantiate_as::<ScreenEffects>();
        self.base_mut().add_child(tmp.clone().upcast());
        self.screen_effects = tmp;
    }
}

#[godot_api()]
impl GlobalClass {
    #[func]
    fn flush_multi(&mut self) {
        match &mut self.multi_manager {
            Some(manager) => {
                manager;
            }
            None => {
                godot_error!("MultiManager has not been initial")
            }
        }
    }

    fn get_screen_effects_scene(&mut self) -> Gd<PackedScene> {
        Gd::from_variant(
            &self
                .base_mut()
                .get_property()
                .get("screen_effects_scene".into()),
        )
    }
}
