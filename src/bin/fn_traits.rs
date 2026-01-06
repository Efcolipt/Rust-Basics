#![allow(unused)]

fn f_fn_pointer(f: fn(i32) -> i32) {}

// Fn, FnMut and FnOnce traits
// fn f_closure_input(f: ?) {}

fn f_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn f_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn f_fn_once<F: FnOnce()>(mut f: F) {
    f();
}

fn main() {
    // Fn (capture by &)
    // - immutable borrow from environment
    // - can be called more than once

    // FnMut (capture by &mut)
    // - mutable borrow from environment
    // - can be called more than once

    // FnOnce (capture by value)
    // - moves captured values into closures, if needed
    // - can be called at least once

    // Fn (capture by &)
    let s = "a".to_string();
    let f = || println!("fn: {s}");

    f_fn(f);
    f_fn(f);

    println!("main: {s}");

    // FnMut (capture by &mut)
    let mut v = vec![1, 2, 3];
    let mut f = || v.push(0);

    f_fn_mut(f);

    println!("main: {:?}", v);

    // FnOnce (capture by value) dyn
    let v = vec![1, 2, 3];
    let f = move || println!("fn once: {:?}", v);

    f_fn_once(f);
    // Cant, u already moved ownership and GC dropped
    // f_fn_once(f);
    // println!("fn once: {:?}", v)

    // FnOnce (capture by value)
    let v = 123;
    let f = move || println!("fn once: {:?}", v);

    // Can cos just copy
    f_fn_once(f);
    f_fn_once(f);
}
