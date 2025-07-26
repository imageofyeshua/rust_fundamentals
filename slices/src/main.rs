fn main() {
    // slices
    let bible_hero = String::from("Joshua Park");
    let string_reference = &bible_hero;
    println!("{string_reference}");

    let first_name = &bible_hero[0..6]; // byte range
    let last_name = &bible_hero[7..11]; // each byte represents a character
    println!("{first_name} {last_name}");
}
