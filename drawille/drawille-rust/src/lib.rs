use terminal_size::{terminal_size, Height, Width};

#[rustfmt::skip]
const PIXEL_MAP: [[u32; 2]; 4] = [[0x01, 0x08],
                                  [0x02, 0x10],
                                  [0x04, 0x20],
                                  [0x40, 0x80]];
const BASE_CHAR: u32 = 0x2800;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Canvas {
    pixels: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new() -> Self {
        let (w, h) = Self::get_terminal_size();
        let pixels = vec![vec![0; w]; h];
        Self {
            pixels,
            width: w,
            height: h,
        }
    }

    pub fn frame(&mut self) -> String {
        self.pixels.shrink_to(0);
        self.pixels.iter_mut().for_each(|row| row.shrink_to(0));

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
        let (w, h) = Self::get_terminal_size();
        self.pixels = vec![vec![0; w]; h];
        self.width = w;
        self.height = h;
    }

    pub fn set(&mut self, x: f64, y: f64) {
        let (row, col) = get_pos(x, y);
        if row >= self.height || col >= self.width {
            panic!(
                "the canvas size is too small, it need ({}, {}), but only have ({}, {})",
                col, row, self.width, self.height
            );
        }
        let (x, y) = (normalize(x), normalize(y));

        let pixel = PIXEL_MAP[y % 4][x % 2];
        self.pixels[row][col] |= pixel;
    }

    pub fn unset(&mut self, x: f64, y: f64) {
        todo!()
    }

    pub fn toggle(&mut self, x: f64, y: f64) {
        todo!()
    }

    pub fn set_text(&mut self, x: f64, y: f64) {
        todo!()
    }

    fn get(&mut self, x: f64, y: f64) {
        todo!()
    }

    fn rows(&self) {
        todo!()
    }

    fn get_terminal_size() -> (usize, usize) {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            if w > 2 * h {
                (2 * h as usize, (h - 5) as usize)
            } else {
                (w as usize, (w / 2) as usize)
            }
        } else {
            (15, 20)
        }
    }
}

fn get_pos(x: f64, y: f64) -> (usize, usize) {
    (y.round() as usize / 4, x.round() as usize / 2)
}

fn normalize(v: f64) -> usize {
    v.round() as usize
}
