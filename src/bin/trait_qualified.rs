#![allow(unused)]

trait Color {
    fn get(&self) -> String;
}

trait Rectangle {
    fn get(&self) -> (i32, i32, u32, u32);
}

struct Square {
    color: String,
    top: i32,
    left: i32,
    size: u32,
}

impl Color for Square {
    fn get(&self) -> String {
        self.color.clone()
    }
}

impl Rectangle for  Square {
    fn get(&self) -> (i32, i32, u32, u32) {
        (self.top, self.left, self.size, self.size)
    }
}

fn main() {
    // Fully qualified syntax
    let square = Square {
        color: "#fff".to_string(),
        left: 0,
        size: 0,
        top: 10,
    };

    // square.get()
    let color = Color::get(&square);
    let (x,y, width, height) = Rectangle::get(&square);

    println!("color: {color}");
    println!("x: {x}, y: {y}, width: {width}, height: {height}");
}
