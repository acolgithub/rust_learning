// use statements
use std::time::Duration;
use std::thread;


#[derive(Debug, PartialEq, Copy, Clone)]
// shirt colors available
enum ShirtColor {
    Red,
    Blue,
}

// shirt colors in stock
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // method representing giving away a shirt
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // unwrap_or_else takes a closure as an argument
        user_preference.unwrap_or_else(|| self.most_stocked())  // if no user preference use most stocked
    }

    // method representing most stocked shirt
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        // determine current inventory
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

            // return the color with more inventory
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
    }
}


#[derive(Debug)]
// create rectangle struct instance
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    // store with 2 blue shirts and one red shirt
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    // first user has preference for red shirt
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    // second user has no preference
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    println!("");

    // closure with annotated types to resemble functions
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // comparison of funcions and closures
    fn add_one_v1    (x: u32) -> u32 {x+1}
    let add_one_v2 = |x: u32| -> u32 {x+1};

    // these versions are valid definitions but need to be evaluated to be able to compile, types inferred from usage
    //let add_one_v3 = |x|             {x+1};
    //let add_one_v4 = |x|              x+1 ;

    // example closure with inferred type
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5);  // this code will cause errors since type was already inferred

    // define vector
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // define closure that borrows
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure {:?}", list);
    only_borrows();  // calls closure as if it were a function
    println!("After calling closure: {:?}", list);  // list is still accessible

    println!("");

    // define mutable vector
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // define closure that modifies
    let mut borrows_mutably = || list.push(7);

    // use closure and print result
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    println!("");

    // define immutable list
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // define closure that takes ownership using "move" keyword
    thread::spawn(move || println!("From thread: {:?}", list))  // need move since main thread may finish earlier and invalidate the reference
        .join()
        .unwrap();


    println!("");


    // create list of Rectangle instances
    let mut list = [
        Rectangle { width: 10, height:1 },
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];

    // sort by key
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // define number of sort operations
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);


    // closure designed to make a clone of a string
    // inform Rust that closure returned must not live longer than input
    fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
        move || s_ref.to_string()
    }
}
