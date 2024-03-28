fn main() {
    // create String variable
    let s1 = String::from("hello");

    // get length of string
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // try to change borrowed information (this code causes error)
    //change(&s1);

    // create mutable string
    let mut s = String::from("hello");

    // change string through mutable reference
    mut_change(&mut s);
    println!("{s}");

    // cannot have two mutable references to the same value (this code causes error)
    //let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{}, {}", r1, r2);  // code compiles if we just assign r1 and r2 but cannot use them

    // proper implementation of multiple mutable references
    {
	let r1 = &mut s;  // first mutable reference
    }  // r1 goes out of scope here, so we can make a new reference with no problems

    let r2 = &mut s;  // second mutable reference

    // combining mutable and immutable references (this code causes error)
    //let ref1 = &s; // no problem
    //let ref2 = &s; // no problem
    //let ref3 = &mut s;  // BIG PROBLEM
    //println!("{}, {}, and {}", r1, r2, r3);  // cannot have mutable refefrence while we have an immutable one at the same time

    // can include mutable reference after mutable references if we do not use them since scope ends after last usage
    let ref1 = &s;  // no problem
    let ref2 = &s;  // no problem
    println!("{} and {}", ref1, ref2);
    // variables r1 and r2 will not be used after this point

    let ref3 = &mut s;  // no problem
    println!("{}", ref3);

    // attempt to create dangling reference (this code causes error)
    //let reference_to_nothing = dangle()
}

// calculate length of string
fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}  // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

// attempt to change borrowed information
//fn change(some_string: &String) {
//    some_string.push_str(", world");  // attempt to append to string
//}

// function using mutable reference to change mutable Strings
fn mut_change(some_string: &mut String) {
    some_string.push_str(", world");
}

// create dangling reference
//fn dangle() -> &String {  // dangle returns a reference to a String
//    let s = String::from("hello");  // s is a new String
//
//    &s  // we return a reference to the String, s
//}  // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

// prevent dangle
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
