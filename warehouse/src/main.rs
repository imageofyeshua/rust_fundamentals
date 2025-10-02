#![allow(dead_code)]

mod inventory;
mod orders;

use inventory::products::{Item, ProductCategory};
use inventory::FLOOR_SPACE;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tall_ladder = Item::new(String::from("O-matic-Ladder 2030"), favorite_category, 100);
    println!("{:#?}", tall_ladder);
}
