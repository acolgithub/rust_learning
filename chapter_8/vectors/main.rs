fn main() {
    
    // create new vector holding type i32
    let v: Vec<i32> = Vec::new();

    // create new vector using vec! macro holding type i32
    let v2 = vec![1, 2, 3];

    // add elements to mutable vector using push
    let mut v_mut = Vec::new();  // note that type is inferred from input

    v_mut.push(5);
    v_mut.push(6);
    v_mut.push(7);
    v_mut.push(8);

    // access values in a vector using indexing and get methods
    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Rust allows different access methods to customize behaviour
    //let does_not_exist1 = &v3[100];  // program panics
    //let does_not_exist2 = v.get(100);  // can use that Option is None to print message in match statement

    // this code causes error since try to modify vector since mutable and immutable references in same scope
    //let first = &v_mut[0];
    //v_mut.push(9);
    //println!("The first element is: {first}");

    // get immutable references to each element in vector and print
    let v4 = vec![100, 32, 57];
    for n_ref in &v4 {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    println!("");  // printing line to separate

    // iterate over mutable references to each element to make changes
    let mut v5 = vec![100, 32, 57];
    for n_ref in &mut v5 {
        // n_ref has type &mut i32
        *n_ref += 50;
        println!("{}", *n_ref);
    }

    // create vector that holds different types using enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
