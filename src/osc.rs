use crate::config;
use std::io;
use rand::Rng;
use rosc::{OscPacket,OscMessage,OscType};
use rosc::encoder;
use std::net::{UdpSocket};

pub type Type = OscType;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "OSC error")]
    OscError {
        error: rosc::OscError,
    },
    #[fail(display = "IO error: {}", error)]
    SocketError {
        error: io::Error,
    },
}

impl From<rosc::OscError> for Error {
    fn from(error: rosc::OscError) -> Error {
        Error::OscError { error: error }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::SocketError { error: error }
    }
}

fn check_port(port: u16) -> bool {
    match UdpSocket::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn random_port() -> u16 {
    let mut rng = rand::thread_rng();

    loop {
        let port = rng.gen_range(1024, 65535);

        if check_port(port) {
            return port;
        }
    }
}

pub fn send(name: &str, args: Option<Vec<OscType>>) -> Result<(), Error> {
    let packet = OscPacket::Message(
        OscMessage {
            addr: name.to_string(),
            args: args,
        }
    );

    let bytes = encoder::encode(&packet)?;
    let port = random_port();
    let socket = UdpSocket::bind(("127.0.0.1", port))?;

    let conf = &*config::CONFIG.get();
    socket.send_to(&bytes, &conf.osc.addr)?;

    Ok(())
}
