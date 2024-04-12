// use statements
use crate::List::{Cons, Nil};
use std::rc::Rc;


// enum representing cons list implemented with RC smart pointers
enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));  // get share count on data

    let b = Cons(3, Rc::clone(&a));  // build b on top of a, clone increases number of references sharing data
    println!("count after creating b = {}", Rc::strong_count(&a));  // get updated share count on data
    {
        let c = Cons(4, Rc::clone(&a));  // build c on top of a as well, clone increases number of references sharing data
        println!("count after creating c = {}", Rc::strong_count(&a));  // get another update to share count on data
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));  // update to share count after exiting scope
}
