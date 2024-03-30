// use statements
use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    
    // return Result value because the function could fail
    let greeting_file_result = File::open("hello.txt");  // this should fail since no file exists, return type of File::open is Result

    // handle the Result value using match
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {  // match by type of error
            ErrorKind::NotFound => match File::create("hello.txt") {  // check if error is file not found and then try to create file
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),  // new panic if file creation failed
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);  // panic if different type of error
            },
        }
    };

    // separate way to handle the above using closures
    let greeting_file2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap returns value inside Ok variant and panics at Err variant
    let greeting_file3 = File::open("hello.txt").unwrap();

    // use expect method to choose panic message
    let greeting_file4 = File::open("hello.txt")
        .expect("hello.txt should be included in this project");  // detailed panic message
}


// function that tries to read a username from a file, also chooses how to handle error
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");  // try to open file

    let mut username_file = match username_file_result {
        Ok(file) => file,  // if file opened then get file name
        Err(e) => return Err(e),  // else return error
    };

    let mut username = String::new();  // create new string to hold username

    match username_file.read_to_string(&mut username) {  // read username from file
        Ok(_) => Ok(username),  // if okay return Ok variant with username
        Err(e) => Err(e),  // else return error if read to string fails
    }
}

// same functionality as above propagation of errors function but with ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;  // if error return error value
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;  // if error return error value
    Ok(username)
}

// short alternative of previous read functions
fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

