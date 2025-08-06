fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining only_borrows: {:?}", list);
    let only_borrows = || println!("From only_borrows: {:?}", list);
    println!("After defining only_borrows: {:?}", list);

    println!("Before calling only_borrows: {:?}", list);
    only_borrows();
    println!("After calling only_borrows: {:?}", list);



    println!("Before defining borrows_mutably: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // cannot print here: list 被闭包 mut 借用了，不能再 immutable borrow 去打印
    // println!("After defining borrows_mutably: {:?}", list);

    // 可以连着调用几次的
    borrows_mutably();
    borrows_mutably();
    println!("After calling borrows_mutably: {:?}", list);
}
