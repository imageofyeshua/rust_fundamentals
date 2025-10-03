use std::io;

fn main() {
    /*
    let pirate = "BloodyHook";
    let sailer = String::from(pirate);
    let bad_guy = pirate.to_string();
    
    println!("{pirate} and {sailer} and {bad_guy}");

    let first_initial = &pirate[0..1];
    println!("{first_initial}");

    let mut full_name = String::from("Daniel");
    let last_name = "Park";
    full_name.push(' ');
    full_name.push_str(last_name);
    println!("{full_name}");

    let middle_name = String::from("John");
    full_name.push(' ');
    full_name.push_str(&middle_name); // &String -> &str
    println!("{full_name}");

    // + (add) moves ownership, thus use clone() method
    let your_name = middle_name.clone() + last_name;
    println!("{your_name}");
    println!("{middle_name}");
    println!("{last_name}");

    // format! macro
    let icon = format!("{middle_name} {last_name}");
    let model = format!("{0} {1} {0} {1}", middle_name, last_name);
    println!("{icon}");
    println!("{model}");

    // common string methods
    let mut music_genres = "    Gospel, Country, Rap    ";
    println!("{}", music_genres.trim());
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("a", "@"));

    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:#?}", genres);

    let mut name = String::new();
    println!("What is your name");
    // io::stdin().read_line(&mut name).expect("Failed to collect input from the user");
    // trim() method removes carriage return '\n'
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name.trim()),
        Err(message) => println!("There was an error: {message}")
    }
    */

    // exercise
    let mut amount = String::from("40");
    make_money(&mut amount);
    println!("{amount}");

    let banana = trim_and_capitalize("     banana    ");
    println!("{banana}");

    let collection = elements("Gold!Silver!Platinum");
    println!("{:#?}", collection);

    let full_name = get_identity();
    println!("{full_name}");
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();
    let input = io::stdin();

    println!("What is your first name?");
    input.read_line(&mut first_name).expect("Wrong input!");
    println!("What is your last name?");
    input.read_line(&mut last_name).expect("Wrong input!");

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect::<Vec<&str>>()
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn make_money(text: &mut String) {
    text.push_str("$$$");
}

