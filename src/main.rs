use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let first = parts.next().unwrap();
    let second = parts.next().unwrap();
    let a = first.parse::<i64>().unwrap();
    let b = second.parse::<i64>().unwrap();
    println!("{}", a + b);
}
