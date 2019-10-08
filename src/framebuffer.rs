use libc::ioctl;
use memmap::{MmapOptions, MmapMut};
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path};
use std::os::unix::io::AsRawFd;

const FBIOGET_VSCREENINFO: libc::c_ulong = 0x4600;
const FBIOGET_FSCREENINFO: libc::c_ulong = 0x4602;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Bitfield {
    pub offset: u32,
    pub length: u32,
    pub msb_right: u32,
}

// Struct definitons from linux/fb.h

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VarScreenInfo {
    pub xres: u32,
    pub yres: u32,
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub bits_per_pixel: u32,
    pub grayscale: u32,
    pub red: Bitfield,
    pub green: Bitfield,
    pub blue: Bitfield,
    pub transp: Bitfield,
    pub nonstd: u32,
    pub activate: u32,
    pub height: u32,
    pub width: u32,
    pub accel_flags: u32,
    pub pixclock: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub sync: u32,
    pub vmode: u32,
    pub rotate: u32,
    pub colorspace: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FixScreenInfo {
    pub id: [u8; 16],
    pub smem_start: usize,
    pub smem_len: u32,
    pub fb_type: u32,
    pub type_aux: u32,
    pub visual: u32,
    pub xpanstep: u16,
    pub ypanstep: u16,
    pub ywrapstep: u16,
    pub line_length: u32,
    pub mmio_start: usize,
    pub mmio_len: u32,
    pub accel: u32,
    pub capabilities: u16,
    pub reserved: [u16; 2],
}

impl ::std::default::Default for Bitfield {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl ::std::default::Default for VarScreenInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl ::std::default::Default for FixScreenInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub struct Framebuffer {
    pub device: File,
    pub frame: MmapMut,
    pub var_screen_info: VarScreenInfo,
    pub fix_screen_info: FixScreenInfo,
}

#[derive(Debug)]
pub enum FramebufferErrorKind {
    IoctlFailed,
    IoError,
}

#[derive(Debug)]
pub struct FramebufferError {
    pub kind: FramebufferErrorKind,
    pub details: String,
}

impl FramebufferError {
    fn new(kind: FramebufferErrorKind, details: &str) -> FramebufferError {
        FramebufferError {
            kind,
            details: String::from(details),
        }
    }
}

impl std::error::Error for FramebufferError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for FramebufferError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl std::convert::From<io::Error> for FramebufferError {
    fn from(err: io::Error) -> FramebufferError {
        FramebufferError::new(FramebufferErrorKind::IoError, err.description())
    }
}

fn get_var_screen_info(device: &File) -> Result<VarScreenInfo, FramebufferError> {
    let mut info: VarScreenInfo = Default::default();
    let result = unsafe {
        ioctl(device.as_raw_fd(), FBIOGET_VSCREENINFO as _, &mut info)
    };

    match result {
        -1 => {
            let err = FramebufferError::new(FramebufferErrorKind::IoctlFailed, "ioctl failed");
            Err(err)
        },
        _ => Ok(info),
    }
}

fn get_fix_screen_info(device: &File) -> Result<FixScreenInfo, FramebufferError> {
    let mut info: FixScreenInfo = Default::default();
    let result = unsafe {
        ioctl(device.as_raw_fd(), FBIOGET_FSCREENINFO as _, &mut info)
    };

    match result {
        -1 => {
            let err = FramebufferError::new(FramebufferErrorKind::IoctlFailed, "ioctl failed");
            Err(err)
        },
        _ => Ok(info),
    }
}

impl Framebuffer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Framebuffer, FramebufferError> {
        let device = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;

        let var_screen_info = get_var_screen_info(&device)?;
        let fix_screen_info = get_fix_screen_info(&device)?;

        // TODO: read ioctl to get resolution
        let frame_len = 128 * 64 * 2;

        let frame = unsafe {
            MmapOptions::new().len(frame_len).map_mut(&device)
        };

        match frame {
            Ok(result) => Ok(Framebuffer {
                device: device,
                frame: result,
                var_screen_info: var_screen_info,
                fix_screen_info: fix_screen_info,
            }),
            Err(_) => Err(FramebufferError::new(
                FramebufferErrorKind::IoError,
                "mmap failed",
            ))
        }
    }
}
