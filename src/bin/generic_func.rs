#![allow(unused)]

use std::cmp::PartialOrd;

fn swap<T1, T2>(t: (T1, T2)) -> (T2, T1) {
    (t.1, t.0)
}

fn max<T: PartialOrd>(s: &[T]) -> Option<&T> {
    if s.len() == 0 {
        return None;
    }

    let mut largest = &s[0];

    for item in s {
        if item > largest {
            largest = item
        }
    }

    Some(largest)
}

fn main() {
    let t: (i32, i32) = (1, 2);
    let s = swap(t);

    println!("{:?}", s);

    let t: (i32, u32) = (-1, 2);
    let s = swap(t);

    println!("{:?}", s);

    let nums = vec![1, 2, 3, 4, 5];
    let largest = max(&nums);
    println!("max: {:?}", largest);

    let nums = vec!['a', 'b', 'c', 'd', 'e'];
    let largest = max(&nums);
    println!("max: {:?}", largest);
}
