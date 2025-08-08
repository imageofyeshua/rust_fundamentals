struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

struct Empty;

// tuple struct
// hours and minues
struct ShortDuration(u32, u32);
// years and month
struct LongDuration(u32, u32);

#[derive(Debug)] // debug trait to print struct
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

#[derive(Debug)]
struct HosannaSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl HosannaSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
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
        println!("Years since Release: {}", self.years_since_release());
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

    fn years_since_release(&self) -> u32 {
        2025 - self.release_year
    }
}

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
       Self {
        origin,
        destination,
        price,
        passengers
       } 
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn itinerary (&self) {
        println!("{} -> {}", self.origin, self.destination);
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

    let amen = HosannaSong::new(String::from("Amen"), 2024, 420);

    amen.display_song_info_ref();

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

    let mut computer = Computer::new(String::from("M3 Max"), 54, 500);

    // build pattern
    computer
        .upgrade_cpu(String::from("M4 Ultimate"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(1024);

    println!("Stats: {computer:#?}");

    let work_shift = ShortDuration(8, 0);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years and {} months", era.0, era.1);

    go_to_work(work_shift);

    // a unit is an empty tuple, a tuple without values
    let empty = ();
    let my_empty_struct = Empty;

    let mut my_flight = Flight::new(
        String::from("New York"),
        String::from("Los Angeles"),
        299.99,
        150,
    );

    println!("{:#?}", my_flight);
    my_flight.change_destination(String::from("San Diego"));
    my_flight.increase_price();
    my_flight.itinerary();
    println!("{:#?}", my_flight);

    let another_flight = Flight {
        origin: String::from("Paris"),
        destination: String::from("Rome"),
        ..my_flight
    };
    println!("{:#?}", another_flight);
    /*
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
    */
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours and {} minutes", length.0, length.1);
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
