fn main() {
    let cake_diameters: Vec<i8> = Vec::new();
    let noodle_diameters = Vec::<i16>::new();
    let pasta = vec!["Rigatoni", "Angel hair", "Fettucine"];
    println!("{cake_diameters:?}");
    println!("{noodle_diameters:?}");
    println!("{pasta:?}");

    let mut pancake_diameters = vec![8, 10, 12, 15];
    pancake_diameters.push(16);
    pancake_diameters.push(19);
    pancake_diameters.insert(0, 4);

    let last_pancake_diameter = pancake_diameters.pop();
    let third_pancake_diameter = pancake_diameters.remove(2);

    println!("pancake diameter: {:?}", last_pancake_diameter.unwrap());
    println!("third pancake diameter: {:?}", third_pancake_diameter);
    println!("{pancake_diameters:?}");

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let radish = String::from("Radish");
    let mut cake_toppings = vec![pepperoni, mushroom, radish];

    let value = pancake_diameters[2];
    println!("pancake diameter: {value}");

    let reference = &cake_toppings[2];
    println!("reference: {reference}");

    let pancake_slice = &pancake_diameters[1..3];
    println!("{pancake_slice:?}");

    let cake_toppings_slice = &cake_toppings[1..];
    println!("{cake_toppings_slice:?}");

    let option = cake_toppings.get(2);

    match option {
        Some(value) => println!("The topping is {value}"),
        None => println!("There is nothing, man!")
    }

    cake_toppings[1] = String::from("Peach");
    println!("{cake_toppings:#?}");

    let target_topping = &mut cake_toppings[2];
    target_topping.push_str(" and Spaghetti");

    let another_topping = &cake_toppings[2];
    let yet_another_topping = &cake_toppings[2];
    println!("{cake_toppings:#?}");

    // ownership movement
    let mut delicious_toppings = cake_toppings;
    
    let topping_reference = &delicious_toppings[1];
    println!("The topping is {topping_reference}");
    
    delicious_toppings.push(String::from("Olives"));
    
    // vector capacity
    let mut seasons: Vec<&str> = Vec::with_capacity(4); 
    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());

    seasons.push("Spring");
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");

    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());

    // copy and enlarge original memory to different spot
    seasons.push("Summer");
    println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());
}
