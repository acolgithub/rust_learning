// create custom smart pointer struct
struct CustomSmartPointer {
    data: String,
}

// provide implementation for drop trait which prints a message when dropping
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}


fn main() {

    // define some smart pointers from custom struct
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");  // print message to indicate smart pointers are both created

    // apply std::mem::drop function to drop pointer early
    drop(c);  // we can see a message indicating smart pointer is dropped before pointer d is dropped (i.e. different than expected order)
    println!("CustomSmartPointer dropped before the end of main.")

}
