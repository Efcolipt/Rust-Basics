use super::foo;

// same initial always private
#[derive(Debug)]
pub struct S {
    pub id: u32,
    pub name: String,
}

pub fn print() {
    println!("a")
}

pub fn call_foo() {
    foo::print();
}
