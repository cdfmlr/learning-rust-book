use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();

    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("foo"),
            String::from("bar"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
