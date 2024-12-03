extern crate itertools;
extern crate nalgebra;

mod solutions;

fn main() {
    println!("Puzzle 1, Part 1: {}", solutions::p01::part1());
    println!("Puzzle 1, Part 2: {}", solutions::p01::part2());
    println!("Puzzle 2, Part 1: {}", solutions::p02::part1());
    println!("Puzzle 2, Part 2: {}", solutions::p02::part2());
}