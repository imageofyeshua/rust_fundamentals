#![allow(dead_code)]

use fake::{Fake, Faker};

use warehouse::{Item, ProductCategory, FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        FLOOR_SPACE
    );

    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);

    /*
    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tall_ladder = Item::new(String::from("O-matic-Ladder 2030"), favorite_category, 100);
    println!("{:#?}", tall_ladder);
    */
}
