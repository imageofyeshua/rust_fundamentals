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
        _ => println!("Lots of rain!")
    }

    let number = 10;

    match number {
        2 | 4 | 6 | 8 => println!("{number} is even"),
        1 | 3 | 5 | 9 => println!("{number} is odd"),
        _ => println!("{number} is out of boundary")
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

    // recursion

    countdown(5);

    println!("Factorial: {}", factorial_iterative(5));
    println!("Factorial: {}", factorial_recursive(5));
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
