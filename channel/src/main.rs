use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // ❌ val moved to tx.send
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
