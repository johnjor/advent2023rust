use std::fs::File;
use std::io::{BufReader, Lines};

pub const INPUT_PATH: &str = "inputs/day12/sample.txt";

pub fn run(lines: Lines<BufReader<File>>) {
    day12part1(lines)
}

fn day12part1(mut lines: Lines<BufReader<File>>) {
    for line_result in lines {
        let Ok(line) = line_result else {
            continue
        };

        println!("{}", line);
    }
}