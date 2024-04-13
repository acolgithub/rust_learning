// use statements
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};


#[derive(Debug)]
// const list definition using Refcell and Rc pointers
enum List {
    Cons(i32, RefCell<Rc<List>>),  // can modify the List value a Cons variant is pointing to
    Nil,
}

impl List {

    // method to access second item
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


// create tree data structure
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,  // weak reference since ownership of parent is not needed (if parent is dropped then child is too but not vice versa)
    children: RefCell<Vec<Rc<Node>>>,  // has ownership of children nodes
}


fn main() {
    // first define Smart Pointer to Cons Object
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // next define Smart Pointer b pointing to Cons object on top of a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // get share counts
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());



    // create some node instances
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // leaf current has no parent
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // print weak and strong reference count of leaf
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // make branch the parent of leaf
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // print strong and weak reference count of branch
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    // print strong and weak reference count of leaf
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // final strong and weak reference count of leaf
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
