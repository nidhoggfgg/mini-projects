use serde::Deserialize;

const GREEN: &'static str = "\x1B[32m";
const RED: &'static str = "\x1B[31m";
#[allow(dead_code)]
const RESET: &'static str = "\x1B[0m";

const SYMBOL_STICK: &'static str = "│";
const SYMBOL_CANDLE: &'static str = "┃";
const SYMBOL_TOP: &'static str = "╽";
const SYMBOL_BOTTOM: &'static str = "╿";
const SYMBOL_CANDLE_TOP: &'static str = "╻";
const SYMBOL_CANDLE_BOTTOM: &'static str = "╹";
const SYMBOL_STICK_TOP: &'static str = "╷";
const SYMBOL_STICK_BOTTOM: &'static str = "╵";
const SYMBOL_NOTHING: &'static str = " ";

pub enum PriceMove {
    Up,
    Down,
}

#[derive(Debug, Deserialize)]
pub struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

impl Candle {
    pub fn new(open_value: f64, max_value: f64, min_value: f64, end_value: f64) -> Self {
        Candle {
            open: open_value,
            high: max_value,
            low: min_value,
            close: end_value,
        }
    }

    #[inline]
    pub fn top_candle(&self) -> f64 {
        if self.open < self.close { self.close } else { self.open }
    }

    #[inline]
    pub fn bottom_candle(&self) -> f64 {
        if self.open > self.close { self.close } else { self.open }
    }

    fn price_move(&self) -> PriceMove {
        if self.open < self.close {
            PriceMove::Up
        } else {
            PriceMove::Down
        }
    }
}

pub struct CandleStickGraph<'a> {
    global_max: f64,
    global_min: f64,
    data: &'a [Candle],
    height: u64
}

impl<'a> CandleStickGraph<'a> {
    pub fn new(data: &'a [Candle], height: u64) -> Self {
        let global_max = Self::calc_global_max(data);
        let global_min = Self::calc_global_min(data);

        CandleStickGraph { global_max, global_min, data, height, }
    }

    pub fn draw(&self) {
        for y in (0..self.height).rev() {
            for candle in self.data {
                self.render_at(candle, y);
            }
            println!("");
        }
        println!("");
    }

    pub fn render_at(&self, candle: &Candle, y_axis: u64) {
        let y_axis = y_axis as f64;
        let top_stick = self.to_y(candle.high);
        let bottom_stick = self.to_y(candle.low);
        let top_candle = self.to_y(candle.top_candle());
        let bottom_candle = self.to_y(candle.bottom_candle());

        if top_candle.floor() <= y_axis && y_axis <= top_stick.ceil() {
            if top_candle - y_axis > 0.75 {
                self.print_with_color(candle, SYMBOL_CANDLE);
                return;
            }

            if top_candle - y_axis > 0.25 {
                if top_stick - y_axis > 0.75 {
                    self.print_with_color(candle, SYMBOL_TOP);
                    return;
                }
                self.print_with_color(candle, SYMBOL_CANDLE_TOP);
                return;
            }

            if top_stick - y_axis > 0.75 {
                self.print_with_color(candle, SYMBOL_STICK);
                return;
            }

            if top_stick - y_axis > 0.25 {
                self.print_with_color(candle, SYMBOL_STICK_TOP);
                return;
            }

            print!("{}", SYMBOL_NOTHING);
            return;
        }

        if bottom_candle.ceil() <= y_axis && y_axis <= top_candle.floor() {
            self.print_with_color(candle, SYMBOL_CANDLE);
            return;
        }

        if bottom_stick.floor() <= y_axis && y_axis <= bottom_candle.ceil() {
            if bottom_candle - y_axis < 0.25 {
                self.print_with_color(candle, SYMBOL_CANDLE);
                return;
            }

            if bottom_candle - y_axis < 0.75 {
                if bottom_stick - y_axis < 0.25 {
                    self.print_with_color(candle, SYMBOL_BOTTOM);
                    return;
                }
                self.print_with_color(candle, SYMBOL_CANDLE_BOTTOM);
                return;
            }

            if bottom_stick - y_axis < 0.25 {
                self.print_with_color(candle, SYMBOL_STICK);
                return;
            }

            if bottom_stick - y_axis < 0.75 {
                self.print_with_color(candle, SYMBOL_STICK_BOTTOM);
                return;
            }

            print!("{}", SYMBOL_NOTHING);
            return;
        }

        print!(" ");
    }

    fn print_with_color(&self, candle: &Candle, c: &'static str) {
        match candle.price_move() {
            PriceMove::Down => print!("{}{}", GREEN, c),
            PriceMove::Up => print!("{}{}", RED, c),
        }
    }

    #[inline]
    fn to_y(&self, value: f64) -> f64 {
        (value - self.global_min) / (self.global_max - self.global_min) * self.height as f64
    }

    fn calc_global_max(data: &'a [Candle]) -> f64 {
        let mut max_value = data[0].high;
        for v in data {
            if v.high > max_value {
                max_value = v.high;
            }
        }
        max_value
    }

    fn calc_global_min(data: &'a [Candle]) -> f64 {
        let mut min_value = data[0].low;
        for v in data {
            if v.low < min_value {
                min_value = v.low;
            }
        }
        min_value
    }
}
