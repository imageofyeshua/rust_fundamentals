#[allow(unused_variables)]
fn main() {
    open_store("Gimcheon");
    bake_bread(12, "beans");
    bake_bread(7, "mushroom");

    let result = square(5);
    println!("Square result: {result}");

    let outcome = cube(4);
    println!("Cube result: {outcome}");

    // unit
    let achievements = mystery();

    let multiplier = 3;

    // block
    let calculation = {
       let value = 5 + 4;
       value * multiplier
    };

    println!("{calculation}");

    println!("4 is even? {}", is_even(4));

    println!("contains 'a' || 'z' ? {:#?}", alphabets("hallo there"));
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn mystery() {
    println!("Hello there!")
}

fn open_store(neighborhood: &str) {
    println!("Opening my pancake store in {neighborhood}");
}

fn bake_bread(number: i32, fillings: &str) {
    println!("Baking {number} loaves of {fillings} bread");
}

// explicit return
fn square(number: i32) -> i32 {
    return number * number;
}

// implicit return : no semicolon and the last data is returned
fn cube(number: i32) -> i32 {
    number * number * number
}