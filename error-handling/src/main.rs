use std::process;
use std::io::{self, stdin, Read};
use std::fs::File;

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

    let mut file = File::open(input.trim())?;

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    
    Ok(file_contents)
}
