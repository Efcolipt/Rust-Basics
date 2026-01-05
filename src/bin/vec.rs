#![allow(unused)]

fn main() {
    // Vec<T>
    let v: Vec<i32> = vec![1, 0, 1];
    let v: Vec<u32> = vec![1, 2, 3];
    let v: Vec<i32> = Vec::new();

    let v = vec![1u32, 2, 3];
    let v = vec![1u8, 1, 1, 1, 1];
    let v = vec![1u8; 5];
    println!("v = {:?}, len = {}", v, v.len());

    // get
    let v = vec![1u32, 2, 3];
    let x = v[0];
    let x = v.get(0);
    match x {
        Some(v) => println!("{:?} ", v),
        None => println!("none"),
    }

    // update
    let mut v = vec![1, 2, 3];
    v[1] = 99;

    // push
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?} ", v);

    // pop
    let mut v = vec![1, 2, 3];

    match v.pop() {
        Some(v) => println!("pop: {v}"),
        None => println!("none"),
    }
    match v.pop() {
        Some(v) => println!("pop: {v}"),
        None => println!("none"),
    }
    match v.pop() {
        Some(v) => println!("pop: {v}"),
        None => println!("none"),
    }
    match v.pop() {
        Some(v) => println!("pop: {v}"),
        None => println!("none"),
    }

    // slice
    let v: Vec<u32> = vec![5; 5];
    let s = &v[..=2];

    println!("slice {:?}", s)

}
