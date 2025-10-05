struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn distance_from(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

trait Printable {
    fn print(&self);
    fn print_twice(&self) {
        self.print();
        self.print();
    }
}

trait Drawable {
    fn draw(&self);
}

impl Printable for Point {
    fn print(&self) {
        println!("Point coordinates: ({}, {})", self.x, self.y);
    }
}

impl Drawable for Point {
    fn draw(&self) {
        // Draw logic specific to Point
    }
}

trait PrintableAndDrawable: Printable + Drawable {}

impl<T: Printable + Drawable> PrintableAndDrawable for T {}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn main() {
    let p1 = Point { x: 10, y: 10 };

    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(10.0, 5.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    print_trait_object(&p1);
    p1.print();
    p1.draw();
}

fn print_trait_object(obj: &dyn Printable) {
    obj.print();
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle(a, b, c) => {
            // Using Heron's formula to calculate the area of a triangle
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)) .sqrt()
        },
    }
}
