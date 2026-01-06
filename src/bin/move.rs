#![allow(unused)]

fn main() {
    // Closures can capture variables by
    // - Borrow immutable reference &T
    // - Borrow mutable reference &mut T
    // - take ownership of value T

    // - Borrow immutable reference &T
    let s = "a".to_string();
    let f = || println!("borrow: {s}");
    f();
    println!("main: {s}");

    // - Borrow mutable reference &mut T
    let mut s = "a".to_string();
    let mut f = || s += "world";
    f();
    println!("main: {s}");

    // - take ownership of value T
    let s = "a".to_string();
    let f = move || {
        println!("{s}");
        s;
    };
    f();
    // Can't compile, s is dropped
    // f();
}
