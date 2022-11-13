#![feature(test)]

extern crate test;

#[cfg(test)]
mod benches {
    use donut_rust::{gen_char_seq, init_matrix, init_param, render_frame, Colored, Imp, PI};
    use test::Bencher;
    const WIDTH: usize = 320;
    const HEIGHT: usize = 160;
    const IMP: Imp = Imp::Color;

    #[bench]
    fn bench_neodonut(bencher: &mut Bencher) {
        let mut color = (128, 128, 128);
        let colored = gen_char_seq(&mut color, IMP);

        let (k1, sample_theta, sample_phi) = init_param!(WIDTH, HEIGHT);
        let (mut output, mut zbuffer) = init_matrix(WIDTH, HEIGHT);
        let (mut a, mut b) = (0.0_f64, 0.0_f64);
        bencher.iter(|| {
            render_frame(
                (a, b),
                k1,
                (WIDTH, HEIGHT),
                (PI / sample_theta, PI / sample_phi),
                &mut output,
                &mut zbuffer,
                &colored,
            );
            output.fill(Colored::default());
            zbuffer.fill(0.0);
            a += 1.0 / 2.0_f64.powf(6.0) * PI;
            b += 1.0 / 2.0_f64.powf(7.0) * PI;
            test::black_box({
                output
                    .chunks(WIDTH)
                    .map(|l| l.iter().map(|c| format!("{}", c)).collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n");
            });
        });
    }

    #[bench]
    fn bench_neodonut_no_format(bencher: &mut Bencher) {
        let mut color = (128, 128, 128);
        let colored = gen_char_seq(&mut color, IMP);

        let (k1, sample_theta, sample_phi) = init_param!(WIDTH, HEIGHT);
        let (mut output, mut zbuffer) = init_matrix(WIDTH, HEIGHT);
        let (mut a, mut b) = (0.0_f64, 0.0_f64);
        bencher.iter(|| {
            render_frame(
                (a, b),
                k1,
                (WIDTH, HEIGHT),
                (PI / sample_theta, PI / sample_phi),
                &mut output,
                &mut zbuffer,
                &colored,
            );
            output.fill(Colored::default());
            zbuffer.fill(0.0);
            a += 1.0 / 2.0_f64.powf(6.0) * PI;
            b += 1.0 / 2.0_f64.powf(7.0) * PI;
        });
    }

    #[bench]
    fn bench_neodonut_new_matrix(bencher: &mut Bencher) {
        let mut color = (128, 128, 128);
        let colored = gen_char_seq(&mut color, IMP);

        let (mut a, mut b) = (0.0_f64, 0.0_f64);
        bencher.iter(|| {
            let (k1, sample_theta, sample_phi) = init_param!(WIDTH, HEIGHT);
            let (mut output, mut zbuffer) = init_matrix(WIDTH, HEIGHT);
            render_frame(
                (a, b),
                k1,
                (WIDTH, HEIGHT),
                (PI / sample_theta, PI / sample_phi),
                &mut output,
                &mut zbuffer,
                &colored,
            );
            a += 1.0 / 2.0_f64.powf(6.0) * PI;
            b += 1.0 / 2.0_f64.powf(7.0) * PI;
            test::black_box({
                output
                    .chunks(WIDTH)
                    .map(|l| l.iter().map(|c| format!("{}", c)).collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n");
            });
        });
    }
}
