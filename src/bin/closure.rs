#![allow(unused)]

// Closure - anonymous function + capture variables in the environment

fn main() {
    // Diffs
    // - anonymous fn always assign to var
    // - anonymous fn can redeclare
    // - anonymous fn types inferred once
    // - anonymous fn capture variables in the environment
    fn add(x: u32, y: u32) -> u32 {
        x + y
    }

    // Cant declare twice
    // fn add(x: u32, y: u32) -> u32 {
    //     x + y
    // }

    let f = |x: u32, y: u32| -> u32 { x + y };
    let f = |x: u32, y: u32| -> u32 { x + y };

    let z = |x, y| x + y;
    println!("{}", z(1, 2));

    // capture variables in the environment
    let v = 1;
    let f = |x| x + v;

    let w: Vec<i32> = vec![1, 2, 3];
    // closure
    let w2: Vec<i32> = w.iter().map(|x| x + v).collect();

    f(1);
}
