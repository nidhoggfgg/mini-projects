use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(2);
    }

    let lines = read_lines(&args[1]);
    let max_width = get_max_width(&lines);
    add_head(max_width, ('┌', '┐', '─', '│'), "- □ x");
    add_body(&lines, max_width, ('├', '┤', '─', '│'));
    add_tail(max_width, ('└', '┘', '─'))
}

fn get_max_width(lines: &[String]) -> usize {
    let max_width = lines.iter().map(|s| s.len()).max().unwrap_or(0);
    max_width + 3
}

fn add_head(max_width: usize, frame_char: (char, char, char, char), icons: &str) {
    let (lt, rt, h, v) = frame_char;
    let h = format!("{h}");
    println!("{lt}{}{rt}", h.repeat(max_width + 2));
    println!("{v} {icons:>width$} {v}", width = max_width);
}

fn add_body(lines: &[String], max_width: usize, frame_char: (char, char, char, char)) {
    let (lm, rm, h, v) = frame_char;
    let h = format!("{h}");
    println!("{lm}{}{rm}", h.repeat(max_width + 2));
    for line in lines {
        println!("{v} {line:width$} {v}", width = max_width);
    }
}

fn add_tail(max_width: usize, frame_char: (char, char, char)) {
    let (lb, rb, h) = frame_char;
    let h = format!("{h}");
    println!("{lb}{}{rb}", h.repeat(max_width + 2));
}

fn read_lines<P>(filename: &P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("can't open file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("can't parse line")).collect()
}
