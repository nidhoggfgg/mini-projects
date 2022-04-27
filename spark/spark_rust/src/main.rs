use std::{env, process};

fn help(s: &str) {
    println!("Usage: {} [-h|--help] [values, ...]", s);
    process::exit(0);
}

fn spark(values: &[String]) {
    let values: Vec<f64> = values
        .into_iter()
        .map(|v| v.parse().unwrap())
        .collect();

    let max_number = values.iter().fold(f64::NEG_INFINITY, |a, &b| b.max(a));
    let min_number = values.iter().fold(f64::INFINITY, |a, &b| b.min(a));

    let stick = if max_number - min_number < 0.000001 {
        vec!["▅", "▆"]
    } else {
        vec!["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"]
    };

    let mut f = (max_number - min_number) / (stick.len() - 1) as f64;
    f = f.max(1.0);

    for v in values {
        let tmp = ((v - min_number) / f) as usize;
        print!("{}", stick[tmp]);
    }
    println!()
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() == 1
        || argv[1] == "-h"
        || argv[1] == "--help" {
        help(&argv[0]);
    }

    spark(&argv[1..]);
}
