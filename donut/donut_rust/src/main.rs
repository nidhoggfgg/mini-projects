fn main() {
    const PI: f64 = std::f64::consts::PI;
    let theta_spacing = 0.07;
    let phi_spacing = 0.02;
    let screen_width = 80;
    let screen_height = 26;
    let width = 30.0; // the width of the donut, please change the screen_width/height with change this
    let r1 = 1.0;
    let r2 = 2.0;
    let k2 = 5.0;

    // using the gray color of 8-bit color
    let mut colored_char = Vec::with_capacity(11);
    for i in 0..=11 {
        let c = ".,-~:;=!*#$@".chars().nth(i).unwrap();
        colored_char.push(format!("\x1B[38;5;{}m{}", 233 + 2 * i, c));
    }

    let mut a: f64 = 1.0;
    let mut b: f64 = 1.0;

    // use two dims vec is easy to understand, but a little comple for use
    // let (mut output, mut zbuffer) = (
    //     vec![vec![" "; screen_width]; screen_height],
    //     vec![vec![0.0; screen_width]; screen_height],
    // );

    let (mut output, mut zbuffer) = (
        vec![" "; screen_width * screen_height],
        vec![0.0; screen_width * screen_height],
    );

    loop {
        // control how to rotate, try some other value!
        a += 0.07;
        b += 0.04;

        output.fill(" ");
        zbuffer.fill(0.0);

        let (sin_a, cos_a) = a.sin_cos();
        let (sin_b, cos_b) = b.sin_cos();

        let mut theta = 0.0;
        while theta < 2.0 * PI {
            let (sin_theta, cos_theta) = theta.sin_cos();

            let mut phi = 0.0;
            while phi < 2.0 * PI {
                let (sin_phi, cos_phi) = phi.sin_cos();
                let (circlex, circley) = (r1 * cos_theta + r2, r1 * sin_theta);

                let ooz = 1.0 / (sin_phi * circlex * sin_a + circley * cos_a + k2);
                let (t1, t2) = (
                    sin_phi * circlex * cos_a - sin_theta * sin_a,
                    cos_phi * circlex,
                );
                // for ascii char, the font always set the width of "height" is the half of the width of "width"
                let (x, y) = (
                    ((screen_width / 2) as f64 + width * ooz * (t2 * cos_b - t1 * sin_b)) as usize,
                    ((screen_height / 2) as f64 + width / 2.0 * ooz * (t2 * sin_b + t1 * cos_b)) as usize,
                );
                let n = 8.0
                    * ((sin_theta * sin_a - sin_phi * cos_theta * cos_a) * cos_b
                        - sin_phi * cos_theta * sin_a
                        - sin_theta * cos_a
                        - cos_phi * cos_theta * sin_b);
                let o = 80 * y + x;
                if y < screen_height && x < screen_width && ooz > zbuffer[o] {
                    zbuffer[o] = ooz;
                    output[o] = colored_char.get(n as usize).unwrap_or(&colored_char[0]);
                }
                phi += phi_spacing;
            }
            theta += theta_spacing;
        }
        println!(
            "\x1B[H{}",
            output
                .chunks(80)
                .map(|l| l.concat())
                .collect::<Vec<String>>()
                .join("\n")
        );
        std::thread::sleep(std::time::Duration::from_millis(64));
    }
}
