use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 39.99);
    menu.insert(String::from("Sushi"), 19.99);
    
    println!("{:#?}", menu);

    // let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    let mut country_capitals = HashMap::<&str, &str>::new();
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Korea", "Seoul");
    country_capitals.insert("Germany", "Berlin");

    println!("{:#?}", country_capitals);
}
