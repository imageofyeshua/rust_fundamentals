use std::process;
use std::io::{stdin, Read};
use std::fs::File;

fn main() {
    println!("Please enter the name of the file:");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong: {error:?}");
        process::exit(1)
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong opening: {error:?}");
            process::exit(1);
        }
    };

    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        eprintln!("Something went wrong reading: {error}");
        process::exit(1);
    }

    println!("{file_contents}");
    /*
    println!("Some status update");
    // std error with "$ cargo run > error.txt"
    eprintln!("Some error message");
    panic!("Something went wrong!");
    process::exit(1);
    println!("This will not print");
    */
}
