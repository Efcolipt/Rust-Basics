#![allow(unused)]

// Monomorphization
struct Point<T> {
    x: T,
    y: T,
}

struct Point_u32 {
    x: u32,
    y: u32,
}

struct Point_i32 {
    x: i32,
    y: i32,
}

fn get_x<T>(p: Point<T>) -> T {
    p.x
}

fn get_x_u32(p: Point_u32) -> u32 {
    p.x
}

fn get_x_i32(p: Point_i32) -> i32 {
    p.x
}

fn main() {
    // Monomorphization increases compilation time and size of the binary

    let p0: Point<i32> = Point { x: 0, y: 0 };
    let p1: Point<u32> = Point { x: 0, y: 0 };

    get_x(p0);
    get_x(p1);

    // Code above at compile time will be

    let p0: Point_i32 = Point_i32 { x: 0, y: 0 };
    let p1: Point_u32 = Point_u32 { x: 0, y: 0 };

    get_x_i32(p0);
    get_x_u32(p1);
}
