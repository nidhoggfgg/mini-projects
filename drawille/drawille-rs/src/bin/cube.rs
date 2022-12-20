use drawille::{Canvas, Point3D};

// generate the vertices(6) of cube and sides(12) of cube
// the sides contain the index of the vertice
fn gen_cube(side_len: f64) -> ([Point3D; 8], [(usize, usize); 12]) {
    let a = [
        (-1, -1, -1),
        (-1, -1, 1),
        (-1, 1, -1),
        (1, -1, -1),
        (-1, 1, 1),
        (1, -1, 1),
        (1, 1, -1),
        (1, 1, 1),
    ];
    let mut vertices = Vec::new();
    for i in a {
        let x = side_len / 2.0 * i.0 as f64;
        let y = side_len / 2.0 * i.1 as f64;
        let z = side_len / 2.0 * i.2 as f64;
        vertices.push(Point3D::new(x, y, z));
    }
    (
        vertices.try_into().unwrap(),
        [
            (0, 1),
            (1, 4),
            (4, 2),
            (2, 0),
            (3, 5),
            (5, 7),
            (7, 6),
            (6, 3),
            (1, 5),
            (4, 7),
            (2, 6),
            (0, 3),
        ],
    )
}

fn gen_rotate(k: i32) -> (f64, f64, f64) {
    if k % 2 == 0 {
        (1.0, 2.0, 4.0)
    } else {
        (2.0, 3.0, 5.0)
    }
}

fn main() {
    let side_len = 30.0;
    let (mut vertices, sides) = gen_cube(side_len);
    let mut k = 0;
    let mut c = Canvas::new();
    // hide the cursor
    println!("\x1B[?25l");
    loop {
        let (rx, ry, rz) = gen_rotate(k);
        // clean screen & move cursor to (0, 0)
        println!("\x1B[2J\x1B[H");
        for v in &mut vertices {
            v.rotate_xyz(rx, ry, rz);
        }

        for s in sides {
            let (v1, v2) = (vertices[s.0], vertices[s.1]);
            let (x1, y1) = (side_len + v1.x, side_len + v1.y);
            let (x2, y2) = (side_len + v2.x, side_len + v2.y);
            c.line(x1, y1, x2, y2);
        }

        println!("{}", c.frame());
        c.clear();
        std::thread::sleep(std::time::Duration::from_millis(32));
        k += 1;
    }
}
