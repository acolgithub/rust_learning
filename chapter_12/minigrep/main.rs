// need args function to return iterator of command line arguments passed to minigrep
use std::env;

// added process function in order to specify error state
use std::process;

// add Config from library crate
use minigrep::Config;

fn main() {
    // form vector of arguments passed to minigrep
    // note that args will panic if argument contains invalid Unicode, use args_os if such arguments are necessary
    let args: Vec<String> = env::args().collect();

    // store query and file path through config object
    // used iterator obtained through env::args()
    let config = Config::build(env::args()).unwrap_or_else(|err| {

        // return error message if not enough arguments
        // used eprintln to print to std err
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // print arguments to test
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // get file contents
    if let Err(e) = minigrep::run(config) {

        // in the event of error return print error message and exit
        // used eprintln to print to std err
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
