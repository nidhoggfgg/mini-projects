use drawille_rust::Canvas;

fn main() {
    let mut c = Canvas::new();

    for x in 0..1800 {
        let x = x as f64;
        c.set(x / 10.0, x.to_radians().sin() * 10.0 + 15.0);
    }

    println!("{}", c.frame());
}
