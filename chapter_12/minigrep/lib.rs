// need to be able to read a file
use std::fs;

// added to have error messages
use std::error::Error;

// struct to group together the query and file path
// everything is public so that we can use it
pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {

    // create instance of Config with provided arguments
    // made pub so that we can use it
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        // check that slice is long enough before accessing
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

// run function gets contents from file
// error messages return a type that implements Error trait
// made public so that we can use it
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    // take file, open it, and return string of the file contents
    let contents = fs::read_to_string(config.file_path)?;

    // print contents to test
    println!("With text:\n{contents}");

    // print each line where query is found
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())

}

// create search function
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    // vector to store lines containing query
    let mut results = Vec::new();

    // iterate over lines
    for line in contents.lines() {

        // if line contains query add to vector
        if line.contains(query) {
            results.push(line);
        }
    }

    // return results
    results
}

// run test on search function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
