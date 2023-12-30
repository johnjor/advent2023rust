use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;

pub const INPUT_PATH: &str = "inputs/day11/sample.txt";

pub fn run(lines: Lines<BufReader<File>>) {
    day11part1(lines)
}

fn day11part1(mut lines: Lines<BufReader<File>>) {
    for line_result in lines {
        let Ok(line) = line_result else {
            continue
        };
        println!("{}", line);
    }
}
