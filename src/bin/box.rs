#![allow(unused)]

// Smart pointer
// Pointer with metadata and additional capabilities

// Box
// - Allows data to be stored on the heap
// - Useful for data where the size is not known at compile time
//   - Trait objects
//   - Recursive data structure

use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let num = data.parse()?;
    Ok(num)
}

#[derive(Debug)]
struct Tree {
    val: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

fn main() {
    let i = 1;
    let b = Box::new(i);
    let v = *b;

    let tree = Tree {
        val: 1,
        left: Some(Box::new(Tree {
            val: 2,
            left: None,
            right: Some(Box::new(Tree {
                val: 3,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Tree {
            val: 2,
            left: None,
            right: Some(Box::new(Tree {
                val: 3,
                left: None,
                right: None,
            })),
        })),
    };

    println!("{:?}", tree);
    println!("{:?}", tree.left.unwrap().right.unwrap().val);
}
