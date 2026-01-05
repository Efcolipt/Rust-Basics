#![allow(unused)]

enum Animal {
    Cat,
    Dog,
}

fn main() {
    // match
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // multiple cases
    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other"),
    }

    // range
    match x {
        1..10 => println!("between 1 and 10"),
        _ => println!("other"),
    }

    // @
    match x {
        i @ 1..10 => println!("matched {i}"),
        _ => println!("other"),
    }

    // return value
    let animal = Animal::Cat;

    let animal_sound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "woof",
        _ => "?",
    };

    println!("animal_sound {animal_sound}");

    // Option
    let x: Option<i32> = Some(1);
    match x {
        Some(v) => println!("Some {v}"),
        None => println!("none"),
    }

    let res: Result<u32, String> = Ok(10);

    match res {
        Ok(val) => println!("ok {val}"),
        Err(msg) => println!("err {msg}"),
    }
}
