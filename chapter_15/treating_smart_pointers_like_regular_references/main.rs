// use statements
use std::ops::Deref;

// define our own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// create dereference implementation for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0  // returns reference to value we wish to access
    }
}

// illustration of deref coercion
fn hello(name: &str) {
    println!("Hello, {name}!");
}


fn main() {
    // create standard pointer to i32 values
    let x = 5;
    let y = &x;

    // check values are equal
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // create Box smart pointer to i32 value
    let z = Box::new(x);

    // check values are equal
    assert_eq!(5, *z);

    // create MyBoX smart pointer to i32 value
    let w = MyBox::new(x);

    // check values are equal
    assert_eq!(5, *w);

    // test function using deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
