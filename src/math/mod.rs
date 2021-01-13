pub mod number;
pub mod window;
pub mod color;

use rug::Complex;

pub const ESCAPE_ITERS: usize = 1000;
const ESCAPE_SIZE: usize = 100;

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