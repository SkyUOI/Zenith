use godot::engine::{Control, IControl, Sprite2D, Timer};
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
}

#[godot_api]
impl IControl for Fight {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            shake_times: 0,
            origin_offset: Vector2::ZERO,
            shake_delta: 0.0,
        }
    }

    fn ready(&mut self) {
        let mut enchanted_sword = self.base().get_node_as::<Sprite2D>("Enchanted_Sword");
        enchanted_sword.call("fightStart".into(), &[]);
        self.shake(3.0, 1.0);
    }
}

#[godot_api()]
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

    fn get_camera(&self) -> Gd<Camera2D> {
        self.base().get_node_as::<Camera2D>("Camera2D")
    }

    fn get_shake_timer(&self) -> (Gd<Camera2D>, Gd<Timer>) {
        let camera = self.get_camera();
        let timer = camera.get_node_as::<Timer>("Shake");
        (camera, timer)
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
}
