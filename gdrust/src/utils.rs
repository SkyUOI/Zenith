#[macro_export()]
macro_rules! godot_assert {
    ($($tok:tt)*) => {
        if !$($tok)* {
            godot::prelude::godot_error!("{} failed", stringify!($($tok)*))
        }
    };
}

