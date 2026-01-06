#![allow(unused)]

use std::convert::{From, Into};

// pub trait From<T>: Sized {
//     fn from(value: T) -> Self;
// }

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

// (u32, u32) -> Self
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn main() {

}
