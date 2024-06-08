use crate::bullets::star_wrath_bullet::StarWrathBullet;
use crate::player::*;
use godot::engine::{Area2D, IArea2D, Timer};
use godot::obj::{NewAlloc, WithBaseField};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct StarWrath {
    base: Base<Area2D>,
    start_flag: bool,
    state: State,
}

#[derive(PartialEq, Eq)]
enum State {
    Wave1,
    Wave2,
    WaveReSet,
    None,
}

const BULLETS_NUM: i32 = 3;

#[godot_api()]
impl IArea2D for StarWrath {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            start_flag: false,
            state: State::None,
        }
    }

    fn ready(&mut self) {
        // for debug
        // 检查是否是当前场景
        if self.base().get_tree().unwrap().get_current_scene().unwrap()
            == self.base().clone().upcast()
        {
            let mut fight_time = self.base().get_node_as::<Timer>("Fight_Timer");
            fight_time.start();
            self.start();
        }
    }

    fn process(&mut self, delta: f64) {
        if !self.start_flag {
            return;
        }
        if self.state == State::None {
            return;
        }
        if self.state == State::Wave1 {
            self.base_mut().rotate(0.1 * delta as f32)
        }
    }
}

#[godot_api()]
impl StarWrath {
    fn init() {}

    #[func]
    fn new_bullet(&mut self) {
        let player = Player::new_alloc();
        for i in 0..BULLETS_NUM {
            let mut star = StarWrathBullet::new_alloc();
            let sz = self.base_mut().get_viewport_rect().size.x;
            star.bind_mut().init_from_corner();
            self.base_mut()
                .get_parent()
                .unwrap()
                .call_deferred("add_child".into(), &[star.to_variant()]);
        }
    }

    #[func]
    fn on_killer_screen_exited(&mut self) {
        self.base_mut().queue_free()
    }

    #[func]
    fn on_fight_timer_timeout(&mut self) {
        self.new_bullet()
    }

    #[func]
    fn start(&mut self) {
        self.new_bullet()
    }
}
