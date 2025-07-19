fn main() {
    let season = "summer";

    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, so cold!");
    } else {
        println!("Logs of rain!");
    }

    even_or_odd(17);
    even_or_odd(18);
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd"};
    println!("The {number} is {result}");
}
