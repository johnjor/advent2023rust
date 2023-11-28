mod aoc2022;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use aoc2022::aoc2022::day1part1;


fn main() {
    if let Ok(lines) = read_lines("../advent2022/day1/official.txt") {
        day1part1(lines);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
