use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    //fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    fn cdr(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.cdr());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.cdr());

    if let Some(link) = a.cdr() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));

    // cycle: stack overflow
    println!("a next item = {:?}", a.cdr());
}
