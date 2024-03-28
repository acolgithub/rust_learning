fn main() {
    // unsigned 8, 16, 32, 64, and 128 bit integers
    let x_8: u8 = 1;
    let x_16: u16 = 2;
    let x_32: u32 = 3;
    let x_64: u64 = 5;
    let x_128: u128 = 6;
    println!("{}, {}, {}, {}, {}", x_8, x_16, x_32, x_64, x_128);

    // signed 8, 16, 32, 64, and 128 bit integers
    let y_8: i8 = 7;
    let y_16: i16 = 8;
    let y_32: i32 = 9;
    let y_64: i64 = 10;
    let y_128: u128 = 11;
    println!("{}, {}, {}, {}, {}", y_8, y_16, y_32, y_64, y_128);

    // decimal literal
    let dec = 98_222;  // "_" is a comma
    println!("decimal liter: {dec}");

    // hex literal
    let hex = 0xff;
    println!("hex literal: {hex}");

    // octal literal
    let oct = 0o77;
    println!("octal literal: {oct}");

    //binary literal
    let bin = 0b1111_0000;
    println!("binary literal: {bin}");

    // byte literal
    let byt = b'A';
    println!("byte literal: {byt}");

    // floating point numbers
    let f1 = 2.0;  // f64
    let f2: f32 = 3.0;  // f32
    println!("floating point numbers: {f1} and {f2}");

    // numerical operations
    let sum = 5 + 10;  // addition
    println!("Result of addition: {sum}");
    
    let difference = 99.5 - 4.3;  // subtraction
    println!("Result of subtraction: {difference}");

    let product = 4 * 30;  // multiplication
    println!("Result of multiplications: {product}");

    let quotient = 56.7 / 32.2;  // division
    let truncated = -5 / 3;  // integer division, results in -1
    println!("Result of float/double division: {quotient}");
    println!("Result of integer division: {truncated}");

    let remainder = 43 % 5;  // modular arithmetic
    println!("Result of modular arithmetic: {remainder}");

    // boolean type
    let t = true;
    let f: bool = false;  // with explicit type annotation
    println!("Values of bool variables: {}, {}", t, f);

    // character type
    let c = 'z';
    let z: char = 'Z';  // with explicit type annotation
    let heart_eyed_cat = '\u{1F63B}';  // pasted in cat emoji
    println!("Values of char variables: {c}, {z}, {heart_eyed_cat}");

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);  // tuple with signed 32 bit integer, double, unsigned 8 bit integer
    //println!("{tup}");  // cannot print tuple directly (results in error)

    let (a, b, c) = tup;  // assign entries to varaibles
    println!("The value of b is: {b}");

    // access entries of tuple using period notation
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The tuple entries are: {five_hundred}, {six_point_four}, {one}");

    // array type
    let arr = [1, 2, 3, 4, 5];
    //println!("{arr}");  // cannot print array directly (results in error)

    // array of months
    let months = ["January", "February", "March", "April", "May", "June", "July",
		  "August", "September", "October", "November", "December"];

    // array with type annotation
    let arr: [i32; 5] = [1, 2, 3, 4, 5];  // [type of entries, number of entries]

    // shorthand notation for array with same value appearing repeatedly
    let arr_repeat = [3; 5];  // contains "3" occuring five times

    // accessing array elements
    let first = arr[0];
    let second = arr[1];
    println!("First and second entries of array: {first}, {second}");

    // cannot print entries of array past bounds (results in error)
    //let entry = arr[6];  // goes past size of array
    //let entry = arr[-1];  // goes below 0
    //println!("{entry}");
}
