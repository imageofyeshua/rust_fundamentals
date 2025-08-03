#[allow(unused_variables)]
fn main() {
    // stack-only copy
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // types implement copy
    // integer types such as u32
    // boolean types, floating-point types, character type
    // and tuples if they only contain types that also implement copy

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

    // ownership movement example
    let oranges = String::from("Oranges");
    print_my_value(oranges);
    // now the ownership of oranges moved to value in fn print_my_value()
    // println!("{oranges} is still valid");

    let burger = String::from("Burger");
    add_fries(burger);

    // ownership is moved from cake in bake_cake() to cake in main
    let cake = bake_cake();
    println!("My cake: {cake}");

    // example wrap-up
    let is_concert = true;
    let is_event = is_concert;
    println!("{is_concert} : {is_event}");

    // neither stack nor heap but executable binary copied just reference to string "Salmon"
    let sushi = "Salmon";
    let dinner = sushi;
    println!("{sushi} : {dinner}");

    // ownership movement from sushi to dinner for data in heap
    let sushi = String::from("Salmon");
    let dinner = sushi;

    let fish = eat_meal(dinner);
    println!("{fish}");

    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    // immutable reference
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);

    // mutable references cannot co-exist in the same scope
    let mut vehicle = String::from("Rearcar");
    let ref3 = &mut vehicle;
    // let ref4 = &mut vehicle;
    // println!("{ref3} and {ref4}");

    let mut coffee = String::from("Mocha");
    let a = &mut coffee;
    println!("{a}");
    let b = a; // ownership moves from a to b
    println!("{b}"); // cannot access "a" here

    let city = create_city();
    println!("My city: {city}");

    // arrays and tuples
    let registrations = [true, false, true];
    let first = registrations[0]; // copy traits
    println!("{first} and {registrations:?}");

    let languages = [String::from("Rust"), String::from("JavaScript")];
    // partial ownership movement prohibited here
    // let first = languages[0]; 
    // let first = languages[0].clone();
    let first = &languages[0]; // borrowing reference
    println!("{first} and {languages:?}");

    let signup = (true, "Daniel", 56);
    let first = signup.0; // copy traits
    println!("{first} and {signup:?}");

    let frameworks = (String::from("C++"), String::from("Rust"));
    let first = &frameworks.0; // borrowing reference
    println!("{first} and {frameworks:?}");

    // exercise
    let mut trip = start_trip();
    visit_seoul(&mut trip);
    trip.push_str(" and ");
    visit_gimcheon(&mut trip);
    trip.push_str(" and ");
    visit_busan(&mut trip);
    trip.push_str(".");
    show_itinerary(&trip);

    let s = String::from("hell");
    takes_ownership(s);
    // println!("{s}"); // accessing s is no longer valid here

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    // returning ownership of parameters
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("copied integer: {some_integer}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_seoul(trip: &mut String) {
    trip.push_str("Seoul");
}

fn visit_gimcheon(trip: &mut String) {
    trip.push_str("Gimcheon");
}

fn visit_busan(trip: &mut String) {
    trip.push_str("Busan");
}

fn show_itinerary(trip : &String) {
    println!("{trip}");
}

fn create_city() -> String {
    String::from("Gimcheon")
    // let city = String::from("New York");
    // dangling referece if you return &city
    // city
}

fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}

fn eat_meal(meal: String) -> String {
    // the clear() method modifies a heap string to have no content
    // meal.clear();
    meal
}

fn bake_cake() -> String {
    let cake = String::from("Baguette Wholemeal");
    return cake;
}

// the ownership is moved to meal
fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}")
}

fn print_my_value(value: String) {
    println!("Your value is {value}");
}
