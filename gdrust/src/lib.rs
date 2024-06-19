mod bullets;
mod consts;
mod fight;
mod fight_items;
mod global;
mod multi;
mod player;
mod ui;
mod utils;
mod weapons;
mod zenith;

use godot::prelude::*;
use std::panic::{set_hook, PanicInfo};

struct GdExtension;

fn panic_handler(info: &PanicInfo) {
    if let Some(p) = info.location() {
        godot_error!(
            "Panic occurred in file '{}' at line {}\n",
            p.file(),
            p.line()
        );
    } else {
        godot_error!("Panic occurred but can't get location information.");
    }
}

#[gdextension()]
unsafe impl ExtensionLibrary for GdExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            set_hook(Box::new(panic_handler))
        }
    }
}
