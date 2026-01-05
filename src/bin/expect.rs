#![allow(unused)]

fn main() {
    // unwrap and expect
    let x: Option<u32> = Some(1);

    // unwrap
    match x {
        Some(val) => println!("val = {val}"),
        None => panic!("None"),
    }

    // Same above
    let v = x.unwrap();
    println!("val = {v}");

    // expect
    match x {
        Some(val) => println!("val = {val}"),
        None => panic!("x is none"),
    }

    // Same above
    let v = x.expect("x is none");
    println!("val = {v}");

    // Result
    let x = 1;
    let y = 1;
    let z: Result<u32, String> = Err("div by 0".to_string());

    // match z {
    //     Ok(val) => println!("val = {val}"),
    //     Err(err) => panic!("{:?}", err),
    // }

    // Same above
    let v = z.unwrap();
    println!("val = {v}")
}
