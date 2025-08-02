use std::io;

#[allow(unused_variables)]
fn main() {
    let eight_bit: i8 = -112;
    let eight_bit_unsigned: u8 = 112;

    let sixteen_bit_signed: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 64_000;

    let thirty_two_bit_signed: i32 = -2147383648;
    let thirty_two_bit_unsigned: u32 = 4_294_967_295; // readable format

    let some_value: u16 = 20u16;

    println!("Some value: {some_value}");

    // usize and isize

    let days: usize = 55; // system default size for unsigned integer
    let years: isize = -15_000; // system default size for signed integer

    println!("Dear Emily, \nHow have you  been?");
    println!("\tOnce upon a time");
    println!("Juliet said \"I love you Romeo\"");
    let filepath = r"C:\My Documents\new\videos"; // r stands for raw
    println!("{filepath}");

    let value = -15;
    // let value_abs = (value as i32).abs();
    println!("{}", (value as i8).abs());

    let empty_space = "       my content      ";
    println!("{}", empty_space.trim());

    println!("{}", (value as i32).pow(2));
    println!("{}", (value as i32).pow(3));

    // type casting
    let pi = 3.141592653873452985;
    println!("The current value of pi is {pi:.4}");
    println!("The current value of pi is {:.2}", pi);

    println!("{}", (pi as f64).floor());
    println!("{}", (pi as f64).ceil());
    println!("{}", (pi as f64).round());

    let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;

    let miles_away = 100.329032;
    let miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;

    // math operations
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 4;
    println!("Addition: {addition}, subtraction: {subtraction}, multiplication: {multiplication}");

    let floor_division = 5 / 3;
    println!("floor division: {floor_division}");

    let decimal_division = 5.0 / 3.0;
    println!("decimal division: {decimal_division}");

    let remainder = 7 % 3;
    println!("remainder: {remainder}");

    let mut year = 2025;
    year += 1; // same as year = year + 1
    println!("The new year is {year}");
    year -= 5; // same as year = year + 1
    println!("5 years before: {year}");
    year *= 2; // same as year = year + 1
    println!("year multiplied: {year}");
    year /= 3; // same as year = year + 1
    println!("year divided: {year}");

    // boolean
    let is_handsome = true;
    let is_silly = false;
    println!("handsome: {is_handsome}");

    let age = 21;
    let is_young = age < 35;
    println!("I am young: {is_young}");
    println!(
        "positive? {}, negative? {}",
        (age as i32).is_positive(),
        (age as i32).is_negative()
    );

    println!("inversion: {}", !true);

    // equality
    println!("{}", 13 == 13.0 as i32);
    println!("{}", "Daniel" == "daniel");
    println!("{}", 24.1 != 24.13);
    println!("{}", true != false);

    // && || operator
    let purchased_ticket = true;
    let plane_on_time = false;
    let making_event = purchased_ticket && plane_on_time;
    let chance_event = purchased_ticket || plane_on_time;
    println!("He will come soon: {making_event}");
    println!("We don't know yet: {chance_event}");

    // character type
    let first_initial = 'D'; // 4-bytes
    let example = "Daniel";
    let emoji: char = '👌'; // emoji key: mac (fn + e) win (win + .)

    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );

    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());
    println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase());

    // array
    let numbers: [i32; 4] = [4, 6, 9, 10];
    let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("array length: {}", apples.len());

    let currency_rates: [f64; 0] = []; // empty box for array
    let mut seasons = ["Spring", "Summer", "Autumn", "Winter"];

    let first = seasons[0];
    let second = seasons[1];

    seasons[2] = "Fall";

    println!("The first season is {first} and the second season is {second}");
    println!("The Autumn changed to {}", seasons[2]);

    // display trait
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);
    println!("{:?}", seasons); // array implements debug trait to show its contents
    println!("{seasons:?}"); // same as above
    println!("{seasons:#?}"); // pretty print

    // debug macro
    /*
    dbg!(2 + 2);
    dbg!(seasons);
     */

    // tuple
    let employee = ("Molly", 32, "Marketing");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    /*
    let name = employee.0;
    let age = employee.1;
    let department = employee.2;
    */
    let (name, age, department) = employee;

    println!("Name: {name}, age: {age}, department: {department}");
    println!("{employee:#?}");

    // range
    let month_days = 1..31; // exclude the last
    println!("{month_days:?}");
    let week_days = 1..=7; // include the last
    println!("{week_days:?}");

    for number in month_days {
        println!("{}", number * 2);
    }

    let letters = 'b'..'h';
    println!("{letters:#?}");
    for letter in letters {
        println!("{letter}");
    }

    // array
    let colors = ["Red", "Green", "Blue"];

    for color in colors {
        println!("{color} is a great color!");
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("array a: {a:?}");

    // generic
    let year_days: std::ops::Range<i16> = 1..365;
    let alphabet: std::ops::Range<char> = 'a'..'f';

    // parse
    let guess: u32 = "32".parse().expect("Not a number");
    println!("Guess: {guess}");

    /*
    let myarr = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = myarr[index];

    println!("The value of the element at index {index} is: {element}");
    */

    // scalar types : integers, floating-point numbers, booleans, and characters
    // - integer types : i8 / u8, i16 / u16, i32 / u32, i64 / u64, i128 / u128, isize / usize
    /* - integer literals example
            Decimal        : 98_222
            Hex            : 0xff
            Octal          : 0o77
            Binary         : 0b1111_0000
            Byte (u8 only) : b'A'
    */
    // - floating point types : f64, f32
    // - boolean types : true, false

    // compound types : tuples and arrays

    print_labeled_measurement(5, 'h');

    let x = plus_one(5);
    println!("The value of x is {x}");
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}
