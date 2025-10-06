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

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        return Err(error);
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        return Err(error);
    }
    
    Ok(file_contents)
}
