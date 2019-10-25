#![feature(const_fn)]

#[macro_use]
extern crate failure;

mod config;
mod framebuffer;
mod input;
mod osc;
mod ui;

use crate::framebuffer::Framebuffer;
use crate::input::InputDevice;
use crate::ui::build_ui;
use clap::{Arg, App};
use raqote;
use std::sync::mpsc;
use std::thread;

fn ui_loop(rx: mpsc::Receiver<ui::Input>) {
    let conf = &config::CONFIG.get();

    match Framebuffer::new(&conf.devices.framebuffer) {
        Ok(mut fb) => {
            let mut ui = build_ui(&conf.menus);
            let mut target = raqote::DrawTarget::new(
                fb.var_screen_info.xres as i32,
                fb.var_screen_info.yres as i32,
            );

            loop {
                ui.render(&mut target);
                fb.draw(target.get_data());

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

fn enc_loop(tx: mpsc::Sender<ui::Input>) {
    let conf = &config::CONFIG.get();

    match InputDevice::open(&conf.devices.encoder) {
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
    let conf = &config::CONFIG.get();

    match InputDevice::open(&conf.devices.button) {
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
    let matches = App::new("")
        .version("0.1.0")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("config file path")
             .takes_value(true)
             .required(true))
        .get_matches();

    let conf_path = matches.value_of("config").unwrap();
    let conf = config::parse(conf_path).unwrap();

    config::CONFIG.set(conf);

    let (tx, rx) = mpsc::channel();

    let ui_thread = thread::spawn(move || {
        ui_loop(rx);
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
