use derive::gen_debug;
use godot::classes::{Area2D, IArea2D};
use godot::obj::WithBaseField;
use godot::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::{debug_check, godot_debug_assert};
use crate::player::Player;
use crate::utils::split_to_vec;

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
        let tmp = self.base().get_position() + self.direct * delta as f32 * self.speed as f32;
        self.base_mut().set_position(tmp);
    }

    fn draw(&mut self) {}

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
        // let mut new_mask = mask.instantiate().unwrap();
        // self.base_mut().add_child(new_mask.clone());
        godot_print!("now pos:{}", pos);
        // new_mask.call("init".into(), &[pos.to_variant()]);
        self.base_mut().show();
        self.base_mut().set_global_rotation(-PI / 2.0);
    }

    fn track(&mut self) {
        self.base_mut().queue_redraw();
    }

    #[func]
    #[debug]
    fn get_track_scene(&self) -> Gd<PackedScene> {
        load::<PackedScene>("res://scenes/bullets/star_wrath/track.tscn")
    }

    #[func]
    fn on_killer_screen_exited(&mut self) {
        // godot_print!("free bullet");
        self.base_mut().queue_free()
    }

    #[func]
    fn area_entered(&mut self, obj: Gd<Area2D>) {
        if obj.get_name() == "Player".into() {
            let mut obj = obj.get_parent().unwrap().cast::<Player>();
            obj.bind_mut().hit(1);
        }
    }
}
