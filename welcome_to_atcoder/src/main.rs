use std::io;

fn main() {
  let mut l1 = String::new();
  let mut l2 = String::new();
  let mut l3 = String::new();

  let stdin = io::stdin();
  stdin.read_line(&mut l1).unwrap();
  let l1_trimed = l1.trim_right();
  let v1: i32 = l1_trimed.parse().unwrap();

  stdin.read_line(&mut l2).unwrap();
  let mut iter = l2.split_whitespace();
  let v2: i32 = iter.next().unwrap().parse().unwrap();
  let v3: i32 = iter.next().unwrap().parse().unwrap();

  stdin.read_line(&mut l3).unwrap();
  let l3_trimed = l3.trim_right();
  println!("{} {}", (v1 + v2 + v3), l3_trimed);
}
