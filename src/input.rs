use std::fs::File;
use std::io;
use std::io::Read;
use std::mem;
use std::path::{Path};

#[repr(C)]
#[derive(Debug)]
pub struct Timeval {
    pub tv_sec: isize,
    pub tv_usec: isize,
}

// from linux/input.h

#[repr(C)]
#[derive(Debug)]
pub struct InputEvent {
    pub time: Timeval,
    pub type_: u16,
    pub code: u16,
    pub value: i32,
}

const INPUT_EVENT_SIZE: usize = mem::size_of::<InputEvent>();

#[derive(Debug)]
pub struct InputDevice {
    file: File
}

impl InputDevice {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<InputDevice, io::Error> {
        let file = File::open(path)?;
        Ok(InputDevice { file: file })
    }

    pub fn read_event(self: &mut Self) -> Result<InputEvent, io::Error> {
        let mut buf: [u8; INPUT_EVENT_SIZE] = [0; INPUT_EVENT_SIZE];

        let bytes = self.file.read(&mut buf)?;
        if bytes != INPUT_EVENT_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid number of bytes read from input device",
            ));
        }

        let event: InputEvent = unsafe { mem::transmute(buf) };
        Ok(event)
    }
}
