// function parameters

// pattern within function definition
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
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
}
