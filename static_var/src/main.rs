static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    println!("{}", HELLO_WORLD);
    
    println!("COUNTER = {}", COUNTER); // ❌ E0133: use of mutable static is unsafe
                                       //
    unsafe {
        COUNTER = 3;
        println!("COUNTER = {}", COUNTER);
    }
}
