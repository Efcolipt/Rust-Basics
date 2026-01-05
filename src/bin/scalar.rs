#![allow(unused)]

fn main() {
    // Signed int
    // -(2**(n-1)) ~ 2**(n-1) - 1
    let i0: i8 = 0;
    let i2: i16 = 0;
    let i3: i64 = 0;
    let i4: i128 = 0;
    let i5: isize = 1;

    // Unsigned int
    // 0 ~ 2**n - 1
    let u0: u8 = 0;
    let u2: u16 = 0;
    let u3: u64 = 0;
    let u4: u128 = 0;
    let u5: usize = 1;

    //Floats
    let f0: f32 = 0.0;
    let f2: f64 = 0.0;

    // Boolean
    let b: bool = true;

    // Characters
    let c: char = 'c';

    // Type conversion
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // Min and max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");

    let min_c: char = char::MIN;
    let max_c: char = char::MAX;

    println!("char min: {min_c}");
    println!("char max: {max_c}");

    // Overflow

    let mut u: u32 = u32::MAX;

    u += 1;

    println!("overflow u32: {u}");

    // checked_add - Some(x) | None
    let u = u32::checked_add(u32::MAX, 1);

    println!("checked_add {:?}", u);

    // wrapping_add
    let u = u32::wrapping_add(u32::MAX, 1);

    println!("wrapping_add {:?}", u);
}
