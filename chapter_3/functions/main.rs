fn main() {
    println!("Hello, world!");

    // call function
    another_function();

    // call function with parameters
    another_function_parameters(5);

    // call function with multiple parameters
    print_labeled_measurement(5, 'h');

    // can assign using expression created by scope block
    let y = {
	let x = 3;
	x + 1  // no semicolon at the end, so this is an expression
    };
    println!("The value of y is: {y}");

    // adding a semicolon makes it a statement so no return value (this code creates error)
    //let z = {
    //	let x = 3;
    //	x + 1;
    //};  // interestingly the assignment still compiles
    //println!("The value of z is: {z}");  // but the print causes an error

    // function with return value
    let x = five();  // assignment based on expression (functions with return value are expressions)
    println!("The value of x is: {x}");

    // assign by function which modifies input and give return value
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// separate function defined after main
fn another_function() {
    println!("Another function.");
}

// function with parameters
fn another_function_parameters(x: i32) {
    println!("The value of x is: {x}");
}

// function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
