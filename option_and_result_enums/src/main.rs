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

    let _invalid_instrument = _musical_instrument.get(100);
    println!("{:?}", _invalid_instrument);
}
