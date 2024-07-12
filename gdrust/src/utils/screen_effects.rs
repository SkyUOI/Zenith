use derive::gen_debug;
use godot::{
    builtin::Vector2,
    classes::{Camera2D, ICamera2D},
    engine::Timer,
    log::godot_print,
    obj::{Base, Gd, WithBaseField},
    register::{godot_api, GodotClass},
};
use rand::Rng;

use crate::debug_check;

#[derive(GodotClass)]
#[class(base = Camera2D)]
pub struct ScreenEffects {
    shake_times: i32,
    shake_delta: f32,
    origin_offset: Vector2,
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for ScreenEffects {
    fn init(base: Base<Camera2D>) -> Self {
        Self {
            shake_times: 0,
            shake_delta: 0.0,
            origin_offset: Vector2::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        debug_check!(self)
    }
}

#[godot_api]
#[gen_debug]
impl ScreenEffects {
    /// 晃动屏幕
    /// duration: 持续时间
    #[func]
    pub fn shake(&mut self, duration: f64, delta: f32) {
        // 根据每次等待时间和总时间算出需要震动多少次
        let mut timer = self.get_shake_timer();
        let wait_time = timer.get_wait_time();
        let amount = (duration / wait_time) as i32;
        self.shake_delta = f32::max(self.shake_delta, delta);
        if self.shake_times != 0 {
            self.shake_times += amount;
            return;
        }
        self.origin_offset = self.base().get_offset();
        self.shake_times = amount;
        godot_print!("timer start");
        timer.start();
    }

    #[debug]
    fn get_shake_timer(&self) -> Gd<Timer> {
        self.base().get_node_as("Shake")
    }

    #[func]
    fn shake_timeout(&mut self) {
        let mut timer = self.get_shake_timer();
        self.shake_times -= 1;
        if self.shake_times == 0 {
            // 设回原来的位置
            let tmp = self.origin_offset;
            self.base_mut().set_offset(tmp);
            timer.stop();
            self.shake_delta = 0.0;
            return;
        }
        let rng = -self.shake_delta..=self.shake_delta;
        let tmp = self.origin_offset
            + Vector2::new(
                rand::thread_rng().gen_range(rng.clone()),
                rand::thread_rng().gen_range(rng),
            );
        self.base_mut().set_offset(tmp);
    }
}
