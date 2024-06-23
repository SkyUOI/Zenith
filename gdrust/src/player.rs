use core::panic;
use derive::gen_debug;
use godot::classes::{Area2D, CharacterBody2D, GpuParticles2D, ICharacterBody2D, Timer};
use godot::obj::WithBaseField;
use godot::prelude::*;
use real_consts::PI;
use std::time::{Duration, Instant};

use crate::debug_check;

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Player {
    #[var]
    health: i32,
    base: Base<CharacterBody2D>,
    status: Movement,
    /// 上一次点击克盾冲刺键的时间,记录来判断双击
    click_time: Option<Instant>,
    /// 上一次的类型
    click_type: Click,
    /// type to rush,单独设立防止被干扰
    rush_type: Click,
}

/// 描述按键点击状态
#[derive(PartialEq, Eq, Debug, Clone)]
enum Click {
    Right,
    Left,
    Up,
    Down,
    None,
}

/// 控制玩家状态
#[derive(PartialEq, Eq, Debug)]
enum Movement {
    /// 克盾冲刺
    Rush,
    // 普通移动
    Move,
}

const MOVE_RIGHT: &str = "move_right";
const MOVE_LEFT: &str = "move_left";
const MOVE_UP: &str = "move_up";
const MOVE_DOWN: &str = "move_down";
const SLOW_DOWN: &str = "slow_down";
const DOUBLE_CLICK_DURATION: Duration = Duration::from_millis(500);

#[godot_api()]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Player {
        // godot_print!("Player created from Godot Rust");
        Self {
            base,
            health: Self::MAX_HEALTH,
            status: Movement::Move,
            click_time: None,
            click_type: Click::None,
            rush_type: Click::None,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input_obj = Input::singleton();
        let mut down_rate = 1.0;
        if input_obj.is_action_pressed(SLOW_DOWN.into()) {
            down_rate = 0.5;
        }
        if self.status == Movement::Rush {
            // godot_print!("rush inside!");
            let vel = Vector2::new(
                match self.rush_type {
                    Click::Left => -1.0,
                    Click::Right => 1.0,
                    _ => {
                        let msg = format!("wrong movement {:?} when rush", self.click_type);
                        panic!("{}", &msg);
                    }
                },
                0.0,
            );
            self.base_mut().move_and_collide(
                vel.normalized() * Self::RUSH_SPEED as f32 * delta as f32 * down_rate,
            );
            return;
        }
        // 触发克盾
        self.process_rush(MOVE_LEFT.into(), Click::Left);
        self.process_rush(MOVE_RIGHT.into(), Click::Right);
        if input_obj.is_action_just_pressed(MOVE_UP.into()) {
            self.click_type = Click::Up;
            self.click_time = None;
        }
        if input_obj.is_action_just_pressed(MOVE_DOWN.into()) {
            self.click_type = Click::Down;
            self.click_time = None;
        }
        let vel = input_obj.get_vector(
            MOVE_LEFT.into(),
            MOVE_RIGHT.into(),
            MOVE_UP.into(),
            MOVE_DOWN.into(),
        );
        if vel != Vector2::ZERO {
            let res = self
                .base_mut()
                .move_and_collide(vel.normalized() * Self::SPEED as f32 * down_rate * delta as f32);
            if let Some(obj) = res {
                let me = self.base_mut().get_position();
                let point = obj.get_position();
                let diff_x = (me.x - point.x).abs().abs();
                let diff_y = (me.y - point.y).abs().abs();
                if (diff_x - diff_y).abs() >= 6.0 {
                    // 确认没有卡墙角
                    let vel = if diff_x > diff_y {
                        Vector2::new(
                            0.0,
                            if input_obj.is_action_pressed(MOVE_UP.into()) {
                                -1.0
                            } else if input_obj.is_action_pressed(MOVE_DOWN.into()) {
                                1.0
                            } else {
                                0.0
                            },
                        )
                    } else {
                        Vector2::new(
                            if input_obj.is_action_pressed(MOVE_LEFT.into()) {
                                -1.0
                            } else if input_obj.is_action_pressed(MOVE_RIGHT.into()) {
                                1.0
                            } else {
                                0.0
                            },
                            0.0,
                        )
                    };
                    self.base_mut()
                        .move_and_collide(vel * Self::SPEED as f32 * down_rate * delta as f32);
                }
            }
        }
    }

    fn ready(&mut self) {
        debug_check!(self)
    }
}

