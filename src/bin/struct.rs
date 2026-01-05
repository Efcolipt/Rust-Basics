#![allow(unused)]

// Struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Point3D(f32, f32, f32);

struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    let p = Point { x: 1.0, y: 3.0 };
    println!("point.x {} point.y {}", p.x, p.y);

    let p = Point3D(1.0, 1.0, 1.0);
    println!("point.0 {}, point.1 {}, point.2 {}", p.0, p.1, p.2);

    let empty = Empty;

    let circle = Circle {
        center: Point { x: 1.0, y: 1.0 },
        radius: 1,
    };

    println!("{:?}", circle);

    // Shortcut
    let x = 1.0;
    let y = 1.0;
    let p = Point { x, y };

    // Copy fields
    let p0 = Point { x, y: 5.5 };
    let p1 = Point { x: 2.0, ..p0 };
    println!("{:?}", p1);

    // Update
    let mut p = Point { x: 0.0, y: 0.0 };

    p.x += 1.0;
    p.y += 1.0;

    println!("{:?}", p);
}
