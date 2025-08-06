fn main() {
    // 引用 转为 裸指针：
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 直接指向内存地址：
    let address = 0x114514usize;
    let r = address as *const i32;

    unsafe {
        // *r1 = 1; // ❌ error[E0594] cannot assign to *(*const T)
        println!("r1 = {:?}, *r1 = {:?}", r1, *r1);
        *r2 = 2;
        println!("r2 = {:?}, *r2 = {:?}", r2, *r2);

        println!("r = {:?}", r);
        println!("r = {:?}", *r); // ❌ 运行时段错误
    }
}