#[godot_api]
#[gen_debug]
impl Player {
    #[constant]
    const SPEED: i32 = 500;
    #[constant]
    const RUSH_SPEED: i32 = 700;
    #[constant]
    const MAX_HEALTH: i32 = 100;
    #[signal]
    fn hit_sig(attack: i32) {}

    #[func]
    pub fn hit(&mut self, attack: i32) {
        // godot_print!("attack!");
        self.base_mut()
            .emit_signal("hit_sig".into(), &[attack.to_variant()]);
    }

    fn start_rush(&mut self) {
        self.status = Movement::Rush;
        self.rush_type = self.click_type.clone();
        // 冲刺结束计时器
        let mut timer = self.get_cthulhu_timer();
        timer.start();
        // 启动拖尾粒子
        let mut particle = self.get_virtual_particle();
        particle.set_emitting(true);
        self.turn_on_shield();
    }

    #[debug]
    fn get_virtual_particle(&self) -> Gd<GpuParticles2D> {
        self.base().get_node_as("virtual")
    }

    #[func]
    fn stop_rush(&mut self) {
        let mut particle = self.get_virtual_particle();
        particle.set_emitting(false);
        self.status = Movement::Move;
        self.click_type = Click::None;
        self.turn_off_shield();
    }

    fn process_rush(&mut self, movement_name: StringName, click_type: Click) {
        let input_obj = Input::singleton();
        if input_obj.is_action_just_pressed(movement_name) {
            // 先检查上一次按下的键是不是双击键
            if self.click_type != click_type {
                // godot_print!("previous movement {:?}", self.click_type);
                self.click_type = click_type;
                // godot_print!("previous movement {:?}", self.click_type);
                self.click_time = Some(Instant::now());
                return;
            }
            // 判断第一次还是第二次
            // godot_print!("start to select");
            match self.click_time {
                None => {
                    // 第一次按下按钮，记录时间
                    self.click_time = Some(Instant::now())
                }
                Some(t) => {
                    // 第二次按下，比较两次中间的间隔
                    let dur = t.elapsed();
                    if dur <= DOUBLE_CLICK_DURATION {
                        // 达成双击条件
                        // 清除双击时间
                        self.click_time = None;
                        self.start_rush();
                    } else {
                        // 否则将当前时间设为双击开头
                        self.click_time = Some(Instant::now());
                    }
                }
            }
        }
    }

    #[debug]
    fn get_shield(&self) -> Gd<Area2D> {
        self.base().get_node_as("Shield")
    }

    /// 启动克盾保护
    #[func]
    fn turn_on_shield(&mut self) {
        let mut shield = self.get_shield();
        shield.set_rotation(self.cthulhu_rad());
        shield.show();
    }

    /// 关闭克盾保护
    #[func]
    fn turn_off_shield(&mut self) {
        let mut shield = self.get_shield();
        shield.hide();
    }

    #[debug]
    fn get_cthulhu_timer(&self) -> Gd<Timer> {
        self.base().get_node_as("Cthulhu")
    }

    fn cthulhu_rad(&self) -> f32 {
        match self.rush_type {
            Click::Left => PI,
            Click::Right => 0.0,
            _ => {
                let msg = format!("wrong movement {:?} when rush", self.click_type);
                panic!("{}", &msg);
            }
        }
    }
}
