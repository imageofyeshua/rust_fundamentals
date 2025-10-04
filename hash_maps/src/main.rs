use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Bars"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot Bars", "Burgers", "Pretzels"]);
    println!("{:#?}", sauces_to_meals);

    println!("{:#?}", sauces_to_meals.remove("Mayonnaise").unwrap());

    let mustard_meals = sauces_to_meals.get("Mustard");
    println!("{mustard_meals:#?}");

    match mustard_meals {
        Some(meal) => println!("The meals were {meal:#?}"),
        None => println!("There were no meals for that sauce!")
    }

    sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);
    println!("{sauces_to_meals:#?}");
}
