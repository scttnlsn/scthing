mod framebuffer;
mod input;

use framebuffer::Framebuffer;
use input::InputDevice;

fn render(fb: &mut Framebuffer, n: usize) {
    let xres = fb.var_screen_info.xres;
    let yres = fb.var_screen_info.yres;
    let byte_depth = fb.var_screen_info.bits_per_pixel / 8;

    for x in (0..xres * byte_depth).step_by(byte_depth as usize) {
        for y in (0..yres * byte_depth).step_by(byte_depth as usize) {
            let index = (y * xres + x) as usize;

            if index % n == 0 {
                fb.frame[index] = 1;
            } else {
                fb.frame[index] = 0;
            }
        }
    }
}

fn main() {
    let path = "/dev/fb1";
    let mut fb = Framebuffer::new(path).unwrap();

    let xres = fb.var_screen_info.xres;
    let yres = fb.var_screen_info.yres;
    let byte_depth = fb.var_screen_info.bits_per_pixel / 8;
    println!("screen info: {}x{} ({} bpp)", xres, yres, byte_depth);

    let mut device = InputDevice::open("/dev/input/event0").unwrap();

    let mut n = 20;

    loop {
        render(&mut fb, n);

        let event = device.read_event().unwrap();

        if event.value == 1 {
            n += 1;
        } else if event.value == -1 {
            n -= 1;
        }
    }
}
