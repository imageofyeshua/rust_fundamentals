#![allow(unused_variables)] // global compiler directive
const TAX_RATE: f64 = 7.25;  // constant declaration
type Meters = i32; // data type alias

fn main() {
    println!("The tax rate is {TAX_RATE}");

    let mile_race_length: Meters = 1600;

    println!("Race length: {mile_race_length}");

    #[allow(unused_variables)] // compiler directive
    let two_mile_race_length: Meters = 3200;

    let apples = 50;
    let oranges = 14 + 6;
    let fruits = apples + oranges;

    println!("My garden has {apples} apples and {oranges} oranges. {fruits} fruits");
    println!(
        "My garden has {} apples and {} oranges. {} fruits",
        apples, oranges, fruits
    );
    println!(
        "My garden has {0} apples and {1} oranges. {0} apples?",
        apples, oranges
    );

    let mut gym_reps = 10; // mutable variable
    println!("I plan to do {gym_reps} reps");

    gym_reps = 20;
    println!("I plan to do {gym_reps} reps now");

    let grams_of_protein = "100.345";
    let grams_of_protein = 123.456; // variable shadowing
    let grams_of_protein = 100; // variable shadowing again

    // scope of varaibles
    let coffee_price = 5.99;

    {
        let coffee_price = 1.99;
        println!("The coffee price: {coffee_price}");
    }

    println!("The coffee price: {coffee_price}");
}
