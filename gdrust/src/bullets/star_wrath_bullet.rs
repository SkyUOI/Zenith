use godot::engine::{Area2D, IArea2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct StarWrathBullet {
    base: Base<Area2D>,
    direct: Vector2,
}

const INIT_DIRECT: Vector2 = Vector2::new(0.0, 0.0);

#[godot_api()]
impl IArea2D for StarWrathBullet {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            direct: INIT_DIRECT,
        }
    }

    fn process(&mut self, delta: f64) {
        let tmp = self.base().get_position() + self.direct.normalized() * delta as f32;
        self.base_mut().set_position(tmp);
    }
}

#[godot_api()]
impl StarWrathBullet {
    fn init() {}
}
