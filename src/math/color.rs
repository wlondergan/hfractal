use image::Rgb;

const LOG_ESCAPE: f64 = 6.907_755_278_982_137; // maximum precision f64 of the log of 1000; adjust when changing ESCAPE_ITERS
const ESCAPE_FACTOR: f64 = LOG_ESCAPE / 255.0;
const COLOR_ROTATIONS: u32 = 4;
const FIXED_SATURATION: u8 = (255.0 * 0.6) as u8; // We fix the saturation value because it doesn't really have a place in our color model

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
    let escape_factor = 16_777_215.0;
    escapes.iter().map(|x| (x.0.clone(), x.1.clone(), (max_val - correction_alg((x.2 - min) as f64)) / (max_val)))
    .map(|x| {
        let color = f64::ceil(x.2 * escape_factor) as usize;
        let h = (f64::ceil(x.2 * COLOR_ROTATIONS as f64 * 255.0) as u32 % COLOR_ROTATIONS) as u8;
        let s = FIXED_SATURATION;
        let v = (f64::ceil(x.2 * 255.0)) as u8;
    }).collect()
}

#[inline]
fn correction_alg(i: f64) -> f64 {
    f64::ln(i)
}

pub struct RGB {
    r: f64,
    g: f64,
    b: f64
}

struct HSV {
    h: f64,
    s: f64,
    v: f64
}

impl HSV {
    fn new(h: f64, s: f64, v: f64) -> HSV {
        HSV {
            h: h * 360.0, 
            s: s, 
            v: v,
        }
    }

    fn to_rgb(&self) -> RGB {
        let mut hh = 0.0;
        let mut p = 0.0;
        let mut q = 0.0;
        let mut t = 0.0;
        let mut ff = 0.0;
        let i: i64 = 0;
        let mut out = RGB {
            r: 0.0, g: 0.0, b: 0.0
        };

        if self.s <= 0.0 {       // < is bogus, just shuts up warnings
            out.r = self.v;
            out.g = self.v;
            out.b = self.v;
            return out;
        }
        hh = self.h;
        if hh >= 360.0 {
           hh = 0.0 
        };
        hh /= 60.0;
        i = hh as i64;
        ff = hh - i as f64;
        p = self.v * (1.0 - self.s);
        q = self.v * (1.0 - (self.s * ff));
        t = self.v * (1.0 - (self.s * (1.0 - ff)));

        match i {
            0 => {
                out.r = self.v;
                out.g = t;
                out.b = p;
            }
            1 => {
                out.r = q;
                out.g = self.v;
                out.b = p;
            }
            2 => {
                out.r = p;
                out.g = self.v;
                out.b = t;
            }
            3 => {
                out.r = p;
                out.g = q;
                out.b = self.v;
            }
            4 => {
                out.r = t;
                out.g = p;
                out.b = self.v;
            }
            _ => {
                out.r = self.v;
                out.g = p;
                out.b = q;
            }
        }
        out   
    }
}

