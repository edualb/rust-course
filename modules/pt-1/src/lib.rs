mod front_of_house {
    fn do_something(){}
    pub mod hosting {
        pub fn add_to_waitlist(){
            crate::eat_at_restaurant();
            super::do_something();
        }
    }
}


fn eat_at_restaurant() {
    // Absolute Paths
    
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Paths
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("Wheat"); -> We cant do that
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup, // default is public
        Salad,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}