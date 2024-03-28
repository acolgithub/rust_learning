// cannot create variable outside of main (the results in error)
//let z = 3;

// create constant outside of main
const CONSTANT_OUTSIDE_MAIN: u32 = 3;


fn main() {
    // create immutable variable
    let x = 5;
    println!("The value of x is: {x}");

    // attempt to reassign to immutable variable (this results in error)
    //x = 6;
    //println!("The value of x is: {x}");

    // create mutable variable
    let mut y = 5;
    println!("The value of y is: {y}");

    // reassign to mutable variable
    y = 6;
    println!("The value of y is: {y}");

    // create constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadow a variable
    let x = x + 1;
    println!("The value of x is now: {x}");

    // shadow again inside scope
    {
    	let x = x * 2;
    	println!("The value of x in the inner scope is: {x}");
    }

    // print older shadow value in outer scope
    println!("THe value of x in outside scope is: {x}");

    // cannot shadow constant (this results in error)
    //const THREE_HOURS_IN_SECONDS: u32 = 50;

    // shadow variable with different type
    let spaces = "   ";  // string variable
    let spaces = spaces.len();  // number variable

    // cannot modify mutable variable type (this results in error)
    //let mut spaces_mut = "   ";
    //spaces_mut = spaces_mut.len();
}
