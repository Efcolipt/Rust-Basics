#![allow(unused)]

// Concurrency
// - Thread
// - Channel
// - Async/await programming

use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let h1: JoinHandle<()> = thread::spawn(|| {
        for i in 0..5 {
            println!("h1: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    let h2: JoinHandle<()> = thread::spawn(|| {
        for i in 0..5 {
            println!("h2: {i}");
            thread::sleep(Duration::from_millis(100));
        }
    });

    let v = vec![1u32, 2u32, 3u32];

    let h3: JoinHandle<()> = thread::spawn(move || println!("{:?}", v));

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();

    let h: JoinHandle<u32> = thread::spawn(move || {
        return 1u32
    });

    match h.join() {
        Ok(val) => println!("{val}"),
        Err(err) => {}
    }
}
