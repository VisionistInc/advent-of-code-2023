use std::fs;

use aoc::work;

fn log(s: String) {
    println!("{}", s)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    work::run(&input, log);
}
