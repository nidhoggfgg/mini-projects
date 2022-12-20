use std::process;

use neodonut::{render, render_fast, Imp};

fn main() {
    let (imp, sample, threads) = deal_args();
    let color = (192, 192, 192);
    if threads == 1 {
        render(color, imp, 32, sample);
    } else {
        render_fast(color, imp, sample, threads);
    }
}

fn deal_args() -> (Imp, Option<(f64, f64)>, usize) {
    let mut imp = Imp::Color;
    let mut sample = None;
    let mut args = std::env::args();
    let mut threads = 1;
    let help = || {
        println!(
            r#"
    usage: neodonut [--improve <color|light|none>] [--sample <t> <p>] [--threads <num>]
        --improve    color: render donut with color and light
                        light: render donut only with light improvment
                        none: render donut just use ascii char without color and light improve.
        --sample     t: sample of the theta
                        p: sample of the phi
        --threads    num: use how many threads to calc and print (default 1)
        "#
        );
        process::exit(64);
    };
    while let Some(a) = args.next() {
        match a.as_str() {
            "--improve" => match args.next().unwrap_or_else(|| "".to_owned()).as_str() {
                "color" => {
                    imp = Imp::Color;
                }
                "light" => {
                    imp = Imp::Light;
                }
                "none" => {
                    imp = Imp::None;
                }
                _ => {
                    println!("bad argument, use: 'color', 'light' or 'none'!");
                    process::exit(64);
                }
            },
            "--sample" => {
                let print_usage = || {
                    println!(
                        "pls use: --sample <sample_theta> <sample_phi> (e.g. --sample 128.0 64.0)"
                    );
                    process::exit(64);
                };
                if let (Some(t), Some(p)) = (args.next(), args.next()) {
                    if let (Some(t), Some(p)) = (t.parse::<f64>().ok(), p.parse::<f64>().ok()) {
                        sample = Some((t, p));
                    } else {
                        print_usage();
                    }
                } else {
                    print_usage();
                }
            }
            "--threads" => {
                if let Ok(num) = args
                    .next()
                    .unwrap_or_else(|| "bad".to_owned())
                    .parse::<usize>()
                {
                    threads = num;
                } else {
                    println!("pls use: --threads <number> (e.g. --threads 8");
                    process::exit(64);
                }
            }
            "--help" | "help" | "-h" => help(),
            _ => continue, // ignore other args
        }
    }
    (imp, sample, threads)
}
