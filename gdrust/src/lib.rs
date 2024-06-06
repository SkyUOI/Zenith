mod bullets;
mod consts;
mod fight_items;
mod player;
mod weapons;
mod zenith;

use godot::prelude::*;

struct GdExtension;

#[gdextension()]
unsafe impl ExtensionLibrary for GdExtension {}
