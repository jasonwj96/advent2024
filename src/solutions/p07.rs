use std::fs::File;
use std::io::{BufRead, BufReader};
fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("Failed to open file.");
    BufReader::new(file)
}

pub fn part1() -> i32 {
    0
}

pub fn part2() -> i32 {
    let reader = read_file("src/tests/07.txt");
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        println!("{}", line);
    }
    0
}