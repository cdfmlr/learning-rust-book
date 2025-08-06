#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 };
    println!("{}", f);

    unsafe {
        match u {
            MyUnion { f1: 1 } => {
                println!("one");
            }
            MyUnion { f2 } => {
                println!("{}", f2);
            }
        }
    }
}
