use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Area2D)]
struct ZenithBegin {
    base: Base<Area2D>,
}

#[godot_api()]
impl IArea2D for ZenithBegin {
    fn init(base: Base<Area2D>) -> ZenithBegin {
        ZenithBegin { base }
    }
}
