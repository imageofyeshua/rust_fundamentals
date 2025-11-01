use traits::{book_for_one_night, mix_and_match, AirBnB, Hotel, Accommodation, Description};

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    hotel.book("Piers", 5);
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{:#?}", airbnb);
    /*
    let mut hotel = Hotel::new(String::from("The Poor"));
    let mut airbnb = AirBnB::new("Evil Genius");

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb];
    stays[0].book("Piers", 2);
    stays[1].book("Amanda", 3);

    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);

    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];

    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());

    let hotel1 = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel1.summarize());

    let hotel2 = Hotel::new("The Golden Standard");
    println!("{}", hotel2.summarize());

    let hotel3 = Hotel::new(vec!("The Sweet Escape", "Hilton Hell"));
    // println!("{}", hotel3.summarize());

    let mut hotel = choose_best_place_to_stay();
    let mut airbnb = AirBnB::new("Peter");
    mix_and_match(&mut hotel, &mut airbnb, "Piers");

    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    hotel.book("Piers", 5);
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{:#?}", airbnb);
    */
}
