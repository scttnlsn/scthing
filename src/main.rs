mod framebuffer;

fn main() {
    let path = "/dev/fb1";
    let mut fb = framebuffer::Framebuffer::new(path).unwrap();

    let xres = fb.var_screen_info.xres;
    let yres = fb.var_screen_info.yres;
    let byte_depth = fb.var_screen_info.bits_per_pixel / 8;

    println!("screen info: {}x{} ({} bpp)", xres, yres, byte_depth);

    for x in (0..xres * byte_depth).step_by(byte_depth as usize) {
        for y in (0..yres * byte_depth).step_by(byte_depth as usize) {
            let index = (y * xres + x) as usize;

            if index % 20 == 0 {
                fb.frame[index] = 1;
            }
        }
    }
}
