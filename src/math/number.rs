use rug::Complex;
use rug::Float;
use super::window::WindowProperties;

/// The number of bits that are added to the mantissa of a number to get a little bit more precision, mostly for peace of mind.
/// Worth removing for performance increase as long as it doesn't change the way the fractal renders.
const ADDED_PRECISION: u32 = 2;

/// Gets the precision (mantissa size) required to do fractal calculations.
pub fn get_prec(props: &WindowProperties) -> u32 {
    let x_pixel_gap = Float::with_val(props.width_height.prec().0, props.width_height.real() / props.x_res);
    let y_pixel_gap = Float::with_val(props.width_height.prec().1, props.width_height.imag() / props.y_res);
    let pixel_gap = if x_pixel_gap < y_pixel_gap {x_pixel_gap} else {y_pixel_gap};

    // this line might need some reworking because of weird number behavior. Worth keeping an eye on.
    ADDED_PRECISION + 1 - Float::with_val(pixel_gap.prec(), pixel_gap * 2).log2().ceil().to_u32_saturating().unwrap() // force unwrap, should always work?
}

#[cfg(test)]
mod private_tests {

}