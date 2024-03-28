fn main() {
    // example of string slice
    let s = String::from("hello world");
    let hello = &s[0..5];  // slice off first word
    let world = &s[6..11];  // slice off second word
    println!("{hello}");
    println!("{world}");

    // can drop 0 since starts there by default
    let hello2 = &s[..5];  // same as hello
    println!("{hello2}");

    // can also drop last number if goes to end, or use stored value
    let world2 = &s[6..];  // same as world
    let len = s.len();
    let world3 = &s[6..len];
    println!("{world2}, {world3}");

    // can print whole string this way too
    let whole = &s[..];
    println!("{whole}");

    // get first word
    let word = first_word(&s);

    // try to clear word and print (this code results in error)
    //s.clear();
    //println!("the first word is: {}", word);

    // array slice
    let a = [1, 2, 3, 4, 5];
    let arr_slice = &a[1..3];

    // check if slice matches [2, 3]
    assert_eq!(arr_slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    // string as bytes
    let bytes = s.as_bytes();

    // find index of first space and return substring
    for (i, &item) in bytes.iter().enumerate() {
	if item == b' ' {
	    return &s[0..i];
	}
    }

    &s[..]  // return whole string if no space is found
}
