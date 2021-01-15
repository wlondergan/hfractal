use hmand::image::{draw_image_histogram, FractalType};
use hmand::math::window::WindowProperties;
use rug::{Complex, Float};

const WIDTH: &str = "7.111E-5";
const HEIGHT: &str = "4E-5";
const CENTER_X: &str = "-0.235125";
const CENTER_Y: &str = "0.827215";
const X_RES: u32 = 1920;
const Y_RES: u32 = 1080;
const OVERSAMPLE_RATE: u32 = 6; // The amount of extra samples that are taken to increase precision.

fn main() {
    let width = match Float::parse(WIDTH) {
        Ok(f) => Float::with_val(100, f),
        Err(_) => unreachable!(),
    };
    let height = match Float::parse(HEIGHT) {
        Ok(f) => Float::with_val(100, f),
        Err(_) => unreachable!(),
    };
    let center_x = match Float::parse(CENTER_X) {
        Ok(f) => Float::with_val(100, f),
        Err(_) => unreachable!(),
    };
    let center_y = match Float::parse(CENTER_Y) {
        Ok(f) => Float::with_val(100, f),
        Err(_) => unreachable!(),
    };

    let window = WindowProperties {
        x_res: X_RES,
        y_res: Y_RES,
        width_height: Complex::with_val((width.prec(), height.prec()), (&width, &height)),
        start_point: Complex::with_val(
            (width.prec(), height.prec()),
            (center_x - (width / 2), center_y - (height / 2)),
        ),
    };
    //draw_image("render.png", window, FractalType::Mandelbrot);
    draw_image_histogram("render.png", window, FractalType::Mandelbrot).unwrap();
}
