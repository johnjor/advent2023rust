use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;
use crate::aoc2023::day10::Pipe::{Horizontal, LowerLeft, LowerRight, UpperLeft, UpperRight, Vertical};

pub const INPUT_PATH: &str = "inputs/day10/input.txt";

pub fn run(lines: Lines<BufReader<File>>) {
    day10part2(lines)
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
            'F' => Ok(UpperLeft),
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
        let Some(current_pipe) = self.map.get(&self.current) else {
            panic!("current is on an empty space! {:?}", &self)
        };
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

    loop {
        let next = agent.next().unwrap();
        if next == start {
            break
        }
    }

    println!("{}", agent.steps / 2);
}

// Part 2

fn display_map(map: &HashMap<Point, Pipe>, x_len: usize, y_len: usize) {
    for y in 0..y_len {
        for x in 0..x_len {
            let v = match map.get(&Point {x: x as i64, y: y as i64 }) {
                Some(Vertical) => "|",
                Some(Horizontal) => "-",
                Some(UpperLeft) => "F",
                Some(UpperRight) => "7",
                Some(LowerLeft) => "L",
                Some(LowerRight) => "J",
                None => "."
            };
            print!("{}", v);
        }
        print!("\n");
    }
}

fn resolve_start(map: &HashMap<Point, Pipe>, start: Point) -> Pipe {
    let offset_up = Point{x: start.x, y: start.y - 1};
    let offset_down = Point{x: start.x, y: start.y + 1};
    let offset_left = Point{x: start.x - 1, y: start.y};
    let offset_right = Point{x: start.x + 1, y: start.y};

    let up = map.get(&offset_up);
    let down = map.get(&offset_down);
    let left = map.get(&offset_left);
    let right = map.get(&offset_right);

    if let Some(up) = up {
        if *up == UpperLeft || *up == Vertical || *up == UpperRight {
            if let Some(down) = down {
                if *down == LowerLeft || *down == Vertical || *down == LowerRight {
                    return Vertical
                }
            }
        }
    }

    if let Some(right) = right {
        if *right == Horizontal || *right == LowerRight || *right == UpperRight {
            if let Some(down) = down {
                if *down == LowerLeft || *down == Vertical || *down == LowerRight {
                    return UpperLeft
                }
            }
        }
    }

    todo!()
}

fn day10part2(mut lines: Lines<BufReader<File>>) {
    let mut map: HashMap<Point, Pipe> = HashMap::new();
    let mut start = Point{ x: 0, y: 0 };
    let mut x_len: usize = 0;
    let mut y_len: usize = 0;

    for (y, line_result) in lines.enumerate() {
        let Ok(line) = line_result else {
            continue
        };

        x_len = max(x_len, line.len());

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
        y_len += 1;
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

    let mut fill_map: HashMap<Point, Pipe> = HashMap::new();
    while let Ok(next) = agent.next() {
        if next == start {
            let pipe = resolve_start(&fill_map, start);
            fill_map.insert(next, pipe.clone());
            break
        }

        let pipe = map.get(&next).unwrap();
        fill_map.insert(next, pipe.clone());
    }

    // println!("{:?}", fill_map);
    // display_map(&fill_map, x_len, y_len);

    let mut count = 0;
    let mut inside = false;
    let mut left_edge: Option<Pipe> = None;

    for y in 0..y_len {
        for x in 0..x_len {
            let Some(pipe) = fill_map.get(&Point {x: x as i64, y: y as i64 }) else {
                if inside {
                    count += 1;
                }
                continue
            };

            if *pipe == Horizontal {
                continue
            }

            if *pipe == Vertical {
                inside = !inside;
                continue;
            }

            if *pipe == UpperLeft || *pipe == LowerLeft {
                inside = !inside;
                left_edge = Some(*pipe);
                continue
            }

            if *pipe == UpperRight && left_edge == Some(UpperLeft) {
                inside = !inside;
                left_edge = None;
                continue;
            }

            if *pipe == LowerRight && left_edge == Some(LowerLeft) {
                inside = !inside;

                left_edge = None;
                continue;
            }
        }
        inside = false;
    }

    println!("{}", count);

    // sample  = 1
    // sample2 = 1
    // sample3 = 4
}