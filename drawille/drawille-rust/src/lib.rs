#[rustfmt::skip]
const PIXEL_MAP: [[u32; 2]; 4] = [[0x01, 0x08],
                                  [0x02, 0x10],
                                  [0x04, 0x20],
                                  [0x40, 0x80]];
const BASE_CHAR: u32 = 0x2800;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Canvas {
    pixels: Vec<Vec<u32>>,
}

impl Canvas {
    pub fn new() -> Self {
        let pixels = vec![vec![0; 10]; 10];
        Self { pixels }
    }

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
        self.pixels = vec![vec![0; 10]; 10];
    }

    pub fn set(&mut self, x: f64, y: f64) {
        let (row, col) = get_pos(x, y);
        if row >= self.pixels.len() {
            self.pad_row(row);
        }
        if col >= self.pixels[row].len() {
            self.pad_col(row, col);
        }
        let (x, y) = (normalize(x), normalize(y));
        let pixel = PIXEL_MAP[y % 4][x % 2];
        self.pixels[row][col] |= pixel;
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

fn get_pos(x: f64, y: f64) -> (usize, usize) {
    (y.round() as usize / 4, x.round() as usize / 2)
}

fn normalize(v: f64) -> usize {
    v.round() as usize
}
