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
        // TODO: can I allocate this dynamically based on struct size?
        // Is there a way to determine struct size at compile time?
        let mut buf: [u8; 16] = [0; 16];

        let bytes = self.file.read(&mut buf).unwrap();
        if bytes != mem::size_of::<InputEvent>() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid number of bytes read from input device",
            ));
        }

        let event: InputEvent = unsafe { mem::transmute(buf) };
        Ok(event)
    }
}
