// enum representing different US states
#[derive(Debug)]  // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// enum representing various coins
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");  // print lucky penny
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);  // print state quarter comes from
            25
        },
    }
}


fn main() {

    // test value in cents function, should print state and value of quarter
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // simulation of dice roll
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // simulation with modified rules and catchall
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

}

// add one if non-empty
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

// this code causes errors since does not cover all cases
//fn plus_one_error(x: Option<i32) -> Option<i32> {
//    match x {
//        Some(i) => Some(i+1),
//    }
//}


// game implementation
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}


