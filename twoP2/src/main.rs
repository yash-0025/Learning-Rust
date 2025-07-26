// STRINGS VS SLICES
/* 
The string variable will be stored on stack and it will reference to the heap where the actual string data is located 
suppose there is a string 
        let s = String::from("yash");
here the variable s will be stored on stack and the data hello will be stored on heap . The stack structure is like this :: 
name - value
ptr - poiting to heap index
len - 4
capacity - 4        
# Heap will look like this :: 
index - value
0 - Y
1 - A
2 - S
3 - H

1. Creating a string
2. Mutating a string
3. Deleting from a string
fn main() {
let name = String::from("yash");
let mut name = String::from("yash");
name.push_str(" patel")
println!("Name is :: {}", name);
name.replace_range(8..name.len(), "");
println!("name is {}", name); 
}
*/

// GENERICS 
/* 
We can define generics type for the input argument to prevent code repetition or something like the same scenario
*/
fn main() {
    let name = String::from("hello world");
    let ans = first_word(str:name);
    println!("Ans is {}", ans);

// STRING SLICE 
    let mut word = String::from("Hello World");
    let word2 = find_first_word(&word);
    println!("{}", word);


// GENERICS - here we can use the same function for both number and string because of this generic types functionality otherwise we have to use two different functions 
    let bigger = largest(1,2);
    let bigger_char = largest(a,b);
    println!("Bigger number is :: {}",bigger);
    println!("Bigger char is :: {}",bigger_char);
}

fn largest<T: std::cmp::PartialOrd>(a:T, b:T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn find_first_word(word: &String) -> &str {
    let mut index = 0;
    for (_,i) in word.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }

    return &word[0..index];
}

fn first_word(str: String) -> String {
    let mut ans = String::from("");
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }
    return ans;
}