// create enum definition of a cons list
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};


fn main() {
    // create box smart pointer to i32
    let b = Box::new(5);
    println!("b = {}", b);


    // define Cons list
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
