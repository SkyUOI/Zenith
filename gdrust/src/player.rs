use core::panic;
use godot::engine::{CharacterBody2D, GpuParticles2D, ICharacterBody2D, Timer};
use godot::obj::WithBaseField;
use godot::prelude::*;
use std::time::{Duration, Instant};

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Player {
    #[var]
    health: i32,
    base: Base<CharacterBody2D>,
    status: Movement,
    /// 上一次点击克盾冲刺键的时间,记录来判断双击
    click_time: Instant,
    /// 上一次的类型
    click_type: Click,
}

/// 描述按键点击状态
#[derive(PartialEq, Eq, Debug)]
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

#[godot_api()]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Player {
        // godot_print!("Player created from Godot Rust");
        Self {
            base,
            health: Self::MAX_HEALTH,
            status: Movement::Move,
            click_time: Instant::now(),
            click_type: Click::None,
        }
    }

    fn ready(&mut self) {}

    fn physics_process(&mut self, delta: f64) {
        let input_obj = Input::singleton();
        let mut speed = Self::SPEED;
        if input_obj.is_action_pressed(SLOW_DOWN.into()) {
            speed /= 2;
        }
        if self.status == Movement::Rush {
            godot_print!("rush inside!");
            let vel = Vector2::new(
                match self.click_type {
                    Click::Left => -Self::RUSH_SPEED as f32,
                    Click::Right => Self::RUSH_SPEED as f32,
                    _ => {
                        let msg = format!("wrong movement {:?} when rush", self.click_type);
                        godot_error!("{}", &msg);
                        panic!("{}", &msg);
                    }
                },
                0.0,
            );
            self.base_mut()
                .move_and_collide(vel * delta as f32 * speed as f32);
            return;
        }
        let mut vel = Vector2::ZERO;
        if input_obj.is_action_pressed(MOVE_LEFT.into()) {
            vel.x -= 1.0;
        }
        if input_obj.is_action_pressed(MOVE_RIGHT.into()) {
            vel.x += 1.0;
        }
        // 触发克盾
        if input_obj.is_action_just_pressed(MOVE_LEFT.into())
            && self.click_type == Click::Left
            && self.click_time.elapsed() < Duration::new(0, 100 * 1000)
        {
            self.start_rush();
        }
        if input_obj.is_action_just_pressed(MOVE_LEFT.into())
            && self.click_type == Click::Left
            && self.click_time.elapsed() < Duration::new(0, 100 * 1000)
        {
            self.start_rush();
        }
        // 记录克盾第一次按按钮
        if input_obj.is_action_just_released(MOVE_RIGHT.into()) {
            self.click_time = Instant::now();
            self.click_type = Click::Right;
        }
        if input_obj.is_action_just_released(MOVE_LEFT.into()) {
            self.click_time = Instant::now();
            self.click_type = Click::Left;
        }
        if input_obj.is_action_just_released(MOVE_DOWN.into()) {
            self.click_type = Click::Down;
        }
        if input_obj.is_action_just_released(MOVE_UP.into()) {
            self.click_type = Click::Up;
        }
        if input_obj.is_action_pressed(MOVE_UP.into()) {
            vel.y -= 1.0;
        }
        if input_obj.is_action_pressed(MOVE_DOWN.into()) {
            vel.y += 1.0;
        }
        let res = self
            .base_mut()
            .move_and_collide(vel.normalized() * speed as f32 * delta as f32);
    }
}

#[godot_api]
impl Player {
    #[constant]
    const SPEED: i32 = 500;
    #[constant]
    const RUSH_SPEED: i32 = 700;
    #[constant]
    const MAX_HEALTH: i32 = 100;
    #[signal]
    fn hit_sig(attack: i32);

    #[func]
    pub fn hit(&mut self, attack: i32) {
        // godot_print!("attack!");
        self.base_mut()
            .emit_signal("hit_sig".into(), &[attack.to_variant()]);
    }

    fn start_rush(&mut self) {
        godot_print!("rush inside!");
        self.status = Movement::Rush;
        // 冲刺结束计时器
        let mut timer = self.base().get_node_as::<Timer>("Cthulhu");
        timer.start();
        // 启动拖尾粒子
        let mut particle = self.get_virtual_particle();
        particle.set_emitting(true);
    }

    fn get_virtual_particle(&self) -> Gd<GpuParticles2D> {
        self.base().get_node_as::<GpuParticles2D>("virtual")
    }

    #[func]
    fn stop_rush(&mut self) {
        godot_print!("rush!");
        let mut particle = self.get_virtual_particle();
        particle.set_emitting(false);
        self.status = Movement::Move;
        self.click_type = Click::None;
    }
}
