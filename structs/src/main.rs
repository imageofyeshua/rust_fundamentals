struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
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

    let coffee = make_coffee(String::from("Latte"), 4.99, true);

    let caramel_macchiato = Coffee { 
        name: String::from("Caramel Macchiato"),
        ..coffee  // copy the price and is_hot from coffee
    };

    let latte = Coffee {
        name: coffee.name.clone(), // copy the name from coffee, no ownership movement
        ..coffee
    };

    println!("My {} this morning cost {}, It is {} that it was hot", coffee.name, coffee.price, coffee.is_hot);

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

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
