fn main() {
    // unsafe_pointer();
    // copy_fct();
    // print_arrays();
    increment();
}

fn increment() {
    let mut v = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for x in &mut v {
        *x += 5;
    }

    // Printing the modified array to verify the result
    for x in &v {
        println!("{}", x);
    }
}

fn print_arrays() {
    let v = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Iterating over the elements of v
    for x in v.iter() {
        println!("{}", x);
    }

    // Iterating over a slice of a values
    for x in &[10, 21, 32, 43, 54, 65] {
        println!("{}", x);
    }
}

fn copy_fct() {
    let v1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut v2 = [0; 10];
    v2.copy_from_slice(&v1);
    /* same as above
    for i in 0..10 {
        v2[i] = v1[i];
    }
    */

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}

fn unsafe_pointer() {
    let b: [char; 6] = ['0', '1', '2', '3', '4', '5'];
    let a: *const char = &b[3]; // a points to b's fourth element (index 3)
    
    // Dereference a to get the object it points to
    let x: char = unsafe { *a };

    println!("The character at index 3 is: {}", x);
}
