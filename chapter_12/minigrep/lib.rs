// need to be able to read a file
use std::fs;

// added to have error messages
use std::error::Error;

// added to work with environment variables
use std::env;

// struct to group together the query and file path
// everything is public so that we can use it
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
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

        // check if environment variable named IGNORE_CASE is set and assign to ignore_case
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        // if user specifies any argument in args for case then override environment variable
        ignore_case = if args.len() > 3 {args[3].len() > 0} else {ignore_case};

        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }
}

// run function gets contents from file
// error messages return a type that implements Error trait
// made public so that we can use it
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    // take file, open it, and return string of the file contents
    let contents = fs::read_to_string(config.file_path)?;

    // initialize results with case sensitive or case insensitive search depending on case configuration
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // print contents to test
    // println!("With text:\n{contents}");

    // print each line where query is found
    for line in results {
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

// create case insensitive search function
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {

    // convert query to lowercase
    // note that Rust create new String as return
    let query = query.to_lowercase();

    // create vector to store matches
    let mut results = Vec::new();

    // check each line for math
    for line in contents.lines() {

        // check if line made lowercase contains query
        // note that an ampersand is needed since query is String but contains take string slice
        if line.to_lowercase().contains(&query) {

            // if there is a match add line to results
            results.push(line);
        }
    }

    // return results
    results
}

// run tests on search function
#[cfg(test)]
mod tests {
    use super::*;

    // test basic functionality of search function with case sensitivity
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // test case insensitive feature
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
