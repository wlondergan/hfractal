pub mod number;
pub mod window;
use std::cmp;
use rug::Complex;
use window::WindowProperties;

pub const ESCAPE_ITERS: usize = 1000;
const ESCAPE_SIZE: usize = 1000;
const LOG_ESCAPE: f64 = 6.907_755_278_982_137; // maximum precision f64 of the log of 1000; adjust when changing log_escape
const ESCAPE_FACTOR: f64 = LOG_ESCAPE / 255.0;

/// Provides a count of how many function iterations it takes for the given value to escape.
/// Returns ESCAPE_ITERS if the value doesn't escape in that many iterations.
pub fn escape_iters(val: &Complex) -> usize {
    let escape_size = Complex::with_val(53, (ESCAPE_SIZE, 0));

    let mut z = val.clone();
    for i in 0..ESCAPE_ITERS {
        if Complex::with_val(val.prec(), z.abs_ref()).real() > escape_size.real() { // hacky way to compare Euclidean distances, but it'll have to do for now.
            return i;
        }
        //z = Complex::with_val(z.prec(), z.mul_add_ref(&z, val)); // introduces only one rounding error.
        z.square_mut();
        z = z + val;
    }
    return ESCAPE_ITERS;
}

#[inline]
pub fn color(iters: usize) -> u8 {
    f64::ceil((LOG_ESCAPE - f64::ln(iters as f64)) / ESCAPE_FACTOR) as u8
}