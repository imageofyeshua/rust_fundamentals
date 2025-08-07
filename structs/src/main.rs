struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)] // debug trait to print struct
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

#[derive(Debug)]
struct HosannaSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl HosannaSong {
    // Immutable struct value (self parameter takes ownership)
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {}", self.duration_secs);
    }
    // Mutable struct value (self parameter takes ownership, has permission to mutate)
    fn double_length(mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self);
    }
    // Immutable reference to the struct instance (no ownership moved)
    fn display_song_info_ref(&self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {}", self.duration_secs);
    }
    
    // Mutable reference to the struct instance (no ownership moved, have permission to mutate)
    fn double_length_ref(&mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self);
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }
}

#[allow(unused_variables)]
fn main() {
    let mut beverage = Coffee {
        name: String::from("Mocha"), // heap memory >> doesn't have a copy trait
        price: 4.99,
        is_hot: false,
    };

    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = true;

    println!("My {} this morning cost {}, It is {} that it was hot", beverage.name, beverage.price, beverage.is_hot);

    // ownership movement happens
    let favorite_coffee = beverage.name;
    // println!("{}", mocha.name); >> does't work here

    let mut coffee = make_coffee(String::from("Macha"), 4.99, true);

    let caramel_macchiato = Coffee { 
        name: String::from("Caramel Macchiato"),
        ..coffee  // copy the price and is_hot from coffee
    };

    let latte = Coffee {
        name: coffee.name.clone(), // copy the name from coffee, no ownership movement
        ..coffee
    };

    println!("My {} this morning cost {}, It is {} that it was hot", coffee.name, coffee.price, coffee.is_hot);

    drink_coffee(&mut coffee); // doesn't move ownership just reference borrowing to mutate data

    println!("{} {}", coffee.name, coffee.price);

    // debug trait
    let values = ["hello", "world"];
    println!("{:?}", values);
    println!("{:#?}", values); // pretty version 

    println!("{:?}", latte);
    println!("{:#?}", latte);

    let mut again = HosannaSong {
        title: String::from("He is coming, again!"),
        release_year: 2022,
        duration_secs: 240
    };

    let praise_him = HosannaSong {
        title: String::from("Praise Him"),
        release_year: 2025,
        duration_secs: 245,
    };

    if praise_him.is_longer_than(&praise_him) {
        println!("{} is longer than {}", again.title, praise_him.title);
    } else {
        println!("{} is shorter than {}", again.title, praise_him.title);
    }

    // again.display_song_info();
    // println!("{}", again.title); >> display_song_info() took the ownership of song instance above
    // again.double_length();

    // immutable & mutable reference don't move ownership around
    again.display_song_info_ref();
    again.double_length_ref();

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user1 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit struct
    let subject = AlwaysEqual;
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    coffee.price = 10.99;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
