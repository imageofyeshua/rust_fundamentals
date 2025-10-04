use std::collections::HashMap;
use std::collections::HashSet;

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

    // overwriting a value with an existing key
    coffee_pairings.insert("Latte", "Pistachio Milk");
    println!("{:#?}", coffee_pairings);

    // entry method
    coffee_pairings.entry("Latte").or_insert("Cow Milk");
    println!("{coffee_pairings:#?}");

    coffee_pairings.entry("Cappuccino").or_insert("Hazlnut Milk");
    println!("{coffee_pairings:#?}");

    // hashset >> a collection type that stores unique values
    let mut concert_queue = HashSet::<&str>::new();
    println!("{:?}", concert_queue);

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("{:?}", concert_queue);
    println!("{}", concert_queue.len());

    println!("{}", concert_queue.remove("Megan"));
    println!("{:?}", concert_queue);

    println!("{}", concert_queue.contains("Molly"));
    println!("{}", concert_queue.contains("Fred"));

    println!("{:?}", concert_queue.get("Molly"));
    println!("{:?}", concert_queue.get("Joe"));

    // hashset operations
    let mut movie_queue = HashSet::<&str>::new();
    let mut concert_queue = HashSet::<&str>::new();

    concert_queue.insert("Daniel");
    concert_queue.insert("Jason");

    movie_queue.insert("Daniel");
    movie_queue.insert("Issac");

    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    println!("{:?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:?}", movie_queue.is_disjoint(&concert_queue));

    println!("{:?}", concert_queue.is_subset(&movie_queue));
    println!("{:?}", concert_queue.is_superset(&movie_queue));
}
