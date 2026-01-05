#![allow(unused)]

// Compound data types
// - tuple
// - array

fn main() {
    // Array - fixed length, known at compile time;
    let arr: [u32; 3] = [1, 2, 3];
    println!("array arr[2] = {}", arr[2]);

    let mut arr: [u32; 2] = [1, 2];
    arr[1] = 9;

    // create with range - [0,0,0,0,0,0,0,0,] = [0; 10];
    let arr: [u32; 10] = [0; 10];
    println!("{:?}", arr);

    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    // Slice - length  not known at compile time;

    // First 3 elements, not include 3
    let s = &nums[..3];
    println!("first 3 elements {:?}", s);

    // Middle 4 elements
    let s = &nums[3..7];
    println!("middle 4 elements {:?}", s);

    // Last 3 elements
    let s = &nums[7..];
    println!("last 3 elements {:?}", s);

    // All elements
    let s = &nums[..];
    println!("all elements {:?}", s);
}
