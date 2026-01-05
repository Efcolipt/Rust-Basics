#![allow(unused)]

fn modify(s: &mut String) {
    *s += "?";
}
fn main() {
    // Deref
    let mut s = String::from("rust");
    let s1 = &mut s;
    *s1 += "?";

    println!("{s}");

    let mut s = String::from("rust");
    modify(&mut s);
    s += "1";
    println!("{s}");

    // Deref coercion
    // Automatically dereferenced in some situations
    let x = 1;
    let y = &x;
    let z = &x;
    let w = y + z;

    println!("{w}")
}
