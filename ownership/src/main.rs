fn main() {
    let mut words = String::new();
    words.push_str("Our Father in heaven...");
    // name holds reference, length(bytes) and capacity(bytes) to heap
    let mut name = String::from("Yeshua");
    name.push_str(" Messiah");

    println!("{words}, prayer by {name}");

    let person = String::from("Daniel");
    println!("My nams is {person}");

    // rust moves the ownership from person to genius
    // only genius has to be deallocated after out of scope
    let genius = person; // genius is a reference[stack:address] to person[heap:"Daniel"]

    // person is no longer valid
    //let person = String::from("Daniel");

    let beast = String::from("Lucifer");
    drop(beast);
    // drop() function deallocates memory in heap
    // println!("Who has gone? {beast}");

    // clone() function doesn't move ownership now creates two owners
    let angel = String::from("Gabriel");
    let messenger = angel.clone();
    println!("Angel: {angel}");
    println!("Messenger: {messenger}");
}
