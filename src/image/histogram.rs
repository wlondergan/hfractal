use super::super::math:: {
    window::WindowProperties,
    number::get_prec,
    color::color_hist,
    escape_iters,
};
use rayon::prelude::*;
use std::sync::{
    mpsc::channel,
    Arc,
    Mutex};
use std::thread;
use indicatif::ProgressBar;
use std::time::Duration;
use image::{RgbImage, Rgb, ImageFormat, ImageResult};
use rug::{Complex, Float};
use std::cmp;
use super::FractalType;

pub fn draw_image_histogram(path: &str, properties: WindowProperties, _fractal: FractalType) -> ImageResult<()> {
    let prec = properties.start_point.prec();
    let x_gap = Float::with_val(prec.0, properties.width_height.real() / properties.x_res);
    let y_gap = Float::with_val(prec.1, properties.width_height.imag() / properties.y_res);
    let value_prec = cmp::max(53, get_prec(&properties));
    println!("Render corner at {} with image dimension {}", properties.start_point, properties.width_height);
    println!("Render resolution @{}x{}", properties.x_res, properties.y_res);
    println!("Using mantissa precision {}", value_prec);

    let (sender, receiver) = channel();

    let count: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

    let w_count = Arc::clone(&count);
    let columns = properties.x_res;
    thread::spawn(move || {
        println!("Calculating pixels");
        let bar = ProgressBar::new(columns as u64);
        loop {
            if let Ok(ref mut mutex) = w_count.try_lock() {
                if **mutex == columns {
                    bar.finish_and_clear();
                    break
                } else {
                    bar.set_position(**mutex as u64);
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    (0..properties.x_res).map(|w| (0..properties.y_res).map(move |h| (w, h))).flatten().collect::<Vec<_>>().into_par_iter()
        .for_each_with((sender, Arc::clone(&count)), |s, x| {
        let (x, y) = x;
        let value = Complex::with_val(value_prec, 
            (properties.start_point.real() + &x_gap * Float::with_val(prec.0, x), 
            properties.start_point.imag() + &y_gap * Float::with_val(prec.1, y)));
        s.0.send((x, y, escape_iters(&value))).unwrap();
        if y == 0 {
            loop {
                if let Ok(ref mut mutex) = count.try_lock() {
                    **mutex += 1;
                    break
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    });

    println!("Finished calculating points");
    let mut image = RgbImage::new(properties.x_res, properties.y_res);
    let colors = color_hist(&receiver.iter().collect());
    for (x, y, color) in colors.iter() {
        image.put_pixel(*x, *y, *color);
    }
    
    image.save_with_format(path, ImageFormat::Png)
}
