#![allow(unused)]

// String and &str
fn main() {
    // String = vector of u8 (Vec<u8>) valid UTF-8
    // &str = slice of u8 (&[u8]) valid UTF-8

    // When to use String vs &str
    // String -> mutate or data needs to owned
    // &str -> read only

    // String

    let msg: String = String::from("hello rust");
    let len: usize = msg.len();

    println!("msg: {msg}");
    println!("len: {len}");

    // str - string slice
    // &str
    // - usually used str with reference (borrowed)
    // - immutable

    let msg: String = String::from("hello rust");
    let s: &str = &msg[0..];
    let len: usize = s.len();

    println!("msg: {msg}");
    println!("len: {len}");

    // String literal
    // - stored inside binary
    // - slice pointing to a specific part of the binary
    // - immutable because hard-coded inside binary
    let hello: &str = "Hello rust";

    let s: &str = r#"
        { 
            "a": 1,
            "b": { "c": 1 },
            "d": 3
        }
    "#;

    println!("{s}");

    // Deref coercion
    let msg: String = String::from("hello rust");
    let s: &String = &msg;

    // add &str to String
    let mut msg: String = "hello rust".to_string();

    msg += "!";
    println!("{msg}");

    let lang = "Rust";
    let emoji = "dd";

    let msg = format!("Hello {lang} {emoji}");
}
