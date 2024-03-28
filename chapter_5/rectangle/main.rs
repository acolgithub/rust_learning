#[derive(Debug)] // add Debug attribute

// rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
	width: 30,
	height: 30,
    };

    // print area
    println!(
	"The area of the rectangle is {} squared pixels.", area(&rect1)
    );

    // use output format called Debug
    println!("rect1 is {:?}", rect1);

    // alternative formatting
    println!("rect1 is {:#?}", rect1);

    // print using dbg! macro which returns ownership after printing the file, line number, and value
    dbg!(&rect1);
}

// area calculation of rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
