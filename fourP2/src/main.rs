// MULTITHREADING
// Fearless concurrency on rust book 16.1 and 16.2
//  MESSAGE PASSING

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
/* 
    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("Hi Number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
// here now we will wait for this thread to complete using this handle.join
    handle.join().unwrap();
    for i in 1..5 {
        println!("Hii number {i} from the main thread!");
        thread::sleep(Duration::from_millis(i));
    }
    println!("Hello, world!"); */

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
