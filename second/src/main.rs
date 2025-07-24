struct User {
    first_name: String,
    last_name: String,
    age: u32,
}

struct Rect {
    width:u32,
    height:u32,
}

enum Shape{
    Rectangle(f64, f64),
    Circle(f64),
}

impl Rect{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let user = User{
        first_name: String::from("Yash"),
        last_name: String::from("Patel"),
        age: 22,
    };

    let rect = Rect{
        widht: 30,
        height: 50,
    }

    let rect = Shape::Rectangle(1.0,2.0);
    calculate_area(shape:rect);
    let circle = Shape::Circle(1.0);
    calculate_area(shape:circle);

    println!("{}", user.first_name);

// This is how we call a specific function on struct 
    println!("The Area of rectangle is {}", rect.area());
    // println!("{}", user);
}


// This is pattern matching function
fn calculate_area(shape: Shape) -> f64 {
    match shape{
        Shape::Rectangle(a,b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}