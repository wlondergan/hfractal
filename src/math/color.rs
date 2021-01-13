use image::Rgb;
use std::f64;

const LOG_ESCAPE: f64 = 6.907_755_278_982_137; // maximum precision f64 of the log of 1000; adjust when changing ESCAPE_ITERS
const ESCAPE_FACTOR: f64 = LOG_ESCAPE / 255.0;
const COLOR_ROTATIONS: f64 = 2.0;
const R_SHIFT: f64 = 0.0;
const G_SHIFT: f64 = 1.0/3.0 * 2.0 * f64::consts::PI;
const B_SHIFT: f64 = 2.0/3.0 * 2.0 * f64::consts::PI;



#[inline]
pub fn color_naive(iters: usize) -> u8 {
    f64::ceil((LOG_ESCAPE - f64::ln(iters as f64)) / ESCAPE_FACTOR) as u8
}

pub fn color_hist<T: Clone, U: Clone>(escapes: &Vec<(T, U, usize)>) -> Vec<(T, U, Rgb<u8>)> {
    assert!(escapes.len() > 0);
    let mut max = escapes[0].2;
    let mut min = max;
    for (_t, _u, iters) in escapes.iter() {
        if *iters > max {
            max = iters.clone();
        }
        if *iters < min {
            min = iters.clone();
        }
    }
    let max_val = correct_escape((max - min) as f64);
    escapes.iter().map(|x| (x.0.clone(), x.1.clone(), (max_val - correct_escape((x.2 - min) as f64)) / (max_val)))
    .map(|x| {
        let angle = COLOR_ROTATIONS * 2.0 * f64::consts::PI * x.2;
        let correct_x2 = correct_brightness(x.2);
        let r = (f64::cos(angle - R_SHIFT) + 1.0) / 2.0;
        let g = (f64::cos(angle - G_SHIFT) + 1.0) / 2.0;
        let b = (f64::cos(angle - B_SHIFT) + 1.0) / 2.0;
        (x.0, x.1, Rgb([(r * 255.0 * correct_x2) as u8, (g * 255.0 * correct_x2) as u8, (b * 255.0 * correct_x2) as u8]))
    }).collect()
}

#[inline]
fn correct_escape(i: f64) -> f64 {
    f64::ln(i)
}

#[inline]
fn correct_brightness(i: f64) -> f64 {
    f64::sqrt(i)
}

