use std::fmt::Display;
use std::{cmp, error};

#[allow(dead_code)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Align {
    pub fn new() -> Self {
        Align::Center
    }
}

pub enum Segment {
    None,
    OnlyHeader,
    Full,
}

impl Segment {
    pub fn new() -> Self {
        Segment::OnlyHeader
    }
}

#[rustfmt::skip]
pub struct DecChar {
    lt: &'static str, rt: &'static str, lb: &'static str, rb: &'static str,
    tm: &'static str, bm: &'static str, lm: &'static str, rm: &'static str,
    th: &'static str, bh: &'static str,
    lv: &'static str, rv: &'static str,
    sh: &'static str, shv: &'static str,
    sep: &'static str
}

#[rustfmt::skip]
impl DecChar {
    pub fn new() -> Self {
        DecChar {
            lt: "╭", rt: "╖",
            lb: "╰", rb: "╝",
            tm: "─", bm: "═",
            lm: "├", rm: "╢",
            th: "─", bh: "═",
            lv: "│", rv: "║",
            sh: "┄", shv:"┄",
            sep: " ",
        }
    }
}

#[allow(dead_code)]
pub struct Table {
    align: Align,
    seg: Segment,
    rows: Vec<Vec<String>>,
    header: Vec<String>,
    max_widths: Vec<usize>,
    made_seg: String,
    dec_char: DecChar,
    is_dec: bool,
}

impl Table {
    pub fn new(is_dec: bool) -> Self {
        Table {
            align: Align::new(),
            seg: Segment::new(),
            rows: Vec::new(),
            header: Vec::new(),
            max_widths: Vec::new(),
            made_seg: String::new(),
            dec_char: DecChar::new(),
            is_dec,
        }
    }

    #[inline]
    pub fn set_header(&mut self, header: Vec<String>) {
        self.header = header;
    }

    #[inline]
    pub fn set_rows(&mut self, rows: Vec<Vec<String>>) -> Result<(), PushErr> {
        for row in rows {
            self.push(row)?;
        }
        Ok(())
    }

    pub fn push(&mut self, row: Vec<String>) -> Result<(), PushErr> {
        if row.len() != self.header.len() {
            return Err(PushErr);
        }
        self.re_count_width(&row);
        self.rows.push(row);
        Ok(())
    }

    pub fn push_col(&mut self, mut col: Vec<String>, header: String) -> Result<(), PushErr> {
        if col.len() != self.rows.len() {
            return Err(PushErr);
        }

        let mut i = col.len();
        loop {
            i -= 1;
            self.rows[i].push(col.pop().unwrap());
            if i == 0 {
                break;
            }
        }
        self.header.push(header);

        Ok(())
    }

    pub fn make_table(&mut self) -> Vec<String> {
        self.make_seg();
        let mut result: Vec<String> = Vec::new();
        self.add_header(&mut result);
        self.add_body(&mut result);
        self.add_footer(&mut result);

        return result;
    }

    fn add_header(&self, lines: &mut Vec<String>) {
        if self.is_dec {
            let top = self
                .max_widths
                .iter()
                .map(|a| self.dec_char.th.repeat(*a))
                .collect::<Vec<String>>()
                .join(&self.dec_char.tm);
            lines.push(format!("{}{}{}", self.dec_char.lt, top, self.dec_char.rt))
        }
        lines.push(format!("{}{}{}", self.dec_char.lv, self.make_line(&self.header), self.dec_char.rv));
        self.add_seg(lines, true);
    }

    fn add_body(&mut self, lines: &mut Vec<String>) {
        let (prefix, suffix) = if self.is_dec {
            (self.dec_char.lv, self.dec_char.rv)
        } else {
            ("", "")
        };

        for row in &self.rows {
            lines.push(format!("{}{}{}", prefix, self.make_line(row), suffix));
            self.add_seg(lines, false);
        }

        match self.seg {
            Segment::Full => {
                lines.pop();
            }
            _ => (),
        }
    }

    fn add_footer(&mut self, lines: &mut Vec<String>) {
        if !self.is_dec {
            return;
        }

        let footer = self
            .max_widths
            .iter()
            .map(|a| self.dec_char.bh.repeat(*a))
            .collect::<Vec<String>>()
            .join(&self.dec_char.bm);
        lines.push(format!("{}{}{}", self.dec_char.lb, footer, self.dec_char.rb))
    }

    fn make_line(&self, row: &Vec<String>) -> String {
        let tmp = self.max_widths.iter().zip(row);
        let line: Vec<String> = match self.align {
            Align::Left => tmp.map(|(a, b)| format!("{x:<y$}", x = b, y = a)).collect(),
            Align::Center => tmp.map(|(a, b)| format!("{x:^y$}", x = b, y = a)).collect(),
            Align::Right => tmp.map(|(a, b)| format!("{x:>y$}", x = b, y = a)).collect(),
        };

        line.join(&self.dec_char.sep)
    }

    fn make_seg(&mut self) {
        let segs: Vec<String> = self
            .max_widths
            .iter()
            .map(|a| self.dec_char.sh.repeat(*a))
            .collect();
        self.made_seg = segs.join(&self.dec_char.shv);
    }

    fn add_seg(&self, lines: &mut Vec<String>, is_header: bool) {
        let (prefix, suffix) = if self.is_dec {
            (self.dec_char.lm, self.dec_char.rm)
        } else {
            ("", "")
        };
        if is_header {
            match self.seg {
                Segment::OnlyHeader => {
                    lines.push(format!("{}{}{}", prefix, self.made_seg.clone(), suffix))
                }
                _ => return,
            }
        }

        match self.seg {
            Segment::Full => lines.push(self.made_seg.clone()),
            _ => return,
        }
    }

    fn re_count_width(&mut self, row: &Vec<String>) {
        if self.max_widths.is_empty() {
            for v in row {
                self.max_widths.push(v.len() + 3);
            }

            return;
        }

        let re_counted = self
            .max_widths
            .iter()
            .zip(row)
            .map(|(a, b)| cmp::max(*a - 3, b.len() + 3))
            .collect();
        self.max_widths = re_counted;
    }
}

#[derive(Debug)]
pub struct PushErr;

impl Display for PushErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "can't push line into the table!")
    }
}

impl error::Error for PushErr {}
