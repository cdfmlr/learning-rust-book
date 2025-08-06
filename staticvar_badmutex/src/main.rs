use std::thread;

static mut COUNTER: u32 = 0;

fn main() {
    unsafe {
        COUNTER = 1000;
    }

    let thread1 = thread::spawn(|| {
        for _i in 1..1000 {
            unsafe {
                COUNTER -= 1;
            }
        }
    });

    let thread2 = thread::spawn(|| {
        for _i in 1..1000 {
            unsafe {
                COUNTER += 1;
            }
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    unsafe {
        println!("COUNTER = {}", COUNTER);
    }
}
