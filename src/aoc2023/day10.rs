use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;
use crate::aoc2023::day10::Pipe::{Horizontal, LowerLeft, LowerRight, UpperLeft, UpperRight, Vertical};

pub const INPUT_PATH: &str = "inputs/day10/sample.txt";

pub fn run(lines: Lines<BufReader<File>>) {
    day10part1(lines)
}

#[derive(Debug)]
enum Pipe {
    Vertical,   // |
    Horizontal, // -
    UpperLeft,  // F
    UpperRight, // 7
    LowerLeft,  // L
    LowerRight, // J
}

impl Pipe {
    fn from(c: char) -> Result<Pipe, ()> {
        match c {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'F' => Ok(Horizontal),
            '7' => Ok(UpperRight),
            'L' => Ok(LowerLeft),
            'J' => Ok(LowerRight),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize
}

fn day10part1(mut lines: Lines<BufReader<File>>) {
    let mut map: HashMap<Point, Pipe> = HashMap::new();
    let mut start: Point;

    for (y, line_result) in lines.enumerate() {
        let Ok(line) = line_result else {
            continue
        };

        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue
            }

            let point = Point { x, y };

            if c == 'S' {
                start = point;
                continue
            }

            let Ok(pipe) = Pipe::from(c) else {
                continue
            };

            map.insert(point, pipe);


        }
    }

    println!("{:?}", map);
}