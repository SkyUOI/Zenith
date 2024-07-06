use derive::gen_debug;
use godot::{
    engine::{INode, Node},
    obj::{Base, Gd, WithBaseField},
    register::{godot_api, GodotClass},
};
use std::sync::OnceLock;

use crate::debug_check;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct SwordManager {
    sword_idx: usize,
    base: Base<Node>,
}

pub const START: &str = "start";
pub const ATTACK_FINISHED: &str = "attack_finished";

fn get_fight_list() -> &'static Vec<&'static str> {
    static TMP: OnceLock<Vec<&'static str>> = OnceLock::new();
    TMP.get_or_init(|| vec!["EnchantedSword", "StarWrath"])
}

#[godot_api]
impl INode for SwordManager {
    fn init(base: Base<Node>) -> Self {
        Self { base, sword_idx: 0 }
    }

    fn ready(&mut self) {
        debug_check!(self);
    }
}

#[godot_api]
#[gen_debug]
impl SwordManager {
    #[debug]
    fn check_sword(&self) {
        for i in get_fight_list() {
            let obj = self.base().get_node_as::<Node>(*i);
            assert!(obj.has_method(START.into()), "**{}** misses start", *i);
            assert!(
                obj.has_signal(ATTACK_FINISHED.into()),
                "**{}** misses attack finished",
                *i
            );
        }
    }

    #[func]
    fn get_sword(&mut self) -> Gd<Node> {
        self.base().get_node_as(get_fight_list()[self.sword_idx])
    }

    #[func]
    fn next_sword(&mut self) {
        if self.sword_idx >= get_fight_list().len() {
            return;
        }
        self.sword_idx += 1;
    }

    #[func]
    pub fn get_and_next_sword(&mut self) -> Gd<Node> {
        let obj = self.get_sword();
        self.next_sword();
        obj
    }
}
