fn main() {
    let ans = is_even(10);
    println!("{}", ans);
    println!("{}", fib(10));

    let name = String::from("Yash");
    let len = get_str_ln(name);
    println!("The length of the string is {}", len);
}


fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 0..(num - 1) {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}


fn get_str_ln(str: String) -> usize {
    str.chars().count()
}