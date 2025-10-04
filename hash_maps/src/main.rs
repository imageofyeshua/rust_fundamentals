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

    let data = [
        ("Bobby", 7),
        ("Grant", 4),
        ("Ben", 6),
    ];

    let mut years_at_company = HashMap::from(data);
    println!("{:#?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:#?}", years_at_company);
    println!("ben has worked here for {:?} years", ben.unwrap());

    // hashmap ownership
    let mut coffee_pairings = HashMap::<&str, &str>::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    println!("{}", coffee_pairings.len());
    println!("{drink} {milk}");

    // access value thru key
    coffee_pairings.insert("Flat White", "Almond Milk");
    // get() methods returns Option<&&str>, copied() method returns Option<&str>
    let value = coffee_pairings.get("Latte").copied().unwrap_or("Unknown");
    println!("{:?}", value);
}
