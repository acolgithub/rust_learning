mod front_of_house {
    pub mod hosting {  // made public to be referrable by eat_at_restaurant
        pub fn add_to_waitlist() {}  // made public to be referrable by eat_at_restaurant

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

	fn take_payment() {}
    }
}

// we add "use" to shorten notation, made public as well since default is private
pub use crate::front_of_house::hosting;

// public function
pub fn eat_at_restaurant() {

    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // enum entries are public when enum is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


mod back_of_house {
    // create public breakfast struct
    pub struct Breakfast {
        pub toast: String,  // toast is public
        seasonal_fruit: String,  // fruit is private
    }

    // breakfast implementation
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // includes super to refer to parent module
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn deliver_order() {}

