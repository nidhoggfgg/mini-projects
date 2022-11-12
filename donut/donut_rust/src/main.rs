fn main() {
    let scale = 1.0;
    let sample_theta = 2.0_f64.powf(7.0);
    let sample_phi = 2.0_f64.powf(7.0);
    let light_adv = true;

    const PI: f64 = std::f64::consts::PI;
    let theta_step = PI / sample_theta;
    let phi_step = PI / sample_phi;
    let swidth = (60.0 * scale) as usize;
    let sheight = (26.0 * scale) as usize;
    let r1 = 1.0;
    let r2 = 2.0;
    let k2 = 5.0;
    let k1 = 30.0 * scale;

    let mut lighted = Vec::with_capacity(12);
    for i in 0..=11 {
        let c = ".,-~:;=!*#$@".chars().nth(i).unwrap();
        let s = if light_adv {
            format!("\x1B[38;5;{}m{}", 233 + 2 * i, c)
        } else {
            format!("{}", c)
        };
        lighted.push(s);
    }

    let (mut a, mut b) = (0.0_f64, 0.0_f64);

    let (mut output, mut zbuffer) = (vec![" "; swidth * sheight], vec![0.0; swidth * sheight]);

    println!("\x1B[2J");
    loop {
        let (sina, cosa) = a.sin_cos();
        let (sinb, cosb) = b.sin_cos();

        let mut phi = 0.0;
        while phi < 2.0 * PI {
            let (sinp, cosp) = phi.sin_cos();
            let mut theta = 0.0;
            while theta < 2.0 * PI {
                let (sint, cost) = theta.sin_cos();

                let (x1, y1) = (r1 * cost + r2, r1 * sint);
                let (t1, t2) = (sinp * x1 * cosa - sint * sina, cosp * x1);

                let x = t2 * cosb - t1 * sinb;
                let y = t2 * sinb + t1 * cosb;
                let zd = 1.0 / (sinp * x1 * sina + y1 * cosa + k2);
                let (x, y) = (
                    ((swidth / 2) as f64 + k1 * zd * x) as usize,
                    ((sheight / 2) as f64 + k1 / 2.0 * zd * y) as usize,
                );

                let n = 8.0
                    * ((sint * sina - sinp * cost * cosa) * cosb
                        - sinp * cost * sina
                        - sint * cosa
                        - cosp * cost * sinb);
                let o = swidth * y + x;
                if y < sheight && x < swidth && zd > zbuffer[o] {
                    zbuffer[o] = zd;
                    output[o] = lighted.get(n as usize).unwrap_or(&lighted[0]);
                }
                theta += phi_step;
            }
            phi += theta_step;
        }

        print!("\x1B[?25l"); // hide the cursor
                             // print whole graph
        println!(
            "\x1B[H{}",
            output
                .chunks(swidth)
                .map(|l| l.concat())
                .collect::<Vec<String>>()
                .join("\n")
        );
        println!("\x1B[?25h"); // show the cursor

        // prepare for the next loop
        std::thread::sleep(std::time::Duration::from_millis(64));
        output.fill(" ");
        zbuffer.fill(0.0);

        // rotation of whole graph
        a += 1.0 / 2.0_f64.powf(6.0) * PI;
        b += 1.0 / 2.0_f64.powf(7.0) * PI;
    }
}
