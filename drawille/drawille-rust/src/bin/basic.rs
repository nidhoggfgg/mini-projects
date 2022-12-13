use drawille_rust::Canvas;

fn main() {
    let mut c = Canvas::new();

    for x in 0..1800 {
        let x = x as f64;
        c.set(x / 10.0, 15.0 + x.to_radians().sin() * 10.0);
    }
    println!("{}", c.frame());
    c.clear();

    for x in (0..1800).step_by(10) {
        let x = x as f64;
        c.set(x / 10.0, 10.0 + x.to_radians().sin() * 10.0);
        c.set(x / 10.0, 10.0 + x.to_radians().cos() * 10.0);
    }
    println!("{}", c.frame());
    c.clear();

    for x in (0..3600).step_by(20) {
        let x = x as f64;
        c.set(x / 20.0, 4.0 + x.to_radians().sin() * 4.0);
    }
    println!("{}", c.frame());
}
