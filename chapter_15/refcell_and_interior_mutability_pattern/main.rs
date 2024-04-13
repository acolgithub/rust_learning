// use statements
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// create enum with Rc smart pointer to Refcell smart pointers
// this allows for sharing and interior mutability
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


fn main() {
    // define Rc smart pointer reference to Refcell smart pointer reference
    let value = Rc::new(RefCell::new(5));

    // a pairs 5 and Nil
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));  // b pairs 3 with (5, Nil)
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));  // c pairs 4 with (5, Nil)

    *value.borrow_mut() += 10;  // modify starting value of 5 to be 15

    // print cons lists
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
