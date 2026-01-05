#![allow(unused)]

// Compound data types
// - tuple
// - array

fn main() {
    // Tuple
    let t: (bool, u32, char) = (true, 1, 'c');

    // Destructure
    let (a, b, c) = t;

    // ignore with _
    let (_, b, _) = t;

    // Empty tuple - unit type
    let t = ();

    // Nested type

    let nested = ((1.2, 'a'), (true, 1u32, 'b'), ());

    // Access
    let t: (bool, u32, char) = (true, 1, 'c');

    println!("t = {}, {}, {}", t.0, t.1, t.2);

    println!("nested {}", nested.1.0);
}
