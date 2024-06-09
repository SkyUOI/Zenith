use godot::prelude::*;

#[macro_export()]
macro_rules! godot_assert {
    ($($tok:tt)*) => {
        if !($($tok)*) {
            godot::prelude::godot_error!("{} failed", stringify!($($tok)*))
        }
    };
}

#[macro_export()]
macro_rules! godot_debug_assert {
    ($($tok:tt)*) => {
        #[cfg(debug_assertions)] {
            if !($($tok)*) {
                godot::prelude::godot_error!("{} failed", stringify!($($tok)*))
            }
        }
    };
}

#[macro_export]
macro_rules! debug_check {
    ($sself:ident) => {
        #[cfg(debug_assertions)]
        {
            $sself.debug_check()
        }
    };
}

/// 将传入的弧度转换成单位向量
pub fn split_to_vec(rad: f32) -> Vector2 {
    Vector2::new(rad.cos(), rad.sin())
}
