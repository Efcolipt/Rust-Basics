#![allow(unused)]

use std::thread;

fn main() {
    let msg = "msg".to_string();

    let (v1, v2) = thread::scope(|scope| {
        let h1 = scope.spawn(|| {
            println!("h1: {msg}");

            return 1u32
        });

        let h2 = scope.spawn(|| {
            println!("h2: {msg}");

            return 2u32
        });

        (h1.join().unwrap(), h2.join().unwrap())
    });

    println!("{msg}");
    println!("{v1} {v2}");
}
