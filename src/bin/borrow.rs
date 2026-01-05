#![allow(unused)]

fn main() {
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s1;

    // Borrow - temporarily use a value without taking ownership
    // - Creates a reference (either mutable or immutable)
    // - Does not move ownership

    // Immutable borrow
    let s = String::from("rust");
    let s1 = &s;

    // any number of read-only access to a value
    let s2 = &s;
    let s3 = s2;

    // Mutable borrow
    let mut s = String::from("rust");
    let s0 = &mut s;
    let s1 = &mut s;

    s1.push_str("string");
    println!("s = {s}");

    // Cannot borrow immutable and mutable

    let mut s = String::from("rust");
    let s1 = &s;
    let s3 = &mut s;
    // println!("{s1} {s3}")

    // Reference must not outlive the value
    let s = String::from("rust");
    let s1 = &s;
    // {
    //     let s1 = s;
    // } // s1 is dropped

    // std::mem::drop(s);

    println!("{s1}")
}

// fn f(s: String) -> &String {
//     &s
// }