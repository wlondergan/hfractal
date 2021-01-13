use super::super::math::{
    window::WindowProperties,
    color::color_naive,
    number::get_prec,
    escape_iters,
};
use super::{FractalType};
use rug::{Complex, Float};
use image::{RgbImage, Rgb, ImageResult, ImageFormat};
use std::cmp;

/// Creates a png representation of the given window properties and fractal type at the given path.
pub fn draw_image(path: &str, properties: WindowProperties, _fractal: FractalType) -> ImageResult<()> {
    let mut image = RgbImage::new(properties.x_res, properties.y_res);
    let prec = properties.start_point.prec();
    let x_gap = Float::with_val(prec.0, properties.width_height.real() / properties.x_res);
    let y_gap = Float::with_val(prec.1, properties.width_height.imag() / properties.y_res);
    let value_prec = cmp::max(53, get_prec(&properties));
    println!("{}", value_prec);
    for x in 0..properties.x_res {
        if x % 10 == 0 {
            println!("Finished row {}", x);
        }
        for y in 0..properties.y_res {
            let value = Complex::with_val(value_prec, 
                (properties.start_point.real() + &x_gap * Float::with_val(prec.0, x), 
                properties.start_point.imag() + &y_gap * Float::with_val(prec.1, y)));
            let escape_color = color_naive(escape_iters(&value));
            image.put_pixel(x, y, Rgb([escape_color, escape_color, escape_color]));
        }
    }
    image.save_with_format(path, ImageFormat::Png)
}