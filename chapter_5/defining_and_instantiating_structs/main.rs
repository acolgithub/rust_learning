fn main() {
    // create instance of user type
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // get email value from user type
    let user1_email = user1.email;
    println!("{user1_email}");

    // create mutable instance of user type
    let mut user1_mut = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // assign to email value from user type
    user1_mut.email = String::from("anotheremail@example.com");

    // create new user using old instance
    let user2 = User {
        active: user1.active,
        username:user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // create new user using struct update syntax
    let user2_update = User {
        email: String::from("another@example.com"),
        ..user1_mut
    };

    // new tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // create instance of unit struct
    let subject = AlwaysEqual;

    // try to create structure with no lifetime (this code results in error)
    //let user1_no_lifetime = User {
    //    active: true,
    //    username: "someusername123",
    //    email: "someone@example.com",
    //    sign_in_count: 1,
    //};
}

// user structure
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// function to build user and retrun user type
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// function to build user using field init shorthand
fn build_user_init(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// define unit struct
struct AlwaysEqual;

// structure with references and no lifetimes
//struct User_no_lifetime {
//    active: bool,
//    username: &str,
//    email: &str,
//    sign_in_count: u64,
//}

