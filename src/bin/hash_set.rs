#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let mut set: HashSet<u32> = HashSet::new();

    let inserted = set.insert(1);
    println!("{inserted}");

    let contains = set.contains(&1);
    println!("contains1 {contains}");

    let contains = set.contains(&2);
    println!("contains2 {contains}");
}
