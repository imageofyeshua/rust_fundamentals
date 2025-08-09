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

fn main() {
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
}
