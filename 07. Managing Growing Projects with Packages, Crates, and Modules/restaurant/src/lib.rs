/// `src/main.rs` and `src/lib.rs` are called crate roots.
/// Either of them form a module named **crate** at the root of the module tree.
mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// pub use crate::front_of_house::hosting;
use crate::front_of_house::hosting;
pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use self::front_of_house::serving::serve_order;
mod back_of_house;

pub fn eat_breakfast() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
}
