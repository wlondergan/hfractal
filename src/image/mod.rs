pub mod histogram;
pub mod naive;
pub mod oversample;
pub mod parallel;

pub use histogram::draw_image_histogram;
pub use naive::draw_image;
pub use parallel::draw_image_parallel;

/// Represents all of the different fractal types that this library can image-ify.
/// Might actually get used in the future.
pub enum FractalType {
    Mandelbrot,
}