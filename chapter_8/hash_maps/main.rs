// use statements
use std::collections::HashMap;

fn main() {
    
    // create empty hash map
    let mut scores = HashMap::new();

    // add elements to hash map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get values of hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);  //copied gets Option<i32> instead of Opion<&i32>, unwrap_or gets score or sets to zero if no entry
    println!("{}", score);

    // iterate over keys/value pairs
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // define field name and value
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    // define hash map
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    //println!("{}", field_name);  // field name and value have been moved and ownership has been transferred
    //println!("{}", field_value);

    // overwriting a value in hash map
    let mut new_scores = HashMap::new();

    // insert key/value then overwrite
    new_scores.insert(String::from("Blue"), 10);
    new_scores.insert(String::from("Blue"), 25);

    println!("{:?}", new_scores);

    // add a key and value only if a key is not present
    let mut check_present_scores = HashMap::new();
    check_present_scores.insert(String::from("Blue"), 10);

    // check if entry is present and insert 50 if key does not have value
    check_present_scores.entry(String::from("Yellow")).or_insert(50);
    check_present_scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", check_present_scores);

    // update value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // loop to keep track of how many times word is seen
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // get mutable reference to value of 0 if key does not appear otherwise get mutable reference to current value
        *count += 1;  // increment value on key
    }

    println!("{:?}", map);
}
