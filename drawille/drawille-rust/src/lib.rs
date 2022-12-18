use std::cmp;

// http://www.alanwood.net/unicode/braille_patterns.html
// dots:
//    ,___,
//    |1 4|
//    |2 5|
//    |3 6|
//    |7 8|
//    `````
#[rustfmt::skip]
const PIXEL_MAP: [[u32; 2]; 4] = [[0x01, 0x08],
                                  [0x02, 0x10],
                                  [0x04, 0x20],
                                  [0x40, 0x80]];
// braille unicode characters starts at 0x2800
const BASE_CHAR: u32 = 0x2800;

#[inline]
fn get_pixel(x: f64, y: f64) -> u32 {
    let (x, y) = (normalize(x), normalize(y));
    PIXEL_MAP[y % 4][x % 2]
}

#[inline]
fn get_pos(x: f64, y: f64) -> (usize, usize) {
    (y.round() as usize / 4, x.round() as usize / 2)
}

#[inline]
fn normalize(v: f64) -> usize {
    v.round() as usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Canvas {
    pixels: Vec<Vec<u32>>,
}

impl Canvas {
    pub fn new() -> Self {
        let pixels = Vec::new();
        Self { pixels }
    }

    // I don't think safety needs to be explained here
    pub fn frame(&mut self) -> String {
        unsafe {
            self.pixels
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|p| char::from_u32_unchecked(BASE_CHAR + *p))
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
        }
    }

    pub fn clear(&mut self) {
        self.pixels = Vec::new();
    }

    pub fn set(&mut self, x: f64, y: f64) {
        let (row, col) = get_pos(x, y);
        self.pad_row_col(row, col);
        let pixel = get_pixel(x, y);
        self.pixels[row][col] |= pixel;
    }

    pub fn toggle(&mut self, x: f64, y: f64) {
        let (row, col) = get_pos(x, y);
        self.pad_row_col(row, col);
        let pixel = get_pixel(x, y);
        if self.pixels[row][col] & pixel != 0 {
            self.unset(x, y);
        } else {
            self.set(x, y);
        }
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let (x1, y1) = (normalize(x1), normalize(y1));
        let (x2, y2) = (normalize(x2), normalize(y2));
        let d = |v1, v2| {
            if v1 <= v2 {
                (v2 - v1, 1.0)
            } else {
                (v1 - v2, -1.0)
            }
        };

        let (xdiff, xdir) = d(x1, x2);
        let (ydiff, ydif) = d(y1, y2);
        let r = cmp::max(xdiff, ydiff);

        for i in 0..=r {
            let r = r as f64;
            let i = i as f64;
            let (xd, yd) = (xdiff as f64, ydiff as f64);
            let x = x1 as f64 + i * xd / r * xdir;
            let y = y1 as f64 + i * yd / r * ydif;
            self.set(x, y);
        }
    }

    fn unset(&mut self, x: f64, y: f64) {
        let (row, col) = get_pos(x, y);
        self.pad_row_col(row, col);
        let pixel = get_pixel(x, y);
        self.pixels[row][col] &= !(pixel as u8) as u32;
    }

    fn pad_row_col(&mut self, row: usize, col: usize) {
        if row >= self.pixels.len() {
            self.pad_row(row);
        }
        if col >= self.pixels[row].len() {
            self.pad_col(row, col);
        }
    }

    fn pad_row(&mut self, row: usize) {
        let pad_num = row - self.pixels.len() + 1;
        let mut pad = vec![vec![0; 10]; pad_num];
        self.pixels.append(&mut pad);
    }

    fn pad_col(&mut self, row: usize, col: usize) {
        let pad_num = col - self.pixels[row].len() + 1;
        let mut pad = vec![0; pad_num];
        self.pixels[row].append(&mut pad);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Turtle {
    pos_x: f64,
    pos_y: f64,
    rotation: f64,
    brush_on: bool,
    canvas: Canvas,
}

impl Turtle {
    pub fn new(pos_x: f64, pos_y: f64) -> Self {
        Self {
            pos_x,
            pos_y,
            rotation: 0.0,
            brush_on: true,
            canvas: Canvas::new(),
        }
    }

    pub fn frame(&mut self) -> String {
        self.canvas.frame()
    }

    pub fn up(&mut self) {
        self.brush_on = false;
    }

    pub fn down(&mut self) {
        self.brush_on = true;
    }

    pub fn forward(&mut self, step: f64) {
        let (sr, cr) = self.rotation.to_radians().sin_cos();
        let x = self.pos_x + cr * step;
        let y = self.pos_y + sr * step;
        let prev_brush = self.brush_on;
        self.brush_on = true;
        self.move_to(x, y);
        self.brush_on = prev_brush;
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        if self.brush_on {
            self.canvas.line(self.pos_x, self.pos_y, x, y);
        }

        self.pos_x = x;
        self.pos_y = y;
    }

    pub fn right(&mut self, angle: f64) {
        self.rotation += angle;
    }

    pub fn left(&mut self, angle: f64) {
        self.rotation -= angle;
    }

    pub fn back(&mut self, step: f64) {
        self.forward(-step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // see https://en.wikipedia.org/wiki/Rotation_matrix for more information
    pub fn rotate_xyz(&mut self, anlge_x: f64, anlge_y: f64, angle_z: f64) {
        // let (x, y, z) = (self.x, self.y, self.z);
        // let (sx, cx) = anlge_x.to_radians().sin_cos();
        // let (sy, cy) = anlge_y.to_radians().sin_cos();
        // let (sz, cz) = angle_z.to_radians().sin_cos();
        // let (t1, t2, t3) = (
        //     x * cy + y * sx * sy + z * cx * sy,
        //     y * cx - z * sx,
        //     y * sx + z * cx,
        // );
        // self.x = cz * t1 - sz * t2;
        // self.y = sz * t1 + cz * t2;
        // self.z = cz * t3 - sy * x;

        self.rotate_x(anlge_x);
        self.rotate_y(anlge_y);
        self.rotate_z(angle_z);
    }

    pub fn rotate_x(&mut self, angle: f64) {
        let (s, c) = angle.to_radians().sin_cos();
        let (y, z) = (self.y, self.z);
        self.y = y * c - z * s;
        self.z = y * s + z * c;
    }

    pub fn rotate_y(&mut self, angle: f64) {
        let (s, c) = angle.to_radians().sin_cos();
        let (x, z) = (self.x, self.z);
        self.x = x * c + z * s;
        self.z = -x * s + z * c;
    }

    pub fn rotate_z(&mut self, anlge: f64) {
        let (s, c) = anlge.to_radians().sin_cos();
        let (x, y) = (self.x, self.y);
        self.x = x * c - y * s;
        self.y = x * s + y * c;
    }
}
