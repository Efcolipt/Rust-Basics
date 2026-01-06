#![allow(unused)]

use std::thread;
// Multi producer, single consumer
use std::sync::mpsc::{self, TryRecvError};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    // std::mem::drop(rx);

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(format!("{i}")).unwrap();
        });
    }

    std::mem::drop(tx);

    loop {
        match rx.recv() {
            Ok(msg) => println!("{msg}"),
            Err(err) => {
                println!("err {:#?}", err);
                break;
            }
        }
    }

    // loop {
    //     match rx.try_recv() {
    //         Ok(msg) => println!("res: {:#?}", msg),
    //         Err(TryRecvError::Empty) => println!("no msg"),
    //         Err(TryRecvError::Disconnected) => {
    //             println!("disc");
    //             break;
    //         }
    //     }
    //     thread::sleep(Duration::from_millis(10));
    // }
}
