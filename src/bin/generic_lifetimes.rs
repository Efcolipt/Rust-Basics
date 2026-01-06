#![allow(unused)]

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let x = "a".to_string();

    {
        let y = "b".to_string();
        let z = longest_str(&x, &y);
        // y dropped here
        println!("z = {z}");
    };

    // x dropped here
}
