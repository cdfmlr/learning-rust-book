struct CustomSmartPointer (String);

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`.", self.0);
    }
}

fn main() {
    let a = CustomSmartPointer(String::from("a"));
    let b = CustomSmartPointer(String::from("b"));

    println!("CustomSmartPointers created.");
    
    let c = CustomSmartPointer(String::from("c"));
    // c.drop();
    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
    
}
