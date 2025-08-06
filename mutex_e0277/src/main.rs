use std::rc::Rc;
use std::thread;
use std::sync::Mutex;

fn main() {
    let counter = Rc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }
}
