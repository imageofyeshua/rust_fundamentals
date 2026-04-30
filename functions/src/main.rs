fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x is: {}", x);

    let signed_int: i32 = -42;
    let unsigned_int: u32 = 42;
    println!("Signed integer: {}, Unsigned integer: {}", signed_int, unsigned_int);

    let float_num: f64 = 5.14159;
    println!("Floating-point number: {}", float_num);

    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);

    let letter: char = 'R';
    let emoji: char = '😀';

    println!("Letter: {}, Emoji: {}", letter, emoji);

    // A tuple holding an integer, a float, and a character
    let my_tuple: (i32, f64, char) = (500, 6.4, 'R');

    // Method 1: Destructuring with a `let` binding.
    // This is a form of pattern matching that breaks the tuple into separate variables.
    let (x, y, z) = my_tuple;
    println!("Destructured values: x = {}, y = {}, z = {}", x, y, z);

    // Method 2: Direct access using dot notation and the element's index.
    // Indices start from 0.
    let first_element = my_tuple.0;
    let second_element = my_tuple.1;
    println!("Direct access: First element is {}, second is {}", first_element, second_element);

    let input1 = 10;
    let input2 = 5;

    let (sum_result, product_result) = calculate_sum_and_product(input1, input2);

    println!("For {} and {}", input1, input2);
    println!("  Sum: {}", sum_result);
    println!("  Product: {}", product_result);
}

fn calculate_sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}
