fn main() {
    // string literal, scope ends at end of program
    let s = "hello";

    // demonstration of scope
    {			    // S is not valid here, it's not yet declared
	let S = "hello";    // S is valid from this point forward
    }			    // this scope is now over, and S i no longer valid

    // create String from string literal
    let s = String::from("hello");

    // create mutable String
    let mut s = String::from("hello");

    // append to String
    s.push_str(", world!");  // push_str() appends a literal to a String

    println!("{}", s);  // this will print 'hello, world!'

    // two integers with values pushed onto stack
    let x = 5;
    let y = x;  // copy of x

    // 
    let s1 = String::from("hello");  // pointer to memory on stack, length of string, capacity; contents is on heap
    let s2 = s1;  // data is copied (copy pointer, length, capacity from stack; do not copy heap contents)
    // since s1, s2 point to same contents this could lead to double free error so Rust considers s1 as no longer valid after creating s2

    // attempting to use s1 now results in error (this code results in error)
    //println!("{}, world!", s1);

    // make deep copy of heap data
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // example of stack-only copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // ownership and functions
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);  // s's value moves into the function...
			 // and so is no longer valid here

    // try using s after function (this code caues error)
    //println!("{s}");

    let x = 5;  // x comes into scope

    makes_copy(x);  // x would move into the function, but i32 is Copy, so it's oaky to still use x afterward

    // okay to use x after
    println!("{x}");

    // Return values and scope
    let s1 = gives_ownership();  // gives_ownership moves its return value into s1

    let s2 = String::from("hello");  // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    let s4 = String::from("hello");

    let (s5, len) = calculate_lengths(s4);

    println!("The length of '{}' is {}.", s5, len);
}// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) {  // some_string comes into scope
    println!("{}", some_string);
}  // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}  // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {  // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours");  // some_string comes into scope
    some_string  // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {  // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_lengths(s: String) -> (String, usize) {
    let length = s.len();  // len() returns the length of a String

    (s, length)
}
