// MEMORY MANAGEMENT
// MUTABILITY
// HEAP VS STACK

// STack mostly holds the static data which is countable that it will only hold the limited amount of space. like specific unsigned or signed integers.
// Whereas the Heap hold the larger data like large arrays, vectors, hashmaps, strings etc where we cannnot predict the storage of that data at the compile time it can grow in size in simple way we can call that it hold the things which is dynamic in nature.

// Threee ways of memory management
/* 
- What does it means : It means cleaning of data from heap
1. Garbage collector
2. manual
3. The rust way
*/
fn main() {
    let mut a = 10;
    a = 2;
    println!("A is {}",a);
 


// --------------------------------------------------------------- //
                /* 
                OWNERSHIP : ONly one owner at a time
                MOVING
                BORROWING
                REFERENCES
                */
// --------------------------------------------------------------- //

// here we can see now the a1 is not the owner of that string we cannot call that a1 as now the owner is a2 we have to only call a2
let a1 = String::from("yash");
// IF you want two owners just clone the data by using .clone() => let a2 = a1.clone();
let a2 = a1;
println!("Number is {}",a1);
println!("Number is {}",a2);



// REFERENCES
let s1 = String::from("yash");
do_something(s2:&s1);
println!("String is :: {}", s1);
}

fn do_something(s2: &String) {
    println!("{}",s2);
}





// Borrow and not borrow example
/*  - here we are nto borrowing and it will give us error
fn main() {
let s1:String = String::from("yash");
let s2:String = s1;

println!("hii {} , {}", s1, s2);
}
*/
/* - Here we are borrowing and there will be no error
fn main() {
let s1:String = String::from("Hello");
let s2: &String = &s1;

println!("Hii there {}, {}", s1,s2)
}
*/