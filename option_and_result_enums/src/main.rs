#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh!"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

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

    // custom unwrap 
    let some_option = MyOption::Some(100);
    println!("some option: {}", some_option.unwrap());

    let another_option = MyOption::Some(200);
    println!("another option: {}", another_option.unwrap_or(224));

    /*
    let none_option = MyOption::None;
    println!("none option: {}", none_option.unwrap());

    let another_none_option = MyOption::None;
    println!("another none option: {}", another_none_option.unwrap_or(224));
    */

    // result enum
    let ok: Result<i8, &str> = Ok(5);
    println!("{ok:?}");
    let disaster: Result<i32, &str> = Err("Something went wrong!");
    println!("{:?}", disaster);

    let text = "50";
    let _text_as_number = text.parse::<i32>();
    println!("{:?}", _text_as_number);

    let text = "Alabama";
    let _text_as_number = text.parse::<i32>();
    println!("{:?}", _text_as_number);

    let _result = divide(10.0, 4.0);

    /*
    match _result {
        Ok(calculation) => println!("Result: {}", calculation),
        Err(message) => println!("Error: {}", message),
    }
    */

    println!("{}", _result.unwrap());
    // println!("{}", _result.is_ok());
    // println!("{}", _result.is_err());
    // println!("{}", _result.unwrap_or(5.0));
    // println!("{}", _result.expect("Unable to calculate"));
    
    let _my_result = operation(true);

    let _content = match _my_result {
        Ok(message) => message,
        Err(error) => error,
    };

    println!("My result: {}", _my_result.unwrap());
    println!("My result: {}", _my_result.unwrap());
    println!("My result: {}", _my_result.unwrap());

    // while - let construct
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}");
    }
}

// &'static str implements copy trait >> doesn't move ownership
fn operation(great_success: bool) -> Result<&'static str, &'static str> {
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
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
