#![allow(unused)]

fn take_string(s: String) {}
fn borrow_string(s: &String) {}
fn make_string() -> String {
    "".to_string()
}
fn mut_string(s: &mut String) {
    s.push_str("string");
}

fn borrow_str(s: &str) {}

fn main() {
    // String
    // pub struct String {
    // vec: Vec<u8>
    //}
    // - Owned
    // - Mutable, growable
    // - Allocated on the heap
    // - &String can be coerced into &str
    let s = String::from("rust");
    take_string(s);
    // println!("{s}")

    // mut String
    let mut s = String::from("rust");
    s += "!";

    // &String
    let s = String::from("rust");
    borrow_string(&s);
    println!("{s}");
    borrow_str(&s);

    // &mut String
    let mut s = String::from("rust");
    let s1: &mut String = &mut s;
    mut_string(&mut s);

    // str - string slice
    // - Dynamically sized type / unsized type
    // - Size of the type not known at compile time
    // - Compiler needs to know the size of each type
    // let a: str = "r";
    // let b: str = "r";

    // &str
    // - size known at compile time (pointer) 
    // - immutable borrow
    let s: &str = "hello";
    borrow_str(s);
    println!("{s}");

    // &mut str
    let mut s = String::from("rust");
    let r: &mut str = &mut s;


}
