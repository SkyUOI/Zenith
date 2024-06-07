use godot::prelude::*;

#[macro_export()]
macro_rules! godot_assert {
    ($($tok:tt)*) => {
        if !$($tok)* {
            godot::prelude::godot_error!("{} failed", stringify!($($tok)*))
        }
    };
}

/// 将传入的弧度转换成单位向量
pub fn split_to_vec(vec: f32) -> Vector2 {
    Vector2::new(vec.cos(), vec.sin())
}
