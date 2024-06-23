use crate::bullets::star_wrath_bullet::StarWrathBullet;
use crate::debug_check;
use godot::classes::{AnimationTree, Area2D, IArea2D, Timer};
use godot::obj::WithBaseField;
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
            state: State::Wave1,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut animation = self.get_animation();
        // animation.play_ex().name("Wave".into()).done();
        // if !animation.is_playing() {
        //     animation.play_ex().name("Float".into()).done();
        // }
        if !self.start_flag {
            return;
        }
        if self.state == State::None {
            return;
        }
        if self.state == State::Wave1 {}
    }

    fn ready(&mut self) {
        // for debug
        // 检查是否是当前场景
        debug_check!(self);
        if self.base().get_tree().unwrap().get_current_scene().unwrap()
            == self.base().clone().upcast()
        {
            self.start();
        } else {
            self.base_mut().hide();
        }
    }
}

#[godot_api]
#[derive::gen_debug]
impl StarWrath {
    #[func]
    fn new_bullet(&mut self) {
        // bullet scene
        let scene = load::<PackedScene>("res://scenes/bullets/star_wrath_original_bullet.tscn");
        for _ in 0..BULLETS_NUM {
            let mut star = scene.instantiate_as::<StarWrathBullet>();
            self.base_mut()
                .get_parent()
                .unwrap()
                .call_deferred("add_child".into(), &[star.to_variant()]);
            let sz = self.base_mut().get_viewport_rect().size.x;
            star.bind_mut().init_from_corner(Vector2::new(sz, 0.0));
        }
    }

    #[func]
    fn on_fight_timer_timeout(&mut self) {
        self.new_bullet()
    }

    #[debug]
    fn get_fight_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("FightTimer")
    }

    #[debug]
    fn get_animation(&self) -> Gd<AnimationTree> {
        self.base().get_node_as::<AnimationTree>("AnimationTree")
    }

    #[func]
    fn start(&mut self) {
        let mut fight_time = self.get_fight_timer();
        fight_time.start();
        self.start_flag = true;
        self.new_bullet()
    }

    #[signal]
    fn attack_finished() {}
}
