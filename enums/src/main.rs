// Define the Color enum
enum Color {
    Red,
    Green,
    Blue,
}

// Define the TrafficLight
enum TrafficLight {
    Green,
    Yellow,
    Red,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Red => TrafficLight::Green,
        }
    }
}

fn main() {
    // Create instances of the enums
    let col = Color::Red;
    let light = TrafficLight::Red;
    let next = light.next();

    // Match and print the values to demonstrate usage
    match col {
        Color::Red => println!("Color is Red"),
        Color::Blue => println!("Color is Blue"),
        Color::Green => println!("Color is Green"),
    }

    match light {
        TrafficLight::Green => println!("Traffic light is Green"),
        TrafficLight::Yellow => println!("Traffic light is Yellow"),
        TrafficLight::Red => println!("Traffic light is Red"),
    } 

    match next {
        TrafficLight::Green => println!("Next light is Green"),
        TrafficLight::Yellow => println!("Next light is Yellow"),
        TrafficLight::Red => println!("Next light is Red"),
    }
}
