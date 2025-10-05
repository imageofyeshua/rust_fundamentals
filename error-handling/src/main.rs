use std::process;
use std::fs::File;

fn main() {
    let file = match File::open("story.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong: {error:?}");
            process::exit(1);
        }
    };

    println!("{file:#?}");
    /*
    println!("Some status update");
    // std error with "$ cargo run > error.txt"
    eprintln!("Some error message");
    panic!("Something went wrong!");
    process::exit(1);
    println!("This will not print");
    */
}
