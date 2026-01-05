#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();

    // Insert
    scores.insert("red".to_string(), 100);
    scores.insert("green".to_string(), 100);

    // Get
    let val: Option<&u32> = scores.get("red");
    println!("{:?}", val);

    let val: Option<&u32> = scores.get("yellow");

    println!("{:?}", val);

    scores.insert("green".to_string(), 200);

    let val: Option<&u32> = scores.get("green");

    println!("{:?}", val);

    // Upsert
    let v: &mut u32 = scores.entry("blue".to_string()).or_insert(0);

    *v += 200;

    let val: Option<&u32> = scores.get("blue");
    println!("{:?}", val);

    *scores.entry("blue".to_string()).or_insert(0) *= 2;

    let val: Option<&u32> = scores.get("blue");
    println!("{:?}", val);
}
