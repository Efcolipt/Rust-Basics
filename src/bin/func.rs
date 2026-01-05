#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
    // implicit return(without return keyword)
    x + y
}

fn print() {
    println!("no output")
}

// Diverge - never return
fn forever() -> ! {
    loop {}
}

// Diverge - never return
fn crash() -> ! {
    panic!("crash")
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");

    // No output
    print();

    // Diverge
    crash();
}
