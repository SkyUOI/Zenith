use crate::multi::MessageReceiver;
use crate::{debug_check, get_multi_single};
use derive::gen_debug;
use godot::classes::{Control, IControl};
use godot::engine::Label;
use godot::prelude::*;
use proto::connect;

#[derive(GodotClass)]
#[class(base = Control)]
struct MultiSetup {
    base: Base<Control>,
    players: Vec<String>,
    receiver: Option<MessageReceiver>,
}

#[godot_api]
impl IControl for MultiSetup {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            receiver: None,
            players: vec![],
        }
    }

    fn ready(&mut self) {
        debug_check!(self)
    }

    fn process(&mut self, delta: f64) {
        let mut receiver = self.receiver.take();
        match receiver {
            None => {
                panic!("receiver empty");
            }
            Some(ref rec) => {
                while let Ok(data) = rec.try_recv() {
                    match data {
                        proto::ProtoRequest::Join(connect::Join { player_name, .. }) => {
                            godot_print!("{} joined", &player_name);
                            self.players.push(player_name);
                            let text = self.gen_player_text();
                            self.get_player_text().set_text(text.into());
                        }
                        _ => {
                            godot_error!("Wrong msg:{:?}", data)
                        }
                    }
                }
            }
        }
        self.receiver = receiver.take()
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
    fn get_player_text(&mut self) -> Gd<Label> {
        self.base().get_node_as("Panel/Players")
    }

    #[func]
    fn gen_player_text(&self) -> String {
        let mut ret = String::new();
        for i in self.players.iter() {
            ret.push_str(i);
            ret.push('\n')
        }
        ret
    }
}

impl Drop for MultiSetup {
    fn drop(&mut self) {
        let tmp = self.receiver.take();
        if tmp.is_none() {
            return;
        }
        get_multi_single()
            .lock()
            .unwrap()
            .give_back_receiver(tmp.unwrap())
    }
}
