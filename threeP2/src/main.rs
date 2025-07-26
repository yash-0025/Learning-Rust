// LIFETIMES


struct User<'a> {
    name: &'a str,
}

fn main() {
  
/*  LIFETIME
    // let longest_str;
    // let str1 = String::from("Small");
    // let str2 = String::from("Longer");
    // longest_str = longest(str1, str2);
    // println!("Longest string is :: {}", longest_str);

// Changing a syntax by little
    let longest_str;
    let str1 = String::from("Small");
// Why this is giving error because here the str2 is getting out of the memory so when we try to print the longest_str the str2 value is out of the memory and there's nothing so at this we need lifetime specifier
    {
        let str2 = String::from("Longer");
        longest_str = longest(&str1,&str2);

    }
    println!("Longest string is :: {}", longest_str);

 */

// STRUCT WITH LIFETIME
    let name = String::from("yash");
    let user = User {
        name: &name,
    };
    println!("{}", user.name)

}


// TO FIX THIS LIFETIME PROBLEM 
// THERE IS GENERIC LIFETIME ANNOTATION

/// To solve this lifetime issue there is something called tick ' like this 
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

/*  - As of now this function has an lifetime issue 
fn longest(a:&str, b:&str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
} */
// fn longest(a:String, b:String) -> String {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }


// STRUCT WITH LIFETIMES
/* 

*/