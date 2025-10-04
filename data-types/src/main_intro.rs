use std::io::{self, Write};

const DMV: i32 = 17;

const fn square(x: i32) -> i32 {
    x * x
}

const MAX1: f64 = 1.4 * square(DMV) as f64; // OK if square(17) is a constant expression

fn main() {
    let var: i32 = 17;
    let v = vec![1.2, 3.4, 4.5];
    let s1 = sum(&v);

    println!("DMV: {}", DMV);
    println!("var: {}", var);
    println!("MAX1: {}", MAX1);
    println!("s1: {}", s1);

    if accept() {
        println!("Proceeding...");
    } else {
        println!("Operation cancelled");
    }

    // some_function();
    // data_types();
}

fn accept() -> bool {
    let mut tries = 1;
    while tries <= 3 {
        print!("Do you want to proceed (y or n)? ");
        io::stdout().flush().unwrap(); // ensure the question is printed immediately
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        match answer.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => {
                println!("Sorry, I don't understand that.");
                tries += 1; // increment
            }
        }
    }
    println!("I'll take that for a no.");
    false
}

fn sum(v: &Vec<f64>) -> f64 {
    v.iter().sum()
}

fn data_types() {
    /*
    x + y  // addition
    +x     // unary plus (rarely used)
    x - y  // subtraction
    -x     // unary minus
    x * y  // multiplication
    x / y  // division
    x % y  // remainder (modulus) for integers
    x == y  // equal
    x != y  // not equal
    x < y   // less than
    x > y   // greater than
    x <= y  // less than or equal to
    x >= y  // greater than or equal to
    */

    let d1: f64 = 2.3;
    let d2: f64 = 2.3;

    let z = (1.0, 0.0);
    let z2 = (d1, d2);
    let z3 = (1.0, 2.0);

    let v = vec![1, 2, 3, 4, 5, 6];

    println!("z: {}+{}i", z.0, z.1);
    println!("z2: {}+{}i", z2.0, z2.1);
    println!("z3: {}+{}i", z3.0, z3.1);
    println!("v: {:?}", v);

    let i1: i32 = 7.2 as i32;
    let i3: i32 = {7.2 as i32};

    println!("i1: {}", i1);
    println!("i3: {}", i3);

    let b = true;
    let ch = 'x';
    let i = 123;
    let d = 1.2;
    let y: f64 = 4.0;
    let z = y.sqrt();

    println!("b: {}", b);
    println!("ch: {}", ch);
    println!("i: {}", i);
    println!("d: {}", d);
    println!("y: {}", y);
    println!("z: {}", z);
}

fn some_function() {
    let mut d: f64 = 2.2;
    let mut i: i32 = 7;
    d += i as f64;
    println!("d after addtion: {}", d);
    i = (d * i as f64) as i32;
    println!("i after multiplication: {}", i);
}
