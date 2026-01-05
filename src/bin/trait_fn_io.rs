#![allow(unused)]

trait Animal {
    fn speak(&self) -> String;
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".to_string()
    }
}

impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".to_string()
    }
}

// Static dispatch, we known at compile time
fn greet(animal: &impl Animal) {
    println!("static {}", animal.speak());
}

// Dynamic dispatch, we is not known at compile time
fn greet_dyn(animal: &dyn Animal) {
    println!("dyn {}", animal.speak());
}

fn return_concrete_type() -> impl Animal {
    Dog
}

fn rand_animal(rand: u32) -> Box<dyn Animal> {
    if rand <= 10 {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    greet(&cat);
    greet(&dog);

    let animal = return_concrete_type();
    println!("{}", animal.speak());

    let animal_str = "cat";

    // Dynamic
    let animal: &dyn Animal = match animal_str {
        "dog" => &Dog,
        _ => &Cat,
    };

    greet_dyn(animal);

    let animal = rand_animal(11);
    println!("{}", animal.speak());
}
