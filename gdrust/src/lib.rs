mod Zenith;
mod fight_items;
mod player;

use godot::prelude::*;

struct GdExtension;

#[gdextension()]
unsafe impl ExtensionLibrary for GdExtension {}
