#![allow(unused)]

use std::collections::{HashMap, HashSet};

fn main() {
    // Iterator adaptor
    // map
    let vals: Vec<u32> = vec![1, 2, 3];
    let mut data: Vec<u32> = Vec::new();
    // let mut data: HashSet<u32> = HashSet::new();

    for v in vals {
        data.push(v + 2);
    }

    let vals: Vec<u32> = vec![1, 2, 3];
    let data: Vec<u32> = vals.iter().map(|x| 2 * x).collect();
    // let data: HashSet<u32> = vals.iter().map(|x| 2 * x).collect();
    println!("map{:?}", data);

    // filter
    let vals: Vec<u32> = vec![1, 2, 3];
    let mut data: Vec<u32> = Vec::new();

    for v in vals {
        if v <= 2 {
            data.push(v + 2);
        }
    }

    let vals: Vec<u32> = vec![1, 2, 3];
    let data: Vec<u32> = vals
        .into_iter()
        .filter(|x| *x <= 2)
        .map(|x| 2 * x)
        .collect();
    println!("filter map {:?}", data);

    // zip
    let vals: Vec<String> = vec!["a", "b", "c"].iter().map(|x| x.to_string()).collect();
    let keys: Vec<u32> = vec![1, 2, 3];
    // let zipped: Vec<(String, u32)> = vals.into_iter().zip(keys).collect();
    let zipped: HashSet<(String, u32)> = vals.into_iter().zip(keys).collect();

    println!("zip {:?}", zipped);

    // fold
    let vals: Vec<u32> = vec![1, 2, 3];
    let sum = vals.iter().fold(0, |acc, x|  acc + x);

    println!("fold {sum}");
}
