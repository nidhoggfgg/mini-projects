use donut_rust::{
    gen_color_char, get_term_size, init_param, listen_term_change, render_frame, Colored, PI,
};

static SLEEP_TIME: u64 = 32;

fn main() {
    let mut color = (128, 128, 128);
    let mut colored;

    let (mut width, mut height) = get_term_size();

    let (mut a, mut b) = (0.0_f64, 0.0_f64);

    let (mut k1, mut sample_theta, mut sample_phi, mut output, mut zbuffer) =
        init_param!(width, height);

    // clear screen
    println!("\x1B[2J");
    loop {
        if let Some((w, h)) = listen_term_change((width, height)) {
            width = w;
            height = h;
            (k1, sample_theta, sample_phi, output, zbuffer) = init_param!(w, h);
            println!("\x1B[2J");
        }
        colored = gen_color_char(&mut color);
        render_frame(
            (a, b),
            k1,
            (width, height),
            (PI / sample_theta, PI / sample_phi),
            &mut output,
            &mut zbuffer,
            &colored,
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

        // fill it rather than made a new vec is a little fast.
        output.fill(Colored::default());
        zbuffer.fill(0.0);

        // rotate whole graph
        a += 1.0 / 2.0_f64.powf(6.0) * PI;
        b += 1.0 / 2.0_f64.powf(7.0) * PI;
    }
}
