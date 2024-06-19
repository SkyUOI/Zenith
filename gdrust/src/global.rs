use crate::multi::MultiManager;
use godot::engine::{INode, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct GlobalClass {
    #[var]
    connected_ip: GString,
    #[var]
    player_name: GString,
    #[var]
    multi_manager: Option<Gd<MultiManager>>,
    base: Base<Node>,
}

#[godot_api()]
impl INode for GlobalClass {}

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
}
