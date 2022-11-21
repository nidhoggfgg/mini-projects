use rand::Rng;
use std::f64::consts::{PI, SQRT_2};
use std::sync::{mpsc, Arc, Mutex};
use std::{fmt, thread, time};
use terminal_size::{terminal_size, Height, Width};

// use channel for multi-threads. multiple producer, single consumer
// producer almost the render_frame(), will produce (String to print, width + height of the terminal)
// consumer check the terminal size and print the String
pub fn render_fast(color: (u8, u8, u8), imp: Imp, smp: Option<(f64, f64)>, thread_num: usize) {
    // sender and reciver
    let (tx, rx) = mpsc::channel();

    // value used in all thread
    let (a, b) = (Arc::new(Mutex::new(0.0)), Arc::new(Mutex::new(0.0)));
    let color = Arc::new(Mutex::new(color));

    for _ in 0..thread_num {
        // clone the ref
        let tx = tx.clone();
        let color = color.clone();
        let (a, b) = (a.clone(), b.clone());

        thread::spawn(move || {
            // init parameters
            let (mut width, mut height) = get_term_size();
            let (mut k1, mut sample) = init_param!(width, height);
            if let Some(s) = smp {
                sample = s;
            }

            loop {
                // deal with the terminal change
                if let Some((w, h)) = listen_term_change((width, height)) {
                    (k1, sample) = init_param!(w, h, width, sample);
                    println!("\x1B[2J");
                    height = h;
                    width = w;
                }

                let step = sample_to_step!(sample);
                let c = &mut color.lock().unwrap();
                let (mut a, mut b) = (a.lock().unwrap(), b.lock().unwrap());

                let char_seq = gen_char_seq(c, imp);
                let (s, wh) = render_frame((*a, *b), k1, (width, height), step, &char_seq);

                *a += PI / 64.0;
                *b += PI / 128.0;

                // send and sleep
                tx.send((s, wh)).unwrap();
                thread::sleep(time::Duration::from_millis(10));
            }
        });
    }

    println!("\x1B[2J");
    for (s, wh) in rx {
        let (w, h) = get_term_size();
        if w + h != wh {
            continue;
        }

        println!("\x1B[?25l{}\x1B[?25h", s);
    }
}

pub fn render(color: (u8, u8, u8), imp: Imp, sleep_time: u64, smp: Option<(f64, f64)>) {
    let mut color = color;
    let (mut width, mut height) = get_term_size();
    let (mut k1, mut sample) = init_param!(width, height);
    if let Some(s) = smp {
        sample = s;
    }

    let (mut a, mut b) = (0.0_f64, 0.0_f64);
    let mut char_seq = gen_char_seq(&mut color, imp);

    // clear screen
    println!("\x1B[2J");
    loop {
        if let Some((w, h)) = listen_term_change((width, height)) {
            (k1, sample) = init_param!(w, h, width, sample);
            println!("\x1B[2J");
            height = h;
            width = w;
        }
        let (s, _) = render_frame(
            (a, b),
            k1,
            (width, height),
            sample_to_step!(sample),
            &char_seq,
        );

        // print whole graph
        println!("\x1B[?25l{}\x1B[?25h", s);

        // prepare for the next loop
        thread::sleep(time::Duration::from_millis(sleep_time));
        char_seq = gen_char_seq(&mut color, imp);

        // rotate whole graph
        a += 1.0 / 2.0_f64.powf(6.0) * PI;
        b += 1.0 / 2.0_f64.powf(7.0) * PI;
    }
}

pub fn render_frame(
    r: (f64, f64),
    k1: f64,
    size: (usize, usize),
    step: (f64, f64),
    impc: &[Colored],
) -> (String, usize) {
    let (r1, r2, k2) = (1.0, 2.0, 5.0);
    let (width, height) = size;
    let (mut output, mut zbuffer) = init_matrix(width, height);
    let (ts, ps) = step;
    let ((sina, cosa), (sinb, cosb)) = (r.0.sin_cos(), r.1.sin_cos());
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
                    yi * sina - xi * cosa * sinp,
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
                output[o] = *impc.get(n as usize).unwrap_or_else(|| impc.last().unwrap());
            }
            theta += ts;
        }
        phi += ps;
    }

    (
        format!(
            "\x1B[H{}\n",
            output
                .chunks(width)
                .map(|l| l.iter().map(|c| format!("{}", c)).collect())
                .collect::<Vec<String>>()
                .join("\n")
        ),
        width + height,
    )
}

pub fn get_term_size() -> (usize, usize) {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        if w > 2 * h {
            (2 * h as usize, (h - 5) as usize)
        } else {
            (w as usize, (w / 2) as usize)
        }
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
    let color = (192, 192, 192);
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

#[macro_export]
macro_rules! sample_to_step {
    ($sample: expr) => {
        (PI / $sample.0, PI / $sample.1)
    };
}

// the constant 1.8, 2.0 and 6.0 is just what i think is appropriate not the best.
// but the performance of the terminal output has reached its limit,
// and optimizing these constants will not improve the fps. :)
// if the sample is small, render will be really fast and the dot will be very few.
#[macro_export]
macro_rules! init_param {
    ($w: expr, $h: expr) => {
        ($w as f64 / 1.8, ($h as f64 * 2.0, $h as f64 * 6.0))
    };
    ($w: expr, $h: expr, $ow: expr, $sample: expr) => {
        (
            $w as f64 / 1.8,
            (
                $sample.0 * $w as f64 / $ow as f64,
                $sample.1 * $w as f64 / $ow as f64,
            ),
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
