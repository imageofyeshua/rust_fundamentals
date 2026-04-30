use std::io;

fn main() {
    println!("Welcome to the Rust CLI Calculator!");

    let num1 = get_number("Enter the first number: ");
    let num2 = get_number("Enter the second number: ");

    println!("Enter an operation (+, -, *, /): ");

    // Create a mutable string to store the user's input
    let mut operation = String::new();

    // Read the user's input and store it in `operation`
    io::stdin().read_line(&mut operation).expect("Failed to read input");

    // Trim any whitespace (e.g., newline) from the input
    let operation = operation.trim();

    // Perform the calcuation based on the chosen operation
    let result = match operation {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            // Before dividing, check that the second number is not zero
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                println!("Error: Division by zero is not allowed.");
                None
            } 
        }
        _ => {
            // If the user enters an invalid operation, print an error message
            println!("Invalid operation. Please enter +, -, *, or /.");
            None
        }
    };

    // If the result is valid (not None), print the result
    if let Some(res) = result {
        println!("Result: {}", res);
    }
}

fn get_number(prompt: &str) -> f64 {
    loop { // Infinite loop until valid input is provided
        println!("{}", prompt); // Display the prompt message
         
        let mut input = String::new(); // Create a new mutable string for user input
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num, // If parsing succeeds, return the number
            Err(_) => println!("Invalid number. Please enter a valid numeric value."),
        }
    }
}
