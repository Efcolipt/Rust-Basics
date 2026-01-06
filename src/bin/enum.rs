#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    RGBA(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn main() {
    // Enum
    let color: Color = Color::Red;
    let color: Color = Color::RGBA(0, 0, 0, 1.0);
    let color: Color = Color::Hex("#fffff".to_string());
    let color: Color = Color::Hsl { h: 0, s: 0, l: 0 };
    // Attributes - Debug and PartialEq
    println!("{:?}", color);

    // PartialEq
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);

    // Option = Some(x) | None
    let x: Option<i32> = None;
    let x: Option<i32> = Some(-11);
    println!("option : {:?}", x);

    // Result = Ok(10) | Err("")
    let res: Result<u32, String> = Ok(5);
    let res: Result<u32, String> = Err("err".to_string());
    println!("res : {:?}", res);
}
