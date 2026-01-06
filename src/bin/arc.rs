#![allow(unused)]

// Arc (Rc) + Mutex (RefCell)

use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
};

fn main() {
    // Rc<RefCell>
    // Arc<Mutex>
    let m: Mutex<i32> = Mutex::new(0);
    let counter = Arc::new(m);

    let c1 = Arc::clone(&counter);
    let c2 = Arc::clone(&counter);

    let h1 = thread::spawn(move || {
        let mut v = c1.lock().unwrap();
        *v += 1;
    });

    let h2 = thread::spawn(move || {
        let mut v = c2.lock().unwrap();
        *v += 1;
    });

    (h1.join().unwrap(), h2.join().unwrap());

    println!("mutex: {:?}", counter);

    // Code above same
    // thread::scope(|scope| {
    //     scope.spawn(|| {
    //         let mut v: MutexGuard<'_, i32> = m.lock().unwrap();
    //         *v += 1;
    //     });

    //     scope.spawn(|| {
    //         let mut v: MutexGuard<'_, i32> = m.lock().unwrap();
    //         *v += 1;
    //     });
    // });
}
