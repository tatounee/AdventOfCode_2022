#![feature(iter_array_chunks)] // day 3
#![feature(array_windows)] // day 6 + day 14
#![feature(iter_intersperse)] // day 10
#![feature(let_chains)] // day 13
#![allow(dead_code)]

mod solutions;
mod utils;

use solutions::*;
use utils::load_input;

fn main() {
    let input = load_input("input.txt").unwrap();
    let solution = day15::part2(&input);
    println!("Day {} - Part {} : {}", 15, 2, solution);
}
