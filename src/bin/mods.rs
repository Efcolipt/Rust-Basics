#![allow(unused)]

// Modules
use rust::foo;
use rust::my;
use rust::my::a;

fn main() {
    my::print();
    my::a::print();

    let s = my::a::S {
        id: 1,
        name: "S".to_string(),
    };

    println!("{:?}", s);

    my::call_foo();

    my::a::call_foo();

    foo::print();

    a::print();
}
