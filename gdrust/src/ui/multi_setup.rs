use crate::multi::MessageReceiver;
use crate::{debug_check, get_multi_single};
use derive::gen_debug;
use godot::classes::{Control, IControl};
use godot::engine::{Label, Panel};
use godot::prelude::*;
use proto::connect;
use std::mem::swap;

#[derive(GodotClass)]
#[class(base = Control)]
struct MultiSetup {
    base: Base<Control>,
    receiver: Option<MessageReceiver>,
}

#[godot_api]
impl IControl for MultiSetup {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            receiver: None,
        }
    }

    fn ready(&mut self) {
        debug_check!(self)
    }

    fn process(&mut self, delta: f64) {
        match &mut self.receiver {
            None => {
                panic!("receiver empty");
            }
            Some(rec) => {
                while let Ok(data) = rec.try_recv() {
                    match data {
                        proto::ProtoRequest::Join(connect::Join { player_name, .. }) => {
                            godot_print!("{} joined", &player_name)
                        }
                        _ => {
                            godot_error!("Wrong msg:{:?}", data)
                        }
                    }
                }
            }
        }
    }
}

#[godot_api]
#[gen_debug()]
impl MultiSetup {
    fn setup(&mut self) {
        let mut server = get_multi_single().lock().unwrap();
        server.set_up_server();
        self.receiver = server.borrow_receiver();
    }

    #[debug]
    fn get_panel(&mut self) -> Gd<Label> {
        self.base().get_node_as("Panel/Players")
    }
}

impl Drop for MultiSetup {
    fn drop(&mut self) {
        let mut tmp = None;
        swap(&mut tmp, &mut self.receiver);
        get_multi_single()
            .lock()
            .unwrap()
            .give_back_receiver(tmp.unwrap())
    }
}

