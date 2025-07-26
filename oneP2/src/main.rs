// COLLECTIONS
/*  VECTORS
It allows you to store more than one value in a single data structure that puts all the values next to each other in memory

fn main() {
    let mut vec = Vec::new();
    vec.push(1)
    vec.push(2)
    vec.push(3)
    println!("{:?}", vec)

}

*/

// HASHMAPS 
/* 
It stores a key value pair in rust. like [user1: id1, user2: id2]
Methods are :: 1. insert, 2. get, 3.remove, 4.clear
// To use hashmap we have to import hashmap 
use std::collections::HashMap;

*/
use std::collections::HashMap;

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", even_filter(&vec));



    let mut users: HashMap<String, u32> = HashMap::new();

    let input_vec = vec![(String::from("yash"),22), (String::from("patel"),21)];
    let hm = group_values_by_key(input_vec);
    println!("{:?}", hm);


    // ITERATOR
    let mut v1 = vec![1,2,3];
    let mut v1_iter = vec.iter_mut();

    while let Some(val) = v1_iter.next() {
        println!("{}", val);
    }

    /* 
    for val in v1_iter {
    *val = *val + 1
    }
    */
    println!("{:?}", v1);
}

fn group_values_by_key(vec: Vec<String, i32>) -> HashMap<String,i32> {
    let mut hm = HashMap::new();
    for(key, value) in vec {
        hm.insert(k: key, v:value);
    }
    return hm;
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            // Here * is used for derefrencing 
            new_vec.push(*val);
        }
    }

    return new_vec;

}

// ITEREATORS
/* 
The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. While using iterators we don;t have to reimplement that logic .
In rust iterators are lazy meaning they have no effect until we call methods that consume the iterator to use it up.
ex:the code in listing 13-10 creates an iterator over the items in the vector v1 by calling the iter method defined on Vec<T>.

let v1 = vec![1,2,3]
let v1_iter = v1.iter();

// Types of iterators
1. .iter :: If you want immutable references to the inner variables and dont  wamt to transfer ownership
2. .iter_mut :: If you want mutable references to the inner variables and don't want to transfer ownership
3. iter_into :: If you want to move the variable into the iterator and don't want to use it afterwards

There are two type of functions in the iterator :: Consuming adapter and Iterator adaptor
. Consuming adapter has a function like .sum which will calculate the sum of all the values but that iter will not be used again after calling sum over that
. Iterator adapters are methods defined on iterator trait that don't consume the iter instead they will produce different iter by changing some aspect of the original iterator
Eg .map , .filter 
fn main() {
let v1: Vec<i32> = vec![1,2,3];
let iter = v1.iter();
// here |x| =>this is the argument and the x+1 is the return value
let iter2 = iter.map(|x| x+1);
for x in iter2 {
println!("{}",x);
}
}
*/