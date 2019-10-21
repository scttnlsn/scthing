mod framebuffer;
mod input;
mod osc;
mod ui;

use crate::framebuffer::Framebuffer;
use crate::input::InputDevice;
use crate::ui::menu::{Menu, MenuItem};
use crate::ui::patch::Patch;
use crate::ui::param::Param;
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

    let mut ui = ui::UI::new();

    let back = MenuItem::new("<-", ui::Action::Pop);

    let freq_param = ui.register(
        Param::new("freq", 440.0, 10.0, 20.0, 20000.0)
    );

    let tone_patch = ui.register(Patch::new("tone", Menu::new(vec![
        MenuItem::new("freq", ui::Action::Push(freq_param)),
        back.clone(),
    ])));

    let speed_param = ui.register(Param::new("speed", 2.0, 0.2, 0.1, 10.0));
    let depth_param = ui.register(Param::new("depth", 0.5, 0.1, 0.0, 1.0));

    let trem_patch = ui.register(Patch::new("trem", Menu::new(
        vec![
            MenuItem::new("speed", ui::Action::Push(speed_param)),
            MenuItem::new("depth", ui::Action::Push(depth_param)),
            back.clone(),
        ]
    )));

    let testing_menu = ui.register(Menu::new(vec![
        MenuItem::new("tone", ui::Action::Push(tone_patch)),
        back.clone(),
    ]));

    let effects_menu = ui.register(Menu::new(vec![
        MenuItem::new("tremolo", ui::Action::Push(trem_patch)),
        back.clone(),
    ]));

    let root_menu = ui.register(Menu::new(vec![
        MenuItem::new("effects", ui::Action::Push(effects_menu)),
        MenuItem::new("testing", ui::Action::Push(testing_menu)),
    ]));

    ui.push_screen(root_menu);

    loop {
        ui.render(&mut target);
        paint(&target, &mut fb);

        let input = rx.recv().unwrap();
        ui.handle(input);
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
