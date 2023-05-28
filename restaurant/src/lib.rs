mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
    }

    //     mod serving {
    //         fn take_order() {}
    //
    //         fn serve_order() {}
    //
    //         fn take_payment() {}
    //     }
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
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn seasonal_fruit() {
            let meal = Self::summer("Plain with seasonal fruit");

            println!("{}", meal.seasonal_fruit);
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    back_of_house::Breakfast::seasonal_fruit();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// fn deliver_order() {}
//
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }
//
//     fn cook_order() {}
// }