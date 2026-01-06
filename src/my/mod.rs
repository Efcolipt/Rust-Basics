use super::foo;
// use crate::foo;

pub fn call_foo() {
    foo::print();
}

// methods initial always private
pub fn print() {
    f();
    println!("my")
}

fn f() {
    a::print()
}

pub mod a;
