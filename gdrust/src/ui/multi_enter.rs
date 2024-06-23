use crate::{debug_check, get_multi_single};
use derive::gen_debug;
use godot::classes::{AcceptDialog, Button, IButton};
use godot::obj::WithBaseField;
use godot::prelude::*;
use std::net::TcpStream;

#[derive(GodotClass)]
#[class(base = Button)]
struct MultiEnter {
    base: Base<Button>,
    socket: Option<TcpStream>,
}

#[godot_api()]
impl IButton for MultiEnter {
    fn init(base: Base<Button>) -> Self {
        Self { base, socket: None }
    }

    fn ready(&mut self) {
        debug_check!(self)
    }
}

#[godot_api]
#[gen_debug]
impl MultiEnter {
    #[func]
    fn connect_to_server(&mut self, ip: String) -> bool {
        let multi_manager = get_multi_single();
        let mut lock = multi_manager.lock().unwrap();
        match lock.connect_to_server(ip) {
            Ok(_) => true,
            Err(err) => {
                self.show_dialog(err.to_string());
                false
            }
        }
    }

    #[debug]
    fn get_dialog(&self) -> Gd<AcceptDialog> {
        self.base().get_node_as("WrongDialog")
    }

    #[func]
    fn show_dialog(&mut self, text: String) {
        let mut dialog = self.get_dialog();
        dialog.set_text(text.into());
        dialog.popup_centered()
    }
}