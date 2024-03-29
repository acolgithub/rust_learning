fn main() {
    // enumerate types of IP address versions
    enum IpAddrKind {
	V4(String),
	V6(String),
    }

    // define instances of IP versions
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    // enum with many types
    enum Message {
	Quit,
	Move { x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32),
    }

    // method on enum
    impl Message {
	fn call(&self) {
	    //method body would be defined here
	}
    }

    // examples with Some and None (from Option enum)
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

