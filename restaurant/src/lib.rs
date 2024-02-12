/**
 * front_of_house and eat_at_restaurant are siblings so that's
 * why we don't have to add pub to front_of_house in order to
 * be accessed by eat_at_restaurant.
 * */ 

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist(); -> without using use keyword
    hosting::add_to_waitlist();

    // relative path
    // front_of_house::hosting::add_to_waitlist(); -> without using use keyword
    hosting::add_to_waitlist();

    // Order breakfast with Rye
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change breakfast toast to Wheat
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); -> not allowed because it's a private field

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    fn _fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        } 
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}