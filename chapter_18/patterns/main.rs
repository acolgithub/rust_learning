// function parameters

// pattern within function definition
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}


// Point struct
struct Point {
    x: i32,
    y: i32
}



// enum to represent colour
enum Colour {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}


// enum to represent message
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Colour)
}



// function that uses _ to ignore a value
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}



// struct for 3D point
struct ThreeDPoint {
    x: i32,
    y: i32,
    z: i32
}


// hello enum
enum hMessage {
    Hello {id: i32}
}



fn main() {
    // conditional if let expressions

    // get favourite colour
    let favourite_colour: Option<&str> = None;

    // check if Tuesday
    let is_tuesday = false;

    // get age by parsing string
    let age: Result<u8, _> = "34".parse();

    // if we have given a favourite colour then use that as background
    if let Some(colour) = favourite_colour {
        println!("Using your favourite colour, {colour}, as the background")
    } else if is_tuesday {  // else, if it is Tuesday use green
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {  // else check if age provided
        if age > 30 {  // if age is > 30 then purple background
            println!("Using purple as the background colour");
        } else {  // if age <= 30 then orange background
            println!("Using orange as the background colour");
        }
    } else {  // if no age given the blue background
        println!("Using blue as the background colour");
    }


    // while let conditional loops

    // create a new stack vector
    let mut stack = Vec::new();

    // add values to stack
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // while there are elements left pop them and print
    while let Some(top) = stack.pop() {
        println!("{top}")
    }



    // for loops

    // create vector of chars
    let v = vec!['a', 'b', 'c'];

    // iterate over index value pairs
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }



    // let statements

    // initialize many variables by destructuring a tuple
    let (x, y, z) = (1, 2, 3);
    println!("x={}, y={}, z={}", x, y, z);


    let point = (3, 5);
    print_coordinates(&point);


    // refutable and irrefutable patterns

    // some_option_value could be none so this will not compile
    let some_option_value = Some(5);
    // let Some(x) = some_option_value;  // cannot deal with none case

    // we can fix this code by using if let
    if let Some(x) = some_option_value {
        println!("{x}");
    }

    // compiler gives warning if you include irrefutable pattern where refutable is expected
    // if let is intended for refutable patterns
    if let x = 5 {
        println!("{x}");
    };


    // matching literals

    let x = 1;

    // match variable to cases
    match x {
        1 => println!("one"), // code prints 'one' since x=1
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything")
    }

    // matching named variables

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),  // shadows y from external scope
        _ => println!("Default case, x = {x:?}")
    }

    println!("at the end: x = {x:?}, y = {y}");


    // multiple patterns

    let x = 1;

    // uses | as or to match multiple possibilities in first arm
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything")
    }


    // matching ranges of values with ..=

    let x = 5;

    // matches a range of values in first arm
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }


    // ranges can only use numeric or char values
    // example of range of char values

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else")
    }



    // destructuring to break apart values

    // destructure the Point struct into two fields
    let p = Point {x: 0, y: 7};

    let Point {x: a, y: b} = p;  // 'a' and 'b' match values of 'x' and 'y' from Point struct
    assert_eq!(0, a);
    assert_eq!(7, b);


    // shorthand for desstructuring a struct
    let p = Point {x: 0, y: 7};

    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);


    // match statement where we check some fields with literal
    let p = Point {x: 0, y: 7};

    match p {
        Point {x, y: 0} => println!("On the x axis at {x}"),  // checks if y = 0
        Point {x: 0, y} => println!("On the y axis at {y}"),  // checks if x = 0
        Point {x, y} => {
            println!("On neither axis: ({x}, {y})");
        }
    }


    // destructuring enums
    
    // try different variant of Message to see various arms
    // let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    // let msg = Message::Write("my message".to_string());
    // let msg = Message::Move{x: 3, y: 5};
    let msg = Message::Quit;

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move {x,y} => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }

        // need to add both arms of enum Colour in order to be exhaustive
        Message::ChangeColor(Colour::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
        Message::ChangeColor(Colour::Hsv(h, s, v)) => {
            println!("Change colour to hue {h}, saturation {s}, and value {v}")
        }
    }



    // destructuring nested structs and enums

    let msg = Message::ChangeColor(Colour::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Colour::Rgb(r, g, b)) => {
            println!("Change colour to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Colour::Hsv(h, s, v)) => {
            println!("Change colour to hue {h}, saturation {s}, and value {v}")
        }
        _ => ()
    }



    // destructuring structs and tuples

    // complex destructuring involving tuple and struct types contained in tuple
    let ((feet, inches), Point {x, y}) = ((3, 16), Point {x: 3, y:-10});

    println!("{} feet {} inches, ({}, {})", feet, inches, x, y);




    // ignoring values in a pattern

    // ignoring an entire value with _
    // this function ignores the first parameter
    foo(3, 4);


    // ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {

        // check if tuple both contain something
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }

        // otherwise reset mutable first component to the second
        _ => {
            setting_value = new_setting_value;
        }
    }

    // since both entries were not None then should print Some(5)
    println!("setting is {setting_value:?}");


    // we can ignore multiple elements from tuple using _
    let numbers = (2, 4, 8, 16, 32);

    match numbers {

        // ignore second and fourth entries
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }



    // ignoring an unused variable by starting its name with _

    // first variable is ignored
    let _x = 5;
    let y = 10;  // second variable is not ignored, may get warning if not used


    // note that variables starting with _ still bind to value while _ alone does not
    // for this reason the following program causes an error since _s borrows value from s
    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    // println!("{s:?}");

    // the code that follows compiles since _ alone does not borrow
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");



    // ignoring remaining parts of a value with ..

    let origin = ThreeDPoint {x: 0, y: 0, z: 0};

    // get x value of any point, remaining coordinates do not matter
    match origin {
        ThreeDPoint {x, ..} => println!("x is {x}"),
    }

    // example with tuple
    let numbers = (2, 4, 8, 16, 32);

    // match only the first and last number
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // however, using .. must be unambiguous
    // the below example could have second be any number in the middle so results in error
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {second}")
    //     };
    // }

    // using the pattern twice is also ambiguous and results in error
    // match numbers {
    //     (first, .., .., last) => {
    //         println!("Some numbers: {first}, {last}");
    //     }
    // }



    // extra conditionals with match guards

    let num = Some(4);

    // includes match guard in first arm
    // first arm must have num match Some(x) and then it must have x % 2 be zero
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => ()
    }


    let num =Some(5);

    // match will now print that x is odd since match guard condition fails
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => ()
    }


    // we can use match guards in order to add match conditions against other variables
    let x = Some(5);
    let y = 10;

    // variable y does not shadow outer variable
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}")
    }

    println!("at the end: x = {x:?}, y = {y}");



    // @ bindings

    let msg = hMessage::Hello {id: 5};

    match msg {
        hMessage::Hello {
            id: id_variable @ 3..=7,  // creates id_variable as you are testing
        } => println!("Found an id in range: {id_variable}"),  // created variable can be used here
        hMessage::Hello {id: 10..=12} => {
            println!("Found an id in another range")
        }
        hMessage::Hello{id} => println!("Found some other id: {id}")
    }
}
