// create struct that holds references and hence needs lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    {
        let string3 = String::from("wxy");
        let result2 = longest(string1.as_str(), string3.as_str());
        println!("The longest string is {}", result2);
    }

    // this code causes errors since lifetime of string4 ends before println! is executed
    //{
    //    let string4 = String::from("xyz");
    //    result = longest(string1.as_str(), string4.as_str());
    //}
    //println!("THe longest string is {}", result);

    // this code causes errors since lifetime of string1 ends before the following println! is executed on result
    //let string5 = string1;
    //println!("The longest string is {}", result);  // lifetime os string1 ends before result is used here
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetimes last for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";  // string is stored directly in progam's binary
}

// create function which specifies that result lives at least as long as input variables
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // when we pass references 'a becomes the smaller of the two lifetime lengths from inputs
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// function that uses all ideas from chapter
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);  // print announcement, needs Display trait
    if x.len() > y.len() {  // returns longer of two strings
        x
    } else {
        y
    }
}

