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

    println!("{pancake_diameters:?}");
    println!("pancake diameter: {:?}", last_pancake_diameter.unwrap());
    println!("third pancake diameter: {:?}", third_pancake_diameter);
}
