use std::io::{self, stdin};
use std::fs;

fn main() {
    let file_result = read_file();

    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}");
        }
    }
    /*
    println!("Some status update");
    // std error with "$ cargo run > error.txt"
    eprintln!("Some error message");
    panic!("Something went wrong!");
    process::exit(1);
    println!("This will not print");
    */
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file:");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())

    // above fs read_to_string function does below exactly
    /*
    let mut file_contents = String::new();
    File::open(input.trim())?.read_to_string(&mut file_contents)?;
    
    Ok(file_contents)
    */
}
