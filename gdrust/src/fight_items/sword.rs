use derive::gen_debug;
use godot::{
    engine::{INode, Node},
    log::godot_print,
    obj::{Base, Gd, WithBaseField},
    register::{godot_api, GodotClass},
};
use std::{collections::HashSet, fs, sync::OnceLock};

use crate::{cfg::SWORD_DEBUG, debug_check};

#[derive(GodotClass)]
#[class(base = Node)]
pub struct SwordManager {
    sword_idx: usize,
    base: Base<Node>,
}

pub const START: &str = "start";
pub const ATTACK_FINISHED: &str = "attack_finished";

fn get_sword_basic() -> &'static Vec<&'static str> {
    static TMP: OnceLock<Vec<&str>> = OnceLock::new();
    TMP.get_or_init(|| vec!["EnchantedSword", "StarWrath"])
}

fn get_sword_map() -> &'static HashSet<&'static str> {
    static TMP: OnceLock<HashSet<&'static str>> = OnceLock::new();
    TMP.get_or_init(|| get_sword_basic().clone().into_iter().collect())
}

fn get_fight_list() -> &'static Vec<&'static str> {
    static TMP: OnceLock<Vec<&'static str>> = OnceLock::new();
    TMP.get_or_init(|| {
        #[cfg(debug_assertions)]
        {
            // 检查是否存在提前运行的文件
            match fs::read_to_string(SWORD_DEBUG) {
                Err(_) => {}
                Ok(sword) => {
                    // 检查剑是否存在
                    let sword = sword.trim();
                    if get_sword_map().contains(sword) {
                        return vec![get_sword_map().get(sword).unwrap()];
                    } else {
                        panic!("Found {},but sword {} not found", SWORD_DEBUG, sword)
                    }
                }
            }
        }
        get_sword_basic().clone()
    })
}

#[godot_api]
impl INode for SwordManager {
    fn init(base: Base<Node>) -> Self {
        Self { base, sword_idx: 0 }
    }

    fn ready(&mut self) {
        debug_check!(self);
        godot_print!("{:?}", get_fight_list())
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
