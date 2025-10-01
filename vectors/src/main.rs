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
    let cake_toppings = vec![pepperoni, mushroom, radish];

    let value = pancake_diameters[2];
    println!("pancake diameter: {value}");

    let reference = &cake_toppings[2];
    println!("reference: {reference}");

    let pancake_slice = &pancake_diameters[1..3];
    println!("{pancake_slice:?}");

    let cake_toppings_slice = &cake_toppings[1..];
    println!("{cake_toppings_slice:?}");
}
