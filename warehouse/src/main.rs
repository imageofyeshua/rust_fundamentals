#![allow(dead_code)]

mod inventory;
mod orders;

use inventory::products::{self, ProductCategory};
use inventory::{talk_to_manager, FLOOR_SPACE};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );

    talk_to_manager();

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tall_ladder = products::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
}
