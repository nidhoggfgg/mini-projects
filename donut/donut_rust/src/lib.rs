use rand::{distributions::weighted, Rng};
use std::f64::consts::SQRT_2;
use std::fmt;
use terminal_size::{terminal_size, Height, Width};
pub const PI: f64 = std::f64::consts::PI;

pub fn render_frame(
    r: (f64, f64),
    k1: f64,
    size: (usize, usize),
    step: (f64, f64),
    output: &mut [Colored],
    zbuffer: &mut [f64],
    impc: &[Colored],
) {
    let r1 = 1.0;
    let r2 = 2.0;
    let k2 = 5.0;
    let (sina, cosa) = r.0.sin_cos();
    let (sinb, cosb) = r.1.sin_cos();
    let (width, height) = size;
    let (ts, ps) = step;
    let (wd2, hd2, k1d2) = ((width / 2) as f64, (height / 2) as f64, k1 / 2.0);
    let mut phi = 0.0;
    while phi < 2.0 * PI {
        let (sinp, cosp) = phi.sin_cos();

        let mut theta = 0.0;
        while theta < 2.0 * PI {
            let (sint, cost) = theta.sin_cos();

            // this closure rotate a vector (ry -> rx -> rz), M(phi, a, b).
            // see https://en.wikipedia.org/wiki/Rotation_matrix for more information.
            // note: this closure dont use z, just because in this program z always 0.
            let rotate_yxz = |xi, yi| {
                let t1 = xi * sina * sinp + yi * cosa;
                let t2 = xi * cosp;
                (
                    t1 * cosb - t2 * sinb,
                    t1 * sinb + t2 * cosb,
                    yi * sina - xi * cosa * sinp
                )
            };

            // first, make a point on a circle.
            let (x1, y1) = (r1 * cost + r2, r1 * sint);

            // second, rotate it to right position.
            let (x, y, z) = rotate_yxz(x1, y1);
            let zd = 1.0 / (z + k2);

            // third, map the point to the right place on monitor.
            let (x, y) = ((wd2 + k1 * zd * x) as usize, (hd2 - k1d2 * zd * y) as usize);

            // calculating light, assuming that the light source is above.
            // (cost, sint, 0) are the normal vectors of the circle.
            let t = rotate_yxz(cost, sint);
            let n = 11.0 * (t.1 - t.2) / SQRT_2;

            let o = width * y + x;
            if y < height && x < width && zd > zbuffer[o] {
                zbuffer[o] = zd;
                output[o] = *impc.get(n as usize).unwrap_or(&impc[0]);
            }
            theta += ts;
        }
        phi += ps;
    }
}

pub fn get_term_size() -> (usize, usize) {
    let size = terminal_size();
    if let Some((Width(_), Height(h))) = size {
        // dont use the real width, the torus is more likely a square.
        // note: w almost double h just because the width of a char almost equal to double its height
        (2 * h as usize, h as usize - 5)
    } else {
        (60, 30)
    }
}

pub fn listen_term_change(old: (usize, usize)) -> Option<(usize, usize)> {
    let now = get_term_size();
    if old.0 == now.0 && old.1 == now.1 {
        return None;
    }
    Some(now)
}

pub fn gen_char_seq(color: &mut (u8, u8, u8), imp: Imp) -> Vec<Colored> {
    match imp {
        Imp::Color => gen_color_char(color),
        Imp::Light => gen_light_char(),
        Imp::None => gen_none_char(),
    }
}

fn gen_none_char() -> Vec<Colored> {
    let color = (128, 128, 128);
    let mut colored = Vec::with_capacity(12);
    let chars = ".,-~:;=!*#$@".chars();
    for c in chars {
        colored.push(Colored::new(color, c));
    }
    colored
}

fn gen_light_char() -> Vec<Colored> {
    let color = (128, 128, 128);
    let mut colored = Vec::with_capacity(12);
    let lc = |c, l| (c as f64 * l) as u8;
    let light_levels = [
        0.50, 0.6, 0.7, 0.8, 0.9, 1.00, 1.00, 1.1, 1.2, 1.3, 1.4, 1.5,
    ];
    let chars = ".,-~:;=!*#$@".chars();
    for (l, c) in std::iter::zip(light_levels, chars) {
        colored.push(Colored::new(
            (lc(color.0, l), lc(color.1, l), lc(color.1, l)),
            c,
        ))
    }
    colored
}

fn gen_color_char(color: &mut (u8, u8, u8)) -> Vec<Colored> {
    let mut rng = rand::thread_rng();

    // random add or minus, this will make the color more randomly
    let mut add_or_min = |a| {
        if a >= 240 {
            a - rng.gen_range(0..=16)
        } else if a <= 16 {
            a + rng.gen_range(0..=16)
        } else {
            a + rng.gen_range(0..=16) - 8
        }
    };

    color.0 = add_or_min(color.0);
    color.1 = add_or_min(color.1);
    color.2 = add_or_min(color.2);
    let (r, g, b) = *color;

    // this just make a color with light (e.g. A: (100, 100, 100) -> B: (110, 110, 110), B just looks lighter than A)
    let lc = |c, l| (c as f64 * l) as u8;
    let mut colored = Vec::with_capacity(12);
    let light_levels = [
        0.50, 0.6, 0.7, 0.8, 0.9, 1.00, 1.00, 1.1, 1.2, 1.3, 1.4, 1.5,
    ];
    let chars = ".,-~:;=!*#$@".chars();
    for (l, c) in std::iter::zip(light_levels, chars) {
        colored.push(Colored::new((lc(r, l), lc(g, l), lc(b, l)), c))
    }
    colored
}

pub fn init_matrix(w: usize, h: usize) -> (Vec<Colored>, Vec<f64>) {
    (
        vec![Colored::default(); w * h], // output
        vec![0.0; w * h],                // zbuffer
    )
}

// the constant 1.8, 2.0 and 6.0 is just what i think is appropriate not the best.
// but the performance of the terminal output has reached its limit,
// and optimizing these constants will not improve the fps. :)
// if the sample is small, render will be really fast and the dot will be very few.
#[macro_export]
macro_rules! init_param {
    ($w: expr, $h: expr) => {
        ($w as f64 / 1.8, $h as f64 * 2.0, $h as f64 * 6.0)
    };
    ($w: expr, $h: expr, $ow: expr, $sample: expr) => {
        (
            $w as f64 / 1.8,
            $sample.0 * $w as f64 / $ow as f64,
            $sample.1 * $w as f64 / $ow as f64,
        )
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Colored {
    color: (u8, u8, u8),
    ch: char,
}

impl Colored {
    pub fn new(color: (u8, u8, u8), ch: char) -> Colored {
        Colored { color, ch }
    }
}

impl fmt::Display for Colored {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.color;
        write!(f, "\x1B[38;2;{};{};{}m{}", r, g, b, self.ch)
    }
}

impl Default for Colored {
    fn default() -> Self {
        Self {
            color: (0, 0, 0),
            ch: ' ',
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Imp {
    Color,
    Light,
    None,
}
