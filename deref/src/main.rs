fn main() {
    let x = 5;
    let y_ref = &x;
    let y_box = Box::new(x);

    assert_eq!(5, *y_ref);
    assert_eq!(5, *y_box);
}
