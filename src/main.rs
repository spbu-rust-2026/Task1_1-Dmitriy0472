use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let fisrt = parts.next().unwrap();
    let second = parts.next().unwrap();
    let a = fisrt.parse::<i64>().unwrap();
    let b = second.parse::<i64>().unwrap();
    let c = a + b;
    println!("{}", c);
}
