#![allow(unused)]

// Sized and ?Sized

// Sized
// - Size is known at compile time
// - Automatically implemented for primitive types
// - Necessary for allocating values on the stack

// ?Sized
// - Size may not be known at compile time
// - Examples = dynamically sized types, slices, trait objects

fn f<T: Sized>(x: T) {}
fn g<T: ?Sized>(x: &T) {}

trait A {}
impl A for u32 {}
impl A for String {}
fn d(x: Box<dyn A>) {}

fn main() {
    // Sized
    let i: i32 = 1;
    let x: f64 = 1.0;
    let b: bool = true;

    f(i);
    f(x);
    f(b);

    struct S {
        i: u32,
        j: u32,
    };

    let s = S { i: 1, j: 1 };

    f(s);

    let arr: [i32; 4] = [0; 4];
    f(arr);
    f(&arr);

    // ?Sized

    let slice: &[u32] = &[1, 2, 3];
    g(slice);

    let s: &str = "hello";
    g(s);

    let w: Box<dyn A> = Box::new("string".to_string());
    g(&w);
    
    let w: Box<dyn A> = Box::new(1u32);
    g(&w);

}
