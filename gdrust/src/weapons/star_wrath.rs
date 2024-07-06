use crate::bullets::star_wrath_bullet::StarWrathBullet;
use crate::debug_check;
use godot::classes::{Area2D, IArea2D, Timer};
use godot::engine::AnimationPlayer;
use godot::obj::WithBaseField;
use godot::prelude::*;
use rand::{thread_rng, Rng};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct StarWrath {
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

pub type Action = fn(&mut StarWrath);

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

    fn process(&mut self, delta: f64) {}

    fn ready(&mut self) {
        debug_check!(self);
        self.base_mut().set_process(false);
    }
}

#[godot_api]
#[derive::gen_debug]
impl StarWrath {
    #[func]
    fn new_bullet(&mut self) {
        // bullet scene
        let scene = self.get_bullet_scene();
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

    #[func]
    #[debug]
    fn get_bullet_scene(&self) -> Gd<PackedScene> {
        Gd::from_variant(&self.base().get_property().get("star_wrath_origin".into()))
    }

    #[func]
    fn get_animation(&self) -> Gd<AnimationPlayer> {
        self.base().get_node_as("AnimationPlayer")
    }

    #[debug]
    fn get_fight_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("FightTimer")
    }

    #[func]
    fn start(&mut self) {
        // let mut fight_time = self.get_fight_timer();
        // fight_time.start();
        self.base_mut().set_process(true);
        self.start_flag = true;
        self.new_bullet();
        let mut anmi = self.get_animation();
        anmi.play_ex().name("enter_scene".into()).done();
    }

    #[func]
    /// 新建一个从天而降垂直下落的弹幕
    fn fall_star(&mut self) {
        let bullet = self.get_bullet_scene();
        let mut star = bullet.instantiate_as::<StarWrathBullet>();
        let sz = self.base_mut().get_viewport_rect().size.x - 100.0;
        let random_x = thread_rng().gen_range(100.0..sz);
        godot_print!("{}", random_x);
        self.base_mut().add_child(star.clone().upcast());
        star.bind_mut().init_from_sky(random_x);
    }
}
