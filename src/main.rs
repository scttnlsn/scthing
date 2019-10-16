mod framebuffer;
mod input;
mod menu;
mod ui;

use framebuffer::Framebuffer;
use input::InputDevice;
use menu::{Menu, MenuItem};
use raqote;
use std::sync::mpsc;
use std::thread;

const FB_DEVICE: &str = "/dev/fb1";
const ENC_DEVICE: &str = "/dev/input/by-path/platform-rotary@11-event";
const BUTTON_DEVICE: &str = "/dev/input/by-path/platform-button@16-event";

fn paint(target: &raqote::DrawTarget, fb: &mut Framebuffer) {
    let pixel_data = target.get_data();

    let xres = fb.var_screen_info.xres;
    let yres = fb.var_screen_info.yres;
    let byte_depth = (fb.var_screen_info.bits_per_pixel / 8) as usize;

    for x in 0..xres {
        for y in 0..yres {
            let index = (y * xres + x) as usize;

            if pixel_data[index] > 0 {
                fb.frame[index * byte_depth] = 1;
            } else {
                fb.frame[index * byte_depth] = 0;
            }
        }
    }
}

fn ui_loop(rx: mpsc::Receiver<ui::Input>, mut fb: Framebuffer) {
    let mut target = raqote::DrawTarget::new(
        fb.var_screen_info.xres as i32,
        fb.var_screen_info.yres as i32,
    );

    let menu = Menu::new(vec![
        MenuItem::menu(
            "ITEM 1",
            vec![
                MenuItem::item(
                    "SUB ITEM 1-1",
                    || { println!("1-1"); },
                ),
                MenuItem::item(
                    "SUB ITEM 1-2",
                    || { println!("1-2"); },
                )
            ]
        ),
        MenuItem::menu(
            "ITEM 2",
            vec![
                MenuItem::item(
                    "SUB ITEM 2-1",
                    || { println!("2-1"); },
                ),
                MenuItem::item(
                    "SUB ITEM 2-2",
                    || { println!("2-2"); },
                )
            ]
        ),
    ]);

    let mut ui = ui::UI::new();
    ui.push_screen(menu);

    loop {
        ui.render(&mut target);
        paint(&target, &mut fb);

        let message = rx.recv().unwrap();
        ui.handle(message);
    }
}

fn enc_loop(tx: mpsc::Sender<ui::Input>) {
    match InputDevice::open(ENC_DEVICE) {
        Ok(mut device) => {
            loop {
                let event = device.read_event().unwrap();

                if event.value == 1 {
                    tx.send(ui::Input::Right).unwrap();
                } else if event.value == -1 {
                    tx.send(ui::Input::Left).unwrap();
                }
            }
        },
        Err(_) => {
            println!("error opening encoder device");
            return;
        }
    }
}

fn button_loop(tx: mpsc::Sender<ui::Input>) {
    match InputDevice::open(BUTTON_DEVICE) {
        Ok(mut device) => {
            loop {
                let event = device.read_event().unwrap();

                if event.value == 1 {
                    tx.send(ui::Input::Press).unwrap();
                }
            }
        },
        Err(_) => {
            println!("error opening button device");
            return;
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
