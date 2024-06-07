use godot::engine::{Area2D, IArea2D};
use godot::obj::WithBaseField;
use godot::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::utils::split_to_vec;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct StarWrathBullet {
    base: Base<Area2D>,
    direct: Vector2,
    speed: i32,
}

const INIT_DIRECT: Vector2 = Vector2::new(0.0, 0.0);
const SPEED_MIN: i32 = 300;
const SPEED_MAX: i32 = 400;
const DIRECT_MIN: f32 = PI + PI / 3.0;
const DIRECT_MAX: f32 = PI + PI / 3.0 * 2.0;

#[godot_api()]
impl IArea2D for StarWrathBullet {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            direct: INIT_DIRECT,
            speed: 0,
        }
    }

    fn process(&mut self, delta: f64) {
        let tmp = self.base().get_position() + self.direct.normalized() * delta as f32;
        self.base_mut().set_position(tmp);
    }
}

#[godot_api()]
impl StarWrathBullet {
    /// 带方向的初始化
    pub fn init_with_direct(&mut self, direct: Vector2) {
        self.direct = direct;
        self.base_init((direct.y / direct.x).atan());
    }

    fn random_speed(&mut self) {
        self.speed = rand::thread_rng().gen_range(SPEED_MIN..=SPEED_MAX);
    }

    fn base_init(&mut self, rad: f32) {
        self.random_speed();
        self.base_mut().show();
        // 旋转到正确的位置
        self.base_mut().rotate(rad);
    }

    /// 从左上角初始化弹幕，方向随机
    pub fn init_from_corner(&mut self) {
        // 随机方向
        let direct_rad = thread_rng().gen_range(DIRECT_MIN..=DIRECT_MAX);
        self.direct = split_to_vec(direct_rad);
        self.base_init(direct_rad);
    }
}
