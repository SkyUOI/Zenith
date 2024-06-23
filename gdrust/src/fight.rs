use std::sync::OnceLock;

use crate::debug_check;
use godot::classes::{Control, IControl, Timer};
use godot::obj::WithBaseField;
use godot::prelude::*;
use rand::Rng;

#[derive(GodotClass)]
#[class(base = Control)]
struct Fight {
    base: Base<Control>,
    shake_times: i32,
    origin_offset: Vector2,
    shake_delta: f32,
    sword_idx: usize,
}

const START: &str = "start";
const ATTACK_FINISHED: &str = "attack_finished";

fn get_fight_list() -> &'static Vec<&'static str> {
    static TMP: OnceLock<Vec<&'static str>> = OnceLock::new();
    TMP.get_or_init(|| vec!["EnchantedSword", "StarWrath"])
}

#[godot_api]
impl IControl for Fight {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            shake_times: 0,
            origin_offset: Vector2::ZERO,
            shake_delta: 0.0,
            sword_idx: 0,
        }
    }

    fn ready(&mut self) {
        debug_check!(self);
        self.shake(3.0, 1.0);
        self.start_fight();
    }
}

#[godot_api()]
#[derive::gen_debug]
impl Fight {
    /// 晃动屏幕
    /// duration: 持续时间
    #[func]
    fn shake(&mut self, duration: f64, delta: f32) {
        // 根据每次等待时间和总时间算出需要震动多少次
        let (camera, mut timer) = self.get_shake_timer();
        let wait_time = timer.get_wait_time();
        let amount = (duration / wait_time) as i32;
        self.shake_delta = f32::max(self.shake_delta, delta);
        if self.shake_times != 0 {
            self.shake_times += amount;
            return;
        }
        self.origin_offset = camera.get_offset();
        self.shake_times = amount;
        timer.start();
    }

    #[debug]
    fn get_camera(&self) -> Gd<Camera2D> {
        self.base().get_node_as("Camera2D")
    }

    #[debug]
    fn get_shake_timer(&self) -> (Gd<Camera2D>, Gd<Timer>) {
        let camera = self.get_camera();
        let timer = camera.get_node_as("Shake");
        (camera, timer)
    }

    fn get_sword(&self, name: &str) -> Gd<Node> {
        self.base().get_node_as(name)
    }

    #[debug]
    fn check_sword(&self) {
        for i in get_fight_list() {
            let obj = self.base().get_node_as::<Node>(*i);
            assert!(obj.has_method(START.into()), "**{}**", *i);
            assert!(obj.has_signal(ATTACK_FINISHED.into()), "**{}**", *i);
        }
    }

    #[func]
    fn shake_timeout(&mut self) {
        let (mut camera, mut timer) = self.get_shake_timer();
        self.shake_times -= 1;
        if self.shake_times == 0 {
            // 设回原来的位置
            camera.set_offset(self.origin_offset);
            timer.stop();
            self.shake_delta = 0.0;
            return;
        }
        camera.set_offset(
            self.origin_offset
                + Vector2::new(
                    rand::thread_rng().gen_range(-self.shake_delta..=self.shake_delta),
                    rand::thread_rng().gen_range(-self.shake_delta..=self.shake_delta),
                ),
        );
    }

    #[debug]
    fn get_end_fight(&self) -> Callable {
        self.base().callable("end_fight")
    }

    #[func]
    fn start_fight(&mut self) {
        let list = get_fight_list();
        if self.sword_idx >= list.len() {
            return;
        }
        let mut sword = self.get_sword(list[self.sword_idx]);
        sword.connect("attack_finished".into(), self.get_end_fight());
        sword.call(START.into(), &[]);
        godot_print!("start fight:{}", self.sword_idx);
        self.sword_idx += 1;
    }
}
