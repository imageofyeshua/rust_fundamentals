#![allow(dead_code)]

#[derive(Debug)]
struct DeliSandwich {}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T 
}

#[derive(Debug)]
enum Chestnutcake<T> {
    Plain,
    Topping(T)
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

// generic type impl of struct
impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(12.34));
    println!("{}", identity("Hello, Daniel"));
    println!("{}", identity(String::from("Adios!")));
    println!("{}", identity(true));
    println!("{:?}", identity(DeliSandwich {}));

    // turbofish operator
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity::<u32>(5));
    println!("{}", identity::<f64>(12.34));
    println!("{}", identity::<&str>("Hello, Daniel"));
    println!("{}", identity::<String>(String::from("Adios!")));
    println!("{}", identity::<bool>(true));
    println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));

    // multiple generics
    println!("{:?}", make_tuple("hello", 5));
    println!("{:?}", make_tuple(12.34, 153));
    println!("{:?}", make_tuple(true, 3.12));
    println!("{:?}", make_tuple(true, false));

    // generics in struct
    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold"
    };
    println!("{:#?}", gold_chest);

    let mut silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("     Silver     ")
    };
    println!("before clean_treasure: {:#?}", silver_chest);
    silver_chest.clean_treasure();
    println!("after clean_treasure: {:#?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"]
    };
    println!("{:#?}", special_chest);
    println!("amount of treasure: {:#?}", special_chest.amount_of_treasure());
    println!("capital captain: {:#?}", special_chest.capital_captain());

    let mushroom = Chestnutcake::Topping("mushroom");
    println!("{:#?}", mushroom);
    let onions = Chestnutcake::Topping("onions".to_string());
    println!("{:#?}", onions);
    let topping = "sweetpotato".to_string();
    let sweetpotato = Chestnutcake::Topping(&topping);
    println!("{:#?}", sweetpotato);
    let mut plain: Chestnutcake<String> = Chestnutcake::Plain;
    plain = Chestnutcake::Topping("fruits".to_string());
    println!("{:#?}", plain);
}

fn identity<T>(value: T) -> T {
    value
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
