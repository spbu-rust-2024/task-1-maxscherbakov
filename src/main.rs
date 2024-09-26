use itertools::Itertools;
use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let mut x: Vec<i32> = input_line
        .split(' ')
        .map(|x| x.trim().parse().expect("Not Int"))
        .collect();
    x.sort();
    println!("{}", Itertools::join(&mut x.iter(), " "));
}