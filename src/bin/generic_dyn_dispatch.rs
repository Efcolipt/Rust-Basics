#![allow(unused)]

// Static dispatch
// - function to call is known at compile time
// - monomorphization (code size can be larger)
// - no run time cost (no vtable lookup)

// Dynamic dispatch
// - function to call is known at run time
// - vtable lookup (code size is smaller)
// - run time overhead (vtable lookup)

#[derive(Debug)]
struct A {}

#[derive(Debug)]
struct B {}

trait F {
    fn f(&self);
}

impl F for A {
    fn f(&self) {
        println!("{:?}", self)
    }
}

impl F for B {
    fn f(&self) {
        println!("{:?}", self)
    }
}

fn static_dispatch<T: F>(t: &T) {
    t.f();
}

fn dyn_dispatch(t: &dyn F) {
    t.f();
}

fn dyn_dispatch_box(t: Box<dyn F>) {
    t.f();
}

fn main() {
    let obj = A {};
    static_dispatch(&obj);

    let obj = B {};
    static_dispatch(&obj);

    let input = "A";
    // Train object
    // Value that implements a trait,
    // but its concrete type in unknown at compile time

    let obj: &dyn F = match input {
        "A" => &A {},
        "B" => &B {},
        _ => panic!(),
    };

    // Value only borrowed, so ownership stay inside a main function
    dyn_dispatch(obj);

    let obj: Box<dyn F> = match input {
        "A" => Box::new(A {}),
        "B" => Box::new(B {}),
        _ => panic!(),
    };

    // The ownership and value transfers into function
    dyn_dispatch_box(obj);
}
