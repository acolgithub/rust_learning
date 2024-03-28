fn main() {
    // repeating code with loop (code loops infinitely)
    loop {
    	println!("again!");
	break;  // ends loop
    }

    // returning values from loops
    let mut counter = 0;

    let result = loop {
	counter += 1;  // add one to counter

	if counter == 10 {
	    break counter * 2;  // returns 20 when loop ends to provide assignment
	}
    };

    println!("The result is {result}");

    // loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {  // uses loop label in order to modify outer loop
	println!("count = {count}");
	let mut remaining = 10;

	loop {
	    println!("remaining = {remaining}");
	    if remaining == 9 {
		break;  // breaks inner loop
	    }
	    if count == 2 {
		break 'counting_up;  // breaks outer loop
	    }
	    remaining -= 1;
	}

	count += 1;
    }
    println!("End count = {count}");

    // conditional loops with while
    let mut number = 3;

    // first use loop with if, else, and break
    loop {
	if number == 0 {
	    break;
	}
	println!("{number}!");
	number -= 1;
    }
    println!("LIFTOFF!!!");

    // now use while loop
    let mut number = 3;

    while number != 0 {
	println!("{number}!");

	number -= 1;
    }
    println!("LIFTOFF!!!");

    // looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
	println!("the value is: {}", a[index]);
	index += 1;
    }

    // use a for loop to loop over elements
    for element in a {
	println!("the value is: {element}");
    }

    // for loop with rev (rev reverses range)
    for number in (1..4).rev() {
	println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
