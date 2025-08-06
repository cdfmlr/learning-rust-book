unsafe fn dangerous() {
    let mut num = 5;
    let r = &mut num as *mut i32;

    *r += 1;
    println!("r = {:?}, *r = {:?}", r, *r);
}

fn main() {
    // dangerous();

    unsafe {
        dangerous();
    }
}
