mod framebuffer;
mod input;

use framebuffer::Framebuffer;
use input::InputDevice;
use std::sync::mpsc;
use std::thread;

enum Message {
    Inc,
    Dec,
    Reset,
}

fn render(fb: &mut Framebuffer, n: u32) {
    let xres = fb.var_screen_info.xres;
    let yres = fb.var_screen_info.yres;
    let byte_depth = fb.var_screen_info.bits_per_pixel / 8;

    for x in (0..xres * byte_depth).step_by(byte_depth as usize) {
        for y in (0..yres * byte_depth).step_by(byte_depth as usize) {
            let index = (y * xres + x) as usize;

            if index % (n as usize) == 0 {
                fb.frame[index] = 1;
            } else {
                fb.frame[index] = 0;
            }
        }
    }
}

fn ui_loop(rx: mpsc::Receiver<Message>, mut fb: Framebuffer) {
    let mut n: u32 = 20;

    loop {
        match rx.recv().unwrap() {
            Message::Inc => { n += 1 },
            Message::Dec => { n -= 1 },
            Message::Reset => { n = 20 },
        }

        render(&mut fb, n);
    }
}

fn enc_loop(tx: mpsc::Sender<Message>) {
    let mut device = InputDevice::open("/dev/input/event0").unwrap();

    loop {
        let event = device.read_event().unwrap();

        if event.value == 1 {
            tx.send(Message::Inc).unwrap();
        } else if event.value == -1 {
            tx.send(Message::Dec).unwrap();
        }
    }
}

fn button_loop(tx: mpsc::Sender<Message>) {
    let mut device = InputDevice::open("/dev/input/event1").unwrap();

    loop {
        let event = device.read_event().unwrap();

        if event.value == 1 {
            tx.send(Message::Reset).unwrap();
        }
    }
}

fn main() {
    let path = "/dev/fb1";
    let fb = Framebuffer::new(path).unwrap();

    let xres = fb.var_screen_info.xres;
    let yres = fb.var_screen_info.yres;
    let byte_depth = fb.var_screen_info.bits_per_pixel / 8;
    println!("screen info: {}x{} ({} bpp)", xres, yres, byte_depth);

    let (tx, rx) = mpsc::channel();

    let ui_thread = thread::spawn(move || {
        ui_loop(rx, fb);
    });

    let enc_tx = tx.clone();
    let enc_thread = thread::spawn(move || {
        enc_loop(enc_tx);
    });

    let button_tx = tx.clone();
    let button_thread = thread::spawn(move || {
        button_loop(button_tx);
    });

    ui_thread.join().unwrap();
    enc_thread.join().unwrap();
    button_thread.join().unwrap();
}
