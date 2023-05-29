mod front_of_house;
mod back_of_house;

use crate::front_of_house::*;
use crate::back_of_house::*;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    Breakfast::seasonal_fruit();

    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = Appetizer::Soup;
    let _order2 = Appetizer::Salad;
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

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
//
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//
//         pub fn seasonal_fruit() {
//             let meal = Self::summer("Plain with seasonal fruit");
//
//             println!("{}", meal.seasonal_fruit);
//         }
//     }
// }

