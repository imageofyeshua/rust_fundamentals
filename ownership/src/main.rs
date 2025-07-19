fn main() {
    let mut words = String::new();
    words.push_str("Our Father in heaven...");
    // name holds reference, length(bytes) and capacity(bytes) to heap
    let mut name = String::from("Yeshua");
    name.push_str(" Messiah");

    println!("{words}, prayer by {name}");
}
