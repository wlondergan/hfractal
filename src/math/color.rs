use image::Rgb;

const LOG_ESCAPE: f64 = 6.907_755_278_982_137; // maximum precision f64 of the log of 1000; adjust when changing ESCAPE_ITERS
const ESCAPE_FACTOR: f64 = LOG_ESCAPE / 255.0;
const COLOR_ROTATIONS: u32 = 20;
const FIXED_SATURATION: f64 = 0.6; // We fix the saturation value because it doesn't really have a place in our color model

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
    let max_val = correction_alg((max - min) as f64);
    escapes.iter().map(|x| (x.0.clone(), x.1.clone(), (max_val - correction_alg((x.2 - min) as f64)) / (max_val)))
    .map(|x| {
        let rgb = HSV::new(x.2 * COLOR_ROTATIONS as f64,
            FIXED_SATURATION, x.2).to_rgb();
        println!("{} {} {}", rgb.r, rgb.g, rgb.b);
        (x.0, x.1, Rgb([(rgb.r * 255.0) as u8, (rgb.g * 255.0) as u8, (rgb.b * 255.0) as u8]))
    }).collect()
}

#[inline]
fn correction_alg(i: f64) -> f64 {
    //f64::ln(i)
    f64::ln(i)
}

