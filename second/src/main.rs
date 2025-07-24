struct User {
    first_name: String,
    last_name: String,
    age: u32,
}


fn main() {
    let user = User{
        first_name: String::from("Yash"),
        last_name: String::from("Patel"),
        age: 22,
    };

    println!("{}", user.first_name);
    // println!("{}", user);
}


