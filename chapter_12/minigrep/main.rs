// need args function to return iterator of command line arguments passed to minigrep
use std::env;

// need to be able to read a file
use std::fs;

fn main() {
    // form vector of arguments passed to minigrep
    // note that args will panic if argument contains invalid Unicode, use args_os if such arguments are necessary
    let args: Vec<String> = env::args().collect();

    // dbg!(args);  // print the vector using debug macro

    // store arguments that do not correspond to binary name
    let query = &args[1];
    let file_path = &args[2];

    // print arguments to test
    println!("Searching for {query}");
    println!("In file {file_path}");

    // take file, open it, and return string of the file contents
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // print contents to test
    println!("With text:\n{contents}");
}
