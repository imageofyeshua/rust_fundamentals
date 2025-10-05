use std::io::{self, Write};

struct Vector {
    elem: Box<[f64]>, // pointer to the elements
    sz: usize,        // the number of elements
}

impl Vector {
    // Constructor to create a new Vector
    fn new(s: usize) -> Vector {
        Vector {
            elem: vec![0.0; s].into_boxed_slice(), // Allocate elements on the heap
            sz: s,
        }
    }

    fn get(&self, i: usize) -> f64 {
        self.elem[i]
    }

    fn set(&mut self, i:usize, value: f64) {
        self.elem[i] = value;
    }

    fn size(&self) -> usize {
        self.sz
    }
}

fn read_and_sum(s: usize) -> f64 {
    let mut v = Vector::new(s);

    for i in 0..v.size() {
        print!("Enter number {}: ", i + 1);
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        v.set(i, input.trim().parse().expect("Please enter a number"));
    }

    let mut sum = 0.0;
    for i in 0..v.size() {
        sum += v.get(i);
    }
    sum
}

fn main() {
    let mut vec = Vector::new(6);

    for i in 0..vec.size() {
        vec.set(i, i as f64);
    }

    for i in 0..vec.size() {
        println!("vec[{}] = {}", i, vec.get(i));
    }

    let s = 5;
    println!("You will enter {} numbers:", s);
    let sum = read_and_sum(s);
    println!("The sum of the entered numbers is: {}", sum);
}
