use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(2);
    }

    if let Ok(lines) = read_lines(&args[1]) {
        let max_width = get_max_width(&lines);
        add_head(max_width, ('┌', '┐', '─', '│'));
        add_frame(&lines, max_width, ('├', '┤', '└', '┘', '─', '│'));
    }
}

fn get_max_width(lines: &[String]) -> usize {
    let mut max_width: usize = 0;
    for line in lines {
        if line.len() > max_width {
            max_width = line.len();
        }
    }
    max_width + 3
}

fn add_head(max_width: usize, frame_char: (char, char, char, char)) {
    let (lt, rt, h, v) = frame_char;

    let mut _h = String::new();
    _h.push(h);

    println!("{}{}{}", lt, _h.repeat(max_width + 2), rt);
    println!("{0}{1:>width$}{0}", v, "- □ x ", width = max_width + 2);
}

fn add_frame(
    lines: &[String],
    max_width: usize,
    frame_char: (char, char, char, char, char, char),
) {
    let (lm, rm, lb, rb, h, v) = frame_char;

    let mut _h = String::new();
    _h.push(h);

    println!("{}{}{}", lm, _h.repeat(max_width + 2), rm);

    for line in lines {
        println!("{0} {1:width$} {0}", v, line, width = max_width);
    }

    println!(
        "{}{:width$}{}",
        lb,
        _h.repeat(max_width + 2),
        rb,
        width = max_width
    );
}

fn read_lines<P>(filename: &P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut lines: Vec<String> = Vec::new();
    let _lines = _read_lines(filename).unwrap();
    for line in _lines.flatten() {
        lines.push(line);
    }
    Ok(lines)
}

fn _read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
