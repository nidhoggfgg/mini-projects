fn main() {
    const PI: f64 = std::f64::consts::PI;
    let theta_spacing = 0.07;
    let phi_spacing = 0.02;
    let screen_width = 100;
    let screen_height = 30;
    let r1 = 1.0;
    let r2 = 2.0;
    let k2 = 5.0;

    let mut cl = Vec::with_capacity(11);
    for i in 0..=11 {
        let c = ".,-~:;=!*#$@".chars().nth(i).unwrap();
        cl.push(format!("\x1B[38;5;{}m{}", 233 + 2 * i, c));
    }

    let mut a: f64 = 1.0;
    let mut b: f64 = 1.0;

    loop {
        a += 0.07;
        b += 0.03;

        let (sin_a, cos_a) = a.sin_cos();
        let (sin_b, cos_b) = b.sin_cos();
        let (mut output, mut zbuffer) = (
            vec![vec![" "; screen_width]; screen_height],
            vec![vec![0.0; screen_width]; screen_height],
        );
        let mut theta = 0.0;
        while theta < 2.0 * PI {
            let (sin_theta, cos_theta) = theta.sin_cos();
            let mut phi = 0.0;
            while phi < 2.0 * PI {
                let (sin_phi, cos_phi) = phi.sin_cos();
                let circlex = r1 * cos_theta + r2;
                let circley = r1 * sin_theta;
                let ooz = 1.0 / (sin_phi * circlex * sin_a + circley * cos_a + k2);
                let t1 = sin_phi * circlex * cos_a - sin_theta * sin_a;
                let t2 = cos_phi * circlex;
                let (x, y) = (
                    ((screen_width / 2) as f64 + 30.0 * ooz * (t2 * cos_b - t1 * sin_b)) as usize,
                    ((screen_height / 2) as f64 + 15.0 * ooz * (t2 * sin_b + t1 * cos_b)) as usize,
                );
                let n = 8.0
                    * ((sin_theta * sin_a - sin_phi * cos_theta * cos_a) * cos_b
                        - sin_phi * cos_theta * sin_a
                        - sin_theta * cos_a
                        - cos_phi * cos_theta * sin_b);
                if y < screen_height && x < screen_width && ooz > zbuffer[y][x] {
                    zbuffer[y][x] = ooz;
                    output[y][x] = cl.get(n as usize).unwrap_or(&cl[0]);
                }
                phi += phi_spacing;
            }
            theta += theta_spacing;
        }
        println!(
            "\x1B[H{}",
            output
                .iter()
                .map(|w| w.concat())
                .collect::<Vec::<String>>()
                .join("\n")
        );
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}
