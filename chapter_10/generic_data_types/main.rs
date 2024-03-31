// structure with one generic type
struct Point<T> {
    x: T,
    y: T,
}

// generic implementation
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// implementation specific to Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// structure with two generic types
struct Point2<T, U> {
    x: T,
    y: U,
}

// implementation which shows generic types don't need to precisely match the method signature
impl <T, U> Point2<T, U> {  // <T, U> go with the struct definition
    fn mixup<X2, Y2>(self, other:Point2<X2, Y2>) -> Point2<T, Y2> {  // other has different generic types, note that we were consistent in referring though
        Point2 {                                                      // X2, Y2 are only relevant to the method
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // create instances of Point
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y:4.0};

    // this code causes errors
    //let wont_work = Point {x: 5, y: 4.0};  // won't work due to different types

    // create instances of Point2
    let both_integer = Point2 {x: 5, y: 10};
    let both_float = Point2 {x: 1.0, y: 4.0};
    let integer_and_float = Point2 {x: 5, y: 4.0};

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());

    // illustration of use of mixup
    let p1 = Point2 {x:5, y:10.4};
    let p2 = Point2 {x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// generic function for finding largest element in list containing type T
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

