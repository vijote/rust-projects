mod front_of_house {
    fn something() {
        super::eat_at_restaurant()
    }

    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            super::hosting::add_to_waitlist()
        }

        fn serve_order() {
            // Relative path with super
            super::something()
        }

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// when bringing functions into scope it's common to bring the entire module.
// so it's clear that is not a local function
use crate::front_of_house::hosting;
// using an alias with 'as'
use std::io::Result as IoResult;
// Nested use paths
use std::{io, cmp::Ordering};

// this brings std::io and std::io:Write into scope
use std::io::{self, Write};

// glob operator, brings all public items into scope
use std::collections::*;

mod another_mod {
    // when bringing a struct into scope it's better to include only the struct.
    use crate::back_of_house::Breakfast;

    fn this_doesnt_work() {
        // this doesn't work because hosting is included in an upper scope,
        // not this functions' scope
        hosting::eat_at_restaurant();
        Breakfast::summer("test");
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // with 'use' keyword
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}