#![allow(unused)]

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p: Point<u32> = Point::new(3, 2);
    p.move_to(3, 2);

    println!("{:?}", p)
}
