#![feature(associated_type_bounds)]

use image::{RgbImage, Rgb, ImageFormat, ImageResult, ImageBuffer, Pixel};
use rug::{Complex, Float};
use std::f64;
use std::cmp;
use super::math:: {
    window::WindowProperties,
    number::get_prec,
    escape_iters,
    ESCAPE_ITERS
};
use rayon::prelude::*;
use std::sync::{
    mpsc::channel,
    Arc};

const LOG_1000: f64 = 6.907_755_278_982_137; // maximum precision f64 of the log of 1000
const ESCAPE_FACTOR: f64 = LOG_1000 / 255.0;

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
            let escape = escape_iters(&value);
            let escape_color = f64::ceil((LOG_1000 - f64::ln(escape as f64)) / ESCAPE_FACTOR) as u8;
            image.put_pixel(x, y, Rgb([escape_color, escape_color, escape_color]));
        }
    }
    image.save_with_format(path, ImageFormat::Png)
}

pub fn draw_image_parallel(path: &str, properties: WindowProperties, _fractal: FractalType) -> ImageResult<()> {
    let prec = properties.start_point.prec();
    let x_gap = Float::with_val(prec.0, properties.width_height.real() / properties.x_res);
    let y_gap = Float::with_val(prec.1, properties.width_height.imag() / properties.y_res);
    let value_prec = cmp::max(53, get_prec(&properties));
    println!("Using mantissa precision {}", value_prec);

    let (sender, receiver) = channel();

    (0..properties.x_res * properties.y_res).into_par_iter().for_each_with(sender, |s, x| {
        let (x, y) = get_position(x, properties.x_res, properties.y_res);
        let value = Complex::with_val(value_prec, 
            (properties.start_point.real() + &x_gap * Float::with_val(prec.0, x), 
            properties.start_point.imag() + &y_gap * Float::with_val(prec.1, y)));
        let escape = escape_iters(&value);
        let escape_color = f64::ceil((LOG_1000 - f64::ln(escape as f64)) / ESCAPE_FACTOR) as u8;
        s.send((x, y, Rgb([escape_color, escape_color, escape_color]))).unwrap();
    });

    println!("Finished calculating points");
    let mut image = RgbImage::new(properties.x_res, properties.y_res);
    for elem in receiver.iter() {
        let (x, y, rgb) = elem;
        image.put_pixel(x, y, rgb);
    }
    
    image.save_with_format(path, ImageFormat::Png)
}

#[inline]
fn get_position(z: u32, _width: u32, height: u32) -> (u32, u32) {
    (z / height, z % height)
}

/// Represents all of the different fractal types that this library can image-ify.
/// Might actually get used in the future.
pub enum FractalType {
    Mandelbrot,
}