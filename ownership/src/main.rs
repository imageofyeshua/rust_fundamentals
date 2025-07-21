#[allow(unused_variables)]
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

    // references and borrowing
    let my_stack_value = 24;
    let my_stack_reference = &my_stack_value;
    println!("Stack value: {my_stack_reference}");
    println!("Stack value: {}", *my_stack_reference);

    let my_heap_value = String::from("Michael");
    let my_heap_reference = &my_heap_value;
    println!("Heap value: {my_heap_reference}");
    println!("Heap value: {}", *my_heap_reference);

    // string &string, str &str
    /*
        String - A dynamic piece of text stored on the heap at runtime
        &String ("ref String") - A reference to a heap String
        str - A hardcoded, read-only piece of text encoded in the binary
        &str ("ref str") - A reference to the text in the memory that has loaded the binary file
     */

    // reference to executable binary [neither stack nor heap]
    // doesn't move ownership to copied reference
    let ice_cream = "Cookies and Cream";
    let dessert = ice_cream;
    println!("{}", ice_cream);
    println!("{}", dessert);
}
