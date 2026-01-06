#![allow(unused)]

// Arc (Rc) + Mutex (RefCell)

use std::{
    sync::{Mutex, MutexGuard},
    thread,
};

fn main() {
    let m: Mutex<i32> = Mutex::new(0);

    thread::scope(|scope| {
        scope.spawn(|| {
            let mut v: MutexGuard<'_, i32> = m.lock().unwrap();
            *v += 1;
        });

        scope.spawn(|| {
            let mut v: MutexGuard<'_, i32> = m.lock().unwrap();
            *v += 1;
        });
    });

    println!("mutex: {:?}", m);
}
