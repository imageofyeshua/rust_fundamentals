#![allow(dead_code)]

mod inventory;
mod orders;

use inventory::{Item, ProductCategory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
use orders::MANAGER as ORDERS_MANAGER; 

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tall_ladder = Item::new(String::from("O-matic-Ladder 2030"), favorite_category, 100);
    println!("{:#?}", tall_ladder);
}
