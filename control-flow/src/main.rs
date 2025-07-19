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
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The {number} is {result}");
}
