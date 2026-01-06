#![allow(unused)]

fn main() {
    // iter - borrows and returns a iterator that returns &T
    // into_iter - takes ownership and returns a iterator that may return T, &T or &mut T
    // iter_mut - returns &mut T
    let mut vals = vec![1, 2, 3];

    for v in vals.iter() {
        println!("{v}");
    }

    for v in vals.iter_mut() {
        *v += 1;
        println!("{v}");
    }

    for v in vals.iter_mut() {
        *v += 1;
        println!("{v}");
    }
}
