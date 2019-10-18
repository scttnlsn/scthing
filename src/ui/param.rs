use crate::ui;
use raqote;
use rosc::{OscPacket,OscMessage,OscType};
use rosc::encoder;
use std::net::{UdpSocket};

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub value: f32,
    pub step: f32
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

impl Param {
    pub fn new(name: String, value: f32, step: f32) -> Self {
        Param {
            name: name,
            value: value,
            step: step,
        }
    }

    pub fn inc(&mut self) {
        self.value += self.step;
    }

    pub fn dec(&mut self) {
        self.value -= self.step;
    }

    pub fn send(&self) {
        osc_send("set", Some(vec![
            OscType::String(self.name.clone()),
            OscType::Float(self.value),
        ]));
    }
}

impl ui::Screen for Param {
    fn render(&self, target: &mut raqote::DrawTarget) {
        let lines = vec![
            format!("{} = {:?}", self.name, self.value)
        ];

        ui::render_lines(lines, target);
    }

    fn handle(&mut self, input: ui::Input) -> Option<ui::Action> {
        match input {
            ui::Input::Left => {
                self.dec();
                self.send();
                None
            },
            ui::Input::Right => {
                self.inc();
                self.send();
                None
            },
            ui::Input::Press => {
                Some(ui::Action::Pop)
            },
        }
    }

    fn load(&mut self) {
    }

    fn unload(&mut self) {
    }
}
