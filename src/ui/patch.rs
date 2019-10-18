use crate::ui;
use raqote;
use rosc::{OscPacket,OscMessage,OscType};
use rosc::encoder;
use std::net::{UdpSocket};

#[derive(Debug)]
pub struct Patch {
    name: String,
    menu: ui::menu::Menu,
}

fn osc_send(name: &str, args: Option<Vec<OscType>>) {
    let packet = OscPacket::Message(
        OscMessage {
            addr: name.to_string(),
            args: args,
        }
    );

    let bytes = encoder::encode(&packet).unwrap();
    let socket = UdpSocket::bind("127.0.0.1:56874").unwrap();
    socket.send_to(&bytes, "127.0.0.1:57120").unwrap();
}

impl Patch {
    pub fn new(name: String, menu: ui::menu::Menu) -> Self {
        Patch {
            name: name,
            menu: menu,
        }
    }

    pub fn start(&self) {
        osc_send("start", Some(vec![OscType::String(self.name.clone())]));
    }

    pub fn stop(&self) {
        osc_send("stop", None);
    }
}

impl ui::Screen for Patch {
    fn render(&self, target: &mut raqote::DrawTarget) {
        self.menu.render(target);
    }

    fn handle(&mut self, input: ui::Input) -> Option<ui::Action> {
        self.menu.handle(input)
    }

    fn load(&mut self) {
        self.start();
    }

    fn unload(&mut self) {
        self.stop();
    }
}
