mod framebuffer;
mod input;
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

enum ScreenId {
    MainMenu,
    TonePatch,
    TremPatch,
    FreqParam,
    SpeedParam,
    DepthParam,
}

fn ui_loop(rx: mpsc::Receiver<ui::Input>, mut fb: Framebuffer) {
    let mut target = raqote::DrawTarget::new(
        fb.var_screen_info.xres as i32,
        fb.var_screen_info.yres as i32,
    );

    let mut ui = ui::UI::new();

    let menu = Menu::new(vec![
        MenuItem::menu(
            "TESTING".to_string(),
            vec![
                MenuItem::item(
                    "TONE".to_string(),
                    ui::Action::Push(ScreenId::TonePatch as u32),
                ),
            ]
        ),
        MenuItem::menu(
            "EFFECTS".to_string(),
            vec![
                MenuItem::item(
                    "TREMOLO".to_string(),
                    ui::Action::Push(ScreenId::TremPatch as u32),
                ),
            ]
        ),
    ]);
    ui.register(ScreenId::MainMenu as u32, Box::new(menu));

    let tone_param = Param::new("freq".to_string(), 440.0, 10.0);
    ui.register(ScreenId::FreqParam as u32, Box::new(tone_param));

    let tone = Patch::new("tone".to_string(), Menu::new(
        vec![
            MenuItem::item("freq".to_string(), ui::Action::Push(ScreenId::FreqParam as u32)),
            MenuItem::item("<-".to_string(), ui::Action::Pop),
        ]
    ));
    ui.register(ScreenId::TonePatch as u32, Box::new(tone));

    let speed_param = Param::new("speed".to_string(), 2.0, 0.2);
    ui.register(ScreenId::SpeedParam as u32, Box::new(speed_param));

    let depth_param = Param::new("depth".to_string(), 0.5, 0.1);
    ui.register(ScreenId::DepthParam as u32, Box::new(depth_param));

    let trem = Patch::new("trem".to_string(), Menu::new(
        vec![
            MenuItem::item("speed".to_string(), ui::Action::Push(ScreenId::SpeedParam as u32)),
            MenuItem::item("depth".to_string(), ui::Action::Push(ScreenId::DepthParam as u32)),
            MenuItem::item("<-".to_string(), ui::Action::Pop),
        ]
    ));
    ui.register(ScreenId::TremPatch as u32, Box::new(trem));

    ui.push_screen(ScreenId::MainMenu as u32);

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
