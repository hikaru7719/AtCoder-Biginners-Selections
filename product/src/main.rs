use std::io;

fn main() {
    let mut l1 = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut l1).unwrap();
    let mut iter = l1.split_whitespace();
    let v1: i32 = iter.next().unwrap().parse().unwrap();
    let v2: i32 = iter.next().unwrap().parse().unwrap();
    let result = v1 * v2;
    if result % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
