mod aoc2022;
mod aoc2023;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use aoc2023::aoc2023::*;


fn main() {
    if let Ok(lines) = read_lines("inputs/day6/input.txt") {
        day6part1(lines);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
