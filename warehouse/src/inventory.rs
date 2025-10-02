pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

pub mod products;

pub fn talk_to_manager() {
    println!("Hey, {}, how's your coffee?", MANAGER);
    // println!("{:?}", products::ProductCategory::Ladder);
}


