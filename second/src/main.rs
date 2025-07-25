use std::fs;

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

    let index = find_alphabet(String::from('prit'));

    match index{
        Some(value) => println!{"Index is {}", value},
        None => println!("A not found")
    }

    let greeting_file_result = fs::read_to_string("hello.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        },
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
}

// Theres two of returning values in function in rust one is normal by adding semicolons and using return keyword and other is this which is called implicit way of returning values 
// This is pattern matching function
fn calculate_area(shape: Shape) -> f64 {
    match shape{
        Shape::Rectangle(a,b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}


// Default Enums for null pointer values and error handling
// 1. Option and 2. Result
// 1. Options :- When the function is returning either any value or a null value 
// Eg: suppose theres  function find_alphabet _a and the string is yash then the value will be 1 but if the string is nit then ther's no a so it should return -1 but the proper way of doing is returning null.

// fn find_alphabet(s: String) -> i32 {
fn find_alphabet(s: String) -> Option<i32> {
    let mut i = 0;

    for (index, char ) in s.chars().enumerate() {
        if char = 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

// 2. Result is used for Error handling which helps us return two values Ok and Err


// Package Management :- We can add external crate in project by running cargo add crate_name
// use std::fs::read_to_string [external crate from the rust only]
// IN cargo.toml in dependecies we can add other dependencies
 