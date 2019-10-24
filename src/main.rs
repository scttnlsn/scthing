#[macro_use] extern crate failure;

mod config;
mod framebuffer;
mod input;
mod osc;
mod ui;

use crate::framebuffer::Framebuffer;
use crate::input::InputDevice;
use crate::ui::build_ui;
use raqote;
use std::sync::mpsc;
use std::thread;

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

fn ui_loop(rx: mpsc::Receiver<ui::Input>, fb_device: String, menus: Vec<config::Menu>) {
    match Framebuffer::new(fb_device) {
        Ok(mut fb) => {
            let mut ui = build_ui(&menus);
            let mut target = raqote::DrawTarget::new(
                fb.var_screen_info.xres as i32,
                fb.var_screen_info.yres as i32,
            );

            loop {
                ui.render(&mut target);
                paint(&target, &mut fb);

                let input = rx.recv().unwrap();
                ui.handle(input);
            }
        },
        Err(_) => {
            println!("error opening framebuffer device");
            return;
        }
    }
}

fn enc_loop(tx: mpsc::Sender<ui::Input>, device: String) {
    match InputDevice::open(device) {
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

fn button_loop(tx: mpsc::Sender<ui::Input>, device: String) {
    match InputDevice::open(device) {
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
    let conf = config::parse("./config.toml").unwrap();

    let (tx, rx) = mpsc::channel();

    let fb_device = conf.devices.framebuffer;
    let menus = conf.menus;
    let ui_thread = thread::spawn(move || {
        ui_loop(rx, fb_device, menus);
    });

    let enc_device = conf.devices.encoder;
    let enc_tx = tx.clone();
    let enc_thread = thread::spawn(move || {
        enc_loop(enc_tx, enc_device);
    });

    let button_device = conf.devices.button;
    let button_tx = tx.clone();
    let button_thread = thread::spawn(move || {
        button_loop(button_tx, button_device);
    });

    ui_thread.join().unwrap();
    enc_thread.join().unwrap();
    button_thread.join().unwrap();
}
