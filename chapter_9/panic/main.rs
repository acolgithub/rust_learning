fn main() {
    
    // make program panic using panic!
    //panic!("crash and burn");  // prints "crash and burn" error message

    // use panic! backtrace
    let v = vec![1, 2, 3];

    v[99];  // out of range, causes program to panic
}
