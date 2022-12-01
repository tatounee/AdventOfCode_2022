#![allow(dead_code)]

mod solutions;
mod utils;

use solutions::*;
use utils::load_input;

fn main() {
    let input = load_input("input.txt").unwrap();
    let solution = day1::part2(&input);
    println!("Day {} - Part {} : {}", 15, 1, solution);
}
