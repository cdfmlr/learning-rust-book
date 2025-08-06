use std::thread;
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    for _ in 0..10 {
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }
}
