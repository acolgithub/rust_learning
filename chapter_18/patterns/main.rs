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


// enum to represent message
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
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
    // let msg = Message::ChangeColor(0, 160, 255);
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
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }
}
