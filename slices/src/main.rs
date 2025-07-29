fn main() {
    // slices
    // let bible_hero = String::from("Joshua Park"); // heap allocated string
    let bible_hero = "Joshua Park"; // memory in executable file
    let string_reference = &bible_hero;
    println!("{string_reference}");

    let first_name = &bible_hero[0..6]; // byte range same as &bile_hero[..6]
    let last_name = &bible_hero[7..11]; // each byte represents a character
    println!("{first_name} {last_name}");

    let whose_name = {
        let betrayer = "Judas Iscariot";
        &betrayer[0..5] // creates separate reference and return it >> no dangling reference 
    };
    println!("{whose_name}");

    let food = "apricot";
    println!("length: {}", food.len());
    let fruit_slice = &food[0..3];
    println!("length: {}", fruit_slice.len());
    let full_name = &food[..];
    println!("full name: {}", full_name);
    let smile = "😊";
    println!("emoji length: {}", smile.len()); // emoji occupies 4-bytes which cannot be sliced

    let action_hero = String::from("Brad Pitt");
    do_hero_stuff(&action_hero); // &String -> &str : OK however, &str -> &String : NG
    let another_action_hero = "King Slayer";
    do_hero_stuff(another_action_hero);

    let values = [4, 8, 15, 16, 23, 42];

    let my_slice = &values[0..3];
    println!("{my_slice:?}");

    let my_slice = &values[..];
    println!("{my_slice:?}");

    let my_slice = &values; // borrowing full reference
    println!("{my_slice:?}");

    let regular_reference = &values;
    print_length(regular_reference);

    let slice_of_three = &values[..3];
    print_length(slice_of_three);
}

fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day!");
}
