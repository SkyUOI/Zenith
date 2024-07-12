use derive::gen_debug;
use godot::classes::{Area2D, IArea2D};
use godot::engine::CollisionShape2D;
use godot::obj::WithBaseField;
use godot::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::player::Player;
use crate::utils::screen_effects::ScreenEffects;
use crate::utils::split_to_vec;
use crate::{debug_check, get_global, godot_debug_assert};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct StarWrathBullet {
    base: Base<Area2D>,
    direct: Vector2,
    speed: i32,
}

const SPEED_MIN: i32 = 300;
const SPEED_MAX: i32 = 400;
const DIRECT_MIN: f32 = PI + PI / 6.0;
const DIRECT_MAX: f32 = PI + PI / 3.0;

#[godot_api()]
impl IArea2D for StarWrathBullet {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            direct: Vector2::ZERO,
            speed: 0,
        }
    }

    fn process(&mut self, delta: f64) {
        let tmp =
            self.base().get_global_position() + self.direct * delta as f32 * self.speed as f32;
        self.base_mut().set_global_position(tmp);
    }

    fn ready(&mut self) {
        debug_check!(self)
    }
}

#[godot_api]
#[gen_debug]
impl StarWrathBullet {
    /// 带方向的初始化
    #[func]
    pub fn init_with_direct(&mut self, direct: Vector2) {
        self.direct = direct;
        self.base_init((direct.y / direct.x).atan());
    }

    /// 设置随机运行速度
    fn random_speed(&mut self) {
        self.speed = thread_rng().gen_range(SPEED_MIN..=SPEED_MAX);
    }

    fn base_init(&mut self, rad: f32) {
        self.random_speed();
        self.base_mut().show();
        // 旋转到正确的位置
        self.base_mut().rotate(PI - rad);
    }

    /// 从左上角初始化弹幕，方向随机
    #[func]
    pub fn init_from_corner(&mut self, pos: Vector2) {
        // 随机方向
        let direct_rad = thread_rng().gen_range(DIRECT_MIN..=DIRECT_MAX);
        self.direct = split_to_vec(direct_rad);
        // godot坐标系与正常坐标系不同，手动对Y取反
        self.direct.y = -self.direct.y;
        godot_debug_assert!(self.direct.y >= 0.0 && self.direct.x <= 0.0);
        self.base_init(direct_rad);
        self.base_mut().set_global_position(pos);
    }

    /// 从天上致命坠落
    /// x_idx:全局Y坐标
    #[func]
    pub fn init_from_sky(&mut self, x_idx: f32) {
        let pos = Vector2::new(x_idx, 0.0);
        self.base_mut().set_global_position(pos);
        let mask = self.get_track_scene();
        let mut new_mask = mask.instantiate().unwrap();
        self.base_mut().add_child(new_mask.clone());
        new_mask.call("init".into(), &[]);
        self.base_mut().show();
        self.base_mut().set_global_rotation(-PI / 2.0);
        self.random_speed();
        self.direct = Vector2::DOWN;
    }

    #[func]
    #[debug]
    fn get_track_scene(&self) -> Gd<PackedScene> {
        Gd::from_variant(&self.base().get_property().get("track".into()))
    }

    #[func]
    fn area_entered(&mut self, obj: Gd<Area2D>) {
        if obj.get_name() == "Player".into() {
            let mut obj = obj.get_parent().unwrap().cast::<Player>();
            obj.bind_mut().hit(1);
            // 关闭碰撞检测
            self.get_collision()
                .set_deferred("disabled".into(), true.to_variant());
        }
    }

    #[debug]
    fn get_collision(&mut self) -> Gd<CollisionShape2D> {
        self.base_mut().get_node_as("CollisionShape2D")
    }
}
