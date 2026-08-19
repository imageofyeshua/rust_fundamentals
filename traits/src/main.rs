// use traits::lodging::{Hotel, AirBnB, Accommodation, Description};
// use traits::utils;
trait Investment {
    fn amount(&self) -> f64;

    fn set_amount(&mut self, new_amount: f64);

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
}

trait Taxable: Investment {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

impl Taxable for Income {}

impl Investment for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
}

struct QualityTime {
    minutes: f64,
}

impl Investment for QualityTime {
    fn amount(&self) -> f64 {
        self.minutes
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.minutes = new_amount;
    }
}

fn main() {
    let mut income = Income { amount: 50000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owned: ${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 10000.23 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());

    let weekend = QualityTime { minutes: 120.0 };
    println!("Relaxation time: {:.2} minutes", weekend.amount());
    /*
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    hotel.book("Piers", 5);
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    utils::book_for_one_night(&mut airbnb, "Dan");
    utils::mix_and_match(&mut hotel, &mut airbnb, "Daniel");
    println!("{:#?}", airbnb);
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
