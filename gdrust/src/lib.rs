mod bullets;
mod cfg;
mod consts;
mod fight;
mod fight_items;
pub mod global;
mod multi;
mod player;
mod ui;
mod utils;
mod weapons;
mod zenith;

use godot::prelude::*;
use multi::MultiManagerImpl;
use std::{
    panic::{set_hook, PanicInfo},
    sync::{Arc, Mutex, OnceLock},
};
use tokio::runtime::{Builder, Runtime};

struct GdExtension;

type MultiSingle = Arc<Mutex<MultiManagerImpl>>;
fn get_multi_single() -> &'static MultiSingle {
    static TMP: OnceLock<MultiSingle> = OnceLock::new();
    TMP.get_or_init(|| Arc::new(Mutex::new(MultiManagerImpl::new())))
}

fn get_tokio_runtime() -> &'static Runtime {
    static TMP: OnceLock<Runtime> = OnceLock::new();
    TMP.get_or_init(|| Builder::new_multi_thread().enable_all().build().unwrap())
}

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
