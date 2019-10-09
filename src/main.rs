mod framebuffer;
mod input;

use framebuffer::Framebuffer;
use input::InputDevice;
use std::sync::mpsc;
use std::thread;
use raqote;

enum Message {
    Inc,
    Dec,
    Reset,
}

const FB_DEVICE: &str = "/dev/fb1";
const ENC_DEVICE: &str = "/dev/input/event0";
const BUTTON_DEVICE: &str = "/dev/input/event1";

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

fn paint(target: &raqote::DrawTarget, fb: &mut Framebuffer) {
    let pixel_data = target.get_data();

    let xres = fb.var_screen_info.xres;
    let yres = fb.var_screen_info.yres;
    let byte_depth = fb.var_screen_info.bits_per_pixel / 8;

    for x in (0..xres * byte_depth).step_by(byte_depth as usize) {
        for y in (0..yres * byte_depth).step_by(byte_depth as usize) {
            let index = (y * xres + x) as usize;

            if pixel_data[index] & 0x1 > 1 {
                fb.frame[index] = 1;
            } else {
                fb.frame[index] = 0;
            }
        }
    }
}

fn ui_loop(rx: mpsc::Receiver<Message>, mut fb: Framebuffer) {
    let mut target = raqote::DrawTarget::new(
        fb.var_screen_info.xres as i32,
        fb.var_screen_info.yres as i32,
    );

    let mut n: u32 = 20;

    loop {
        match rx.recv().unwrap() {
            Message::Inc => { n += 1 },
            Message::Dec => { n -= 1 },
            Message::Reset => { n = 20 },
        }

        if n <= 0 {
            n = 1;
        }

        let mut pb = raqote::PathBuilder::new();
        pb.move_to(20., 20.);
        pb.line_to(20., 80.);
        pb.line_to(80., 80.);
        let path = pb.finish();

        let background = raqote::SolidSource { r: 0x0, g: 0x0, b: 0x0, a: 0x0 };
        let foreground = raqote::SolidSource { r: 0x1, g: 0x1, b: 0x1, a: 0x1 };
        let draw_options = raqote::DrawOptions::new();

        target.clear(background);
        target.stroke(
            &path,
            &raqote::Source::Solid(foreground),
            &raqote::StrokeStyle {
                cap: raqote::LineCap::Round,
                join: raqote::LineJoin::Round,
                width: 1.,
                miter_limit: 1.,
                dash_array: vec![10., 18.],
                dash_offset: 16.,
            },
            &draw_options
        );

        // render(&mut fb, n);

        paint(&target, &mut fb);
    }
}

fn enc_loop(tx: mpsc::Sender<Message>) {
    let mut device = InputDevice::open(ENC_DEVICE).unwrap();

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
    let mut device = InputDevice::open(BUTTON_DEVICE).unwrap();

    loop {
        let event = device.read_event().unwrap();

        if event.value == 1 {
            tx.send(Message::Reset).unwrap();
        }
    }
}

fn main() {
    let fb = Framebuffer::new(FB_DEVICE).unwrap();

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
