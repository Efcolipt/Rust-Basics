#![allow(unused)]

fn main() {
    // loop
    let mut i = 0;
    loop {
        println!("loop {i}");
        if i == 5 {
            break;
        }
        i += 1;
    }

    // while
    let mut i = 0;

    while i <= 3 {
        println!("while {i}");
        i += 1;
    }

    // for loop
    for i in 0..=5 {
        println!("for loop {i}");
    }

    // for loop array
    let arr = [1, 2, 3];
    for a in arr {
        println!("for array {a}");
    }

    // usize and range
    let n = arr.len();

    for i in 0..n {
        println!("for array {}", arr[i]);
    }

    // for loop vector
    let v = vec![1, 2, 3];

    // iter
    for x in v.iter() {
        println!("for vector {x}");
    }

    for x in v.iter() {
        println!("for vector {x}");
    }

    // Return value
    let mut i = 0;

    let z = loop {
        println!("loop {i}");
        if i == 3 {
            break 99;
        }
        i += 1;
    };

    println!("return loop {z}");

    // labels
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("{i}, {j}");
            if i == 1 && j == 2 {
                break 'outer; 
            }
        }
    }
}
