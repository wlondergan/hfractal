use image::Rgb;

const LOG_ESCAPE: f64 = 6.907_755_278_982_137; // maximum precision f64 of the log of 1000; adjust when changing ESCAPE_ITERS
const ESCAPE_FACTOR: f64 = LOG_ESCAPE / 255.0;

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
    let escape_factor = 255.0;
    escapes.iter().map(|x| (x.0.clone(), x.1.clone(), (max_val - correction_alg((x.2 - min) as f64)) / (max_val)))
    .map(|x| {
        let val = f64::ceil(x.2 * escape_factor) as u8;
        //println!("{}, {}", x.2, val);
        (x.0, x.1, Rgb([val, val, val]))
    }).collect()
}

#[inline]
fn correction_alg(i: f64) -> f64 {
    f64::ln(i)
}