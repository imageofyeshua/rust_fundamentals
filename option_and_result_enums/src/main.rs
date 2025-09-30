fn main() {
    // basic concept
    let _a = Option::Some(5);
    let _b = Option::Some("Daniel");
    let _c = Option::Some(true);
    let _d: Option<i8> = Option::Some(7);
    let _e: Option<i16> = Option::Some(12);

    println!("a: {:?}", _a);
    println!("b: {:?}", _b);
    println!("c: {:?}", _c);
    println!("d: {:?}", _d);
    println!("e: {:?}", _e);

    let _f: Option<&str> = Option::None;
    println!("f: {:?}", _f);

    // example
    let _musical_instrument = [
        String::from("Piano"),
        String::from("Violin"),
        String::from("Cello"),
    ];

    let _cello: Option<&String> = _musical_instrument.get(2);
    println!("{:?}", _cello);

    let _valid_instrument = _cello.unwrap();
    println!("{_valid_instrument}");

    let _invalid_instrument = _musical_instrument.get(100);
    println!("{:?}", _invalid_instrument);
    // _invalid_instrument.expect("### Unable to retrieve musical instrument ###");

    let _piano: Option<&String> = _musical_instrument.first();

    match _piano {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }

    match _invalid_instrument {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }

    play(_piano);
    play(_invalid_instrument);

    // returning an option enum from a function
    let availability = is_item_in_stock(true, false);
    println!("{availability:?}");

    match availability {
        Option::Some(value) => println!("Item is available: {value}"),
        Option::None => println!("Your item doesn't exist in our system"),
    }

    // unwrap_or method
    let _present_value = Some(12);
    let _missing_value: Option<i32> = None;

    println!("{}", _present_value.unwrap_or(0));
    println!("{}", _missing_value.unwrap_or(100));
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option:: Some(false)
    } else {
        Option::None
    }
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}
