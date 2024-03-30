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

fn main() {
    
    // match statement that only matches one pattern
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // rewrite match statement using if-let
    if let Some(max) = config_max {  // binds max to expression inside Some from config_max
        println!("The maximum is configured to be {}", max);
    }

    // count non-quarters and announce state of quarters
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // using if-let to do the same
    let coin2 = Coin::Dime;
    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
