use std::process;

use donut_rust::{
    gen_char_seq, get_term_size, init_matrix, init_param, listen_term_change, render_frame,
    Colored, Imp, PI,
};

static SLEEP_TIME: u64 = 32;

fn main() {
    let mut color = (128, 128, 128);
    let (mut width, mut height) = get_term_size();
    let (mut k1, mut sample_theta, mut sample_phi) = init_param!(width, height);
    let (imp, sample) = deal_args();
    if let Some(s) = sample {
        sample_theta = s.0;
        sample_phi = s.1;
    }

    let (mut a, mut b) = (0.0_f64, 0.0_f64);
    let (mut output, mut zbuffer) = init_matrix(width, height);
    let mut char_seq = gen_char_seq(&mut color, imp);
    // clear screen
    println!("\x1B[2J");
    loop {
        if let Some((w, h)) = listen_term_change((width, height)) {
            (k1, sample_theta, sample_phi) = init_param!(w, h, width, (sample_theta, sample_phi));
            (output, zbuffer) = init_matrix(w, h);
            println!("\x1B[2J");
            height = h;
            width = w;
        }
        render_frame(
            (a, b),
            k1,
            (width, height),
            (PI / sample_theta, PI / sample_phi),
            &mut output,
            &mut zbuffer,
            &char_seq,
        );

        // hide the cursor, when fps is low, it will useful
        println!("\x1B[?25l");
        // print whole graph
        println!(
            "\x1B[H{}",
            output
                .chunks(width)
                .map(|l| l.iter().map(|c| format!("{}", c)).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        );
        // show the cursor
        println!("\x1B[?25h");

        // prepare for the next loop
        std::thread::sleep(std::time::Duration::from_millis(SLEEP_TIME));
        char_seq = gen_char_seq(&mut color, imp);
        // fill it rather than made a new vec is a little fast.
        output.fill(Colored::default());
        zbuffer.fill(0.0);

        // rotate whole graph
        a += 1.0 / 2.0_f64.powf(6.0) * PI;
        b += 1.0 / 2.0_f64.powf(7.0) * PI;
    }
}

fn deal_args() -> (Imp, Option<(f64, f64)>) {
    let mut imp = Imp::Color;
    let mut sample = None;
    let mut args = std::env::args();
    let help = || {
        println!(
            r#"
    usage: neodonut [--improve <color|light|none>] [--sample <t> <p>]
        --improve    color: render donut with color and light
                        light: render donut only with light improvment
                        none: render donut just use ascii char without color and light improve.
        --sample     t: sample of the theta
                        p: sample of the phi
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
            "--help" | "help" | "-h" => help(),
            _ => continue, // ignore other args
        }
    }
    (imp, sample)
}
