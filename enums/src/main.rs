// use rand::Rng;
// use std::io;

/*
#[derive(Debug)]
enum Emotion {
   Calm,
   Delighted,
   Angry,
   Excited,
   Sad,
   Stressed,
   Worried
}

#[derive(Debug)]
#[derive(PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
struct MeBot {
    model: String,
    birthday: String,
    id: u32,
    rps: RPS,
    emotion: Emotion,
    power_on: bool,
}

impl MeBot {
    fn new(model: String, birthday: String, id: u32, rps: RPS, emotion: Emotion, power_on: bool) -> Self {
       Self {
        model,
        birthday,
        id,
        rps,
        emotion,
        power_on
       } 
    }

    fn new_rps(&mut self) -> &mut Self {
        let rand_number = rand::rng().random_range(1..=3);
        match rand_number {
            1 => self.rps = RPS::Rock,
            2 => self.rps = RPS::Paper,
            3 => self.rps = RPS::Scissors,
            _ => unreachable!()
        }
        self
    }

    fn say_model(&self) {
        println!("model: {}", self.model);
    }

    fn say_emotion(&self) {
        println!("emotion: {:?}", self.emotion);
    }

    fn say_rps(&self) {
        println!("Mebot chose {:?}", self.rps);
    }

    fn update_emotion(&mut self, emotion: Emotion) -> &mut Self {
        self.emotion = emotion;
        println!("my emotion changed to {:?}", self.emotion);
        self
    }

    fn update_model(&mut self, model: String) -> &mut Self {
        self.model = model;
        println!("my model changed to {:?}", self.model);
        self
    }

    fn power_on(&mut self) {
        if self.power_on == true {
            println!("the power is already on!");
            return;
        }
        self.power_on = true;
    }
}

#[derive(Debug)]
enum  CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit,
}

#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(Credentials),
    Cash,
    Crypto {username: String, dna_code: String}
}
*/

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito{ meat: Meat, beans: Beans },
    Bowl{ meat: Meat, beans: Beans },
    VeganPlate,
}

enum OperatingSystem {
    Windows,
    MacOS,
    Linux
}

enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String)
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature");
            },
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}");
            },
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}");
            }
        }
    }
}

fn main() {
    let lunch = RestaurantItem::Burrito{
        meat: Meat::Steak, 
        beans: Beans::Pinto
    };
    let dinner = RestaurantItem::Bowl{
        meat: Meat::Chicken,
        beans: Beans::Black
    };
    let best = RestaurantItem::VeganPlate;

    println!("Lunch was {lunch:?} and dinner was {dinner:?} but best was {best:?}");
    /*
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
    let card_suits = (CardSuit::Spades, CardSuit::Diamonds);

    let visa = PaymentMethodType::CreditCard(String::from("1134-1234-7654-9876"));
    let mastercard = PaymentMethodType::DebitCard(String::from("1234-7876-3456-1199"));
    println!("{:#?}", mastercard);

    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("1234-5678-9876-5432"));

    let paypel_credentials = Credentials {
        username: String::from("bob@gmail.com"),
        password: String::from("password"),
    };

    let paypal = PaymentMethodType::PayPal(paypel_credentials);
    println!("{:#?}", paypal);

    let crypto = PaymentMethodType::Crypto { 
        username: String::from("Lucifer Satan"),
        dna_code: String::from("messed-up-dna-leading-to-psycho-path"),
    };
    println!("{:#?}", crypto);

    let mut daniel = MeBot::new(
        String::from("D1-2025"),
        String::from("2025-11-25"),
        123545667,
        RPS::Rock,
        Emotion::Calm,
        false
    );

    daniel.say_model();
    daniel.power_on();
    daniel.say_emotion();

    daniel.update_model(String::from("D02-2030")).update_emotion(Emotion::Stressed);

    println!("{:#?}", daniel);

    println!("Let's play game with MeBot");

    loop {
        println!("Please choose from 1: Rock 2: Paper 3: Scissors");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Wrong Input");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess {
            1 => println!("You chose Rock"),
            2 => println!("You chose Paper"),
            3 => println!("You chose Scissors"),
            _ => {
                println!("You dumb ass! choose between 1 and 3");
                continue
            }
        }

        daniel.new_rps().say_rps();

        if guess == 1 && daniel.rps == RPS::Scissors 
            || guess == 2 && daniel.rps == RPS::Rock
            || guess == 3 && daniel.rps == RPS::Paper
         {
            println!("You Won!");
            break;
        } else {
            println!("You Lost!");
        }
    }
    */

    let my_computer = OperatingSystem::Linux;
    let age = years_since_release(my_computer);
    println!("My computer's operating systems is {age} years old");

    let dads_computer = OperatingSystem::Windows;
    let age = years_since_release(dads_computer);
    println!("My dad's computer is {age} years old");

    LaundryCycle::Cold.wash_laundry();
    let hot_cycle = LaundryCycle::Hot { temperature: 100 };
    hot_cycle.wash_laundry();

    let delicate_cycle = LaundryCycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundry();
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system!");
            39
        },
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}