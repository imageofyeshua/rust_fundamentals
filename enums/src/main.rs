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
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
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
    my_payment_method = PaymentMethodType::PayPal(String::from("bob@gmail.com"), String::from("anna@protonmail.com"));
    println!("{:#?}", my_payment_method);
}
