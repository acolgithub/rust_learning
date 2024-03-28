#[derive(Debug)]

// rectangle class
struct Rectangle {
    width: u32,
    height: u32,
}

// area method for rectangle struct (implementation)
impl Rectangle {
    fn area(&self) -> u32 {
	self.width * self.height
    }
}

// get width method
impl Rectangle {
    fn width(&self) -> u32 {
	self.width
    }
}

// get height method
impl Rectangle {
    fn height(&self) -> u32 {
	self.height
    }
}

// implementation of can_hold
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
	self.width > other.width && self.height > other.height
    }
}

// associated function that is not method of Rectangle
impl Rectangle {
    fn square(size: u32) -> Self {
	Self {
	    width: size,
	    height: size,
	}
    }
}

fn main() {
    // rectangle instance
    let rect1 = Rectangle {
	width: 30,
	height: 50,
    };

    // print area
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // get square instance
    let sq = Rectangle::square(3);
}
