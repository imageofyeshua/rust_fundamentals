use std::io;

fn main() {
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
}
