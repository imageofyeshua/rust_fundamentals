#[allow(unused_variables)]
fn main() {
    let season = "autumn";

    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, so cold!");
    } else {
        println!("Logs of rain!");
    }

    even_or_odd(17);
    even_or_odd(18);

    // match statement

    let evaluation = true;

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("Value: {value}");

    match season {
        "summer" => println!("School's out!"),
        "winter" => println!("Brr, so cold!"),
        _ => println!("Lots of rain!"),
    }

    let number = 10;

    match number {
        2 | 4 | 6 | 8 => println!("{number} is even"),
        1 | 3 | 5 | 9 => println!("{number} is odd"),
        _ => println!("{number} is out of boundary"),
    }

    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        _ => unreachable!(),
    }

    let mut seconds = 24;

    /*
    loop {
        if seconds <= 0 {
            println!("Blastoff!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds...");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }
    */

    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds...");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }

    println!("Blastoff!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is : {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value is : {element}");
    }

    // recursion

    countdown(5);

    println!("Factorial: {}", factorial_iterative(5));
    println!("Factorial: {}", factorial_recursive(5));

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 3;
        }
    };

    println!("The result is {result}");

    // loop inside loop with loop label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");
}

fn factorial_recursive(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }
    number * factorial_recursive(number - 1)
}

fn factorial_iterative(number: i32) -> i32 {
    let mut product = 1;
    let mut count = number;

    while count > 0 {
        product *= count;
        count -= 1;
    }

    product
}

fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff");
    } else {
        println!("{seconds} seconds to blastoff...");
        countdown(seconds - 1);
    }
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The {number} is {result}");
}
