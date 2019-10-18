use rosc::{OscPacket,OscMessage,OscType};
use rosc::encoder;
use std::net::{UdpSocket};

pub type Type = OscType;

pub fn send(name: &str, args: Option<Vec<OscType>>) {
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
