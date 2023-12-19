use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;
use crate::aoc2023::day10::Pipe::{Horizontal, LowerLeft, LowerRight, UpperLeft, UpperRight, Vertical};

pub const INPUT_PATH: &str = "inputs/day10/sample.txt";

pub fn run(lines: Lines<BufReader<File>>) {
    day10part1(lines)
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Debug)]
struct Agent<'a> {
    start: Point,
    current: Point,
    previous: Option<Point>,
    steps: usize,
    map: &'a HashMap<Point, Pipe>,
}

impl Agent<'_> {
    fn next(&mut self) -> Result<Point, ()> {
        let next_step = match self.previous {
            Some(_) => self.find_next_step(),
            None => self.find_first_step(),
        };

        let next =  next_step.unwrap();

        self.previous = Option::from(self.current);
        self.current = next;
        self.steps += 1;

        Ok(next)
    }

    fn find_first_step(&self) -> Result<Point, ()> {
        // look down, if it is a vertical pipe, this is first
        // look up, if it is vertical
        // look left, if it is horizontal
        // look right, if it is horizontal

        // Look down
        let down = Point{x: self.start.x, y: self.start.y + 1};
        if let Some(pipe) = self.map.get(&down) {
            if *pipe == Vertical {
                return Ok(down)
            }
        };

        // Look up
        let up = Point{x: self.start.x, y: self.start.y - 1};
        if let Some(pipe) = self.map.get(&down) {
            if *pipe == Vertical {
                return Ok(up)
            }
        };

        // Look left
        let left = Point{x: self.start.x - 1, y: self.start.y};
        if let Some(pipe) = self.map.get(&down) {
            if *pipe == Horizontal {
                return Ok(left)
            }
        };

        // Look right
        let right = Point{x: self.start.x + 1, y: self.start.y};
        if let Some(pipe) = self.map.get(&down) {
            if *pipe == Horizontal {
                return Ok(right)
            }
        };

        Err(())

    }

    fn find_next_step(&self) -> Result<Point, ()> {
        let current_pipe = self.map.get(&self.current).unwrap();
        let Some(previous) = self.previous else {
            panic!("Cannot call find_next_step() when previous = None")
        };

        if *current_pipe == Vertical {
            let a = Point{x: self.current.x, y: self.current.y + 1};
            let b = Point{x: self.current.x, y: self.current.y - 1};
            return if a == previous { Ok(b) } else { Ok(a) }
        };

        if *current_pipe == Horizontal {
            let a = Point{x: self.current.x + 1, y: self.current.y};
            let b = Point{x: self.current.x - 1, y: self.current.y};
            return if a == previous { Ok(b) } else { Ok(a) }
        };

        if *current_pipe == LowerLeft {
            let a = Point{x: self.current.x + 1, y: self.current.y};
            let b = Point{x: self.current.x, y: self.current.y - 1};
            return if a == previous { Ok(b) } else { Ok(a) }
        }

        if *current_pipe == LowerRight {
            let a = Point{x: self.current.x - 1, y: self.current.y};
            let b = Point{x: self.current.x, y: self.current.y - 1};
            return if a == previous { Ok(b) } else { Ok(a) }
        }

        if *current_pipe == UpperLeft {
            let a = Point{x: self.current.x + 1, y: self.current.y};
            let b = Point{x: self.current.x, y: self.current.y + 1};
            return if a == previous { Ok(b) } else { Ok(a) }
        }

        if *current_pipe == UpperRight {
            let a = Point{x: self.current.x - 1, y: self.current.y};
            let b = Point{x: self.current.x, y: self.current.y + 1};
            return if a == previous { Ok(b) } else { Ok(a) }
        }

        return Err(())
    }
}

fn day10part1(mut lines: Lines<BufReader<File>>) {
    let mut map: HashMap<Point, Pipe> = HashMap::new();
    let mut start = Point{ x: 0, y: 0 };

    for (y, line_result) in lines.enumerate() {
        let Ok(line) = line_result else {
            continue
        };

        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue
            }

            let point = Point { x: x as i64, y: y as i64 };

            if c == 'S' {
                start = point;
                continue
            }

            let Ok(pipe) = Pipe::from(c) else {
                continue
            };

            map.insert(point, pipe);
        }
    }  // Finished populating map

    // start no longer mutable
    let start = start;

    // println!("{:?}", map);

    let mut agent = Agent{
        start,
        current: start,
        previous: None,
        steps: 0,
        map: &map
    };

    println!("{:?}", start);
    println!("{:?}", agent.next().unwrap());
    println!("{:?}", agent.next().unwrap());
    println!("{:?}", agent.next().unwrap());

    println!("{}", agent.steps);

}