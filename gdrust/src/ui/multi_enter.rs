use crate::debug_check;
use derive::gen_debug;
use godot::engine::{AcceptDialog, Button, IButton};
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
        let ret;
        (self.socket, ret) = match TcpStream::connect(&ip) {
            Ok(socket) => (Some(socket), true),
            Err(error) => {
                self.show_dialog(format!("Connect failed:{}", error));
                (None, false)
            }
        };
        ret
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
