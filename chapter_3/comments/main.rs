fn main() {
    // if expression
    let number = 3;

    if number < 5 {
	println!("condition was true");
    }
    else{
	println!("condition was false");
    }

    // condition must be boolean type (this codes results in error)
    //if number {
    //	println!("number was three");
    //}

    // check if number is not zero
    if number != 0 {
	println!("number was something other than zero");
    }

    // multiple conditions with else if
    let number = 6;

    if number % 4 == 0 {
	println!("number is divisible by 4");
    } else if number % 3 == 0 {
	println!("number is divisible by 3");
    } else if number % 2 == 0 {
	println!("number is divisible by 2");
    } else {
	println!("number is not divisible by 4, 3, or 2");
    }

    // if defines an expression
    let condition = true;
    let number = if condition { 5 } else { 6 };  // can be used in assignment, note that types of both ending expressions must be the same

    println!("The value of number is: {number}");

    // mismatched types in assignment through if (this code results in error)
    //let number = if condition { 5 } else { "six" };
    //println!("The value of number is: {number}");
}
