use std::cmp::max;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashSet;

pub const INPUT_PATH: &str = "inputs/day11/input.txt";

pub fn run(lines: Lines<BufReader<File>>) {
    day11part1(lines)
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

fn display_map(map: &HashSet<Point>, x_len: i64, y_len: i64) {
    for y in 0..y_len {
        for x in 0..x_len {
            let v = match map.get(&Point {x, y}) {
                Some(v) => "#",
                None => "."
            };
            print!("{}", v);
        }
        print!("\n");
    }
}

fn day11part1(mut lines: Lines<BufReader<File>>) {
    let mut universe: HashSet<Point> = HashSet::new();
    let mut expanded_y_universe: HashSet<Point> = HashSet::new();
    let mut expanded_universe: HashSet<Point> = HashSet::new();
    let mut x_len: i64 = 0;
    let mut y_len: i64 = 0;

    for (y, line_result) in lines.enumerate() {
        let Ok(line) = line_result else {
            continue
        };

        x_len = max(x_len, line.len() as i64);

        for (x, c) in line.chars().enumerate() {
            if c != '#' {
                continue
            }

            let point = Point {x: x as i64, y: y as i64};
            universe.insert(point);
        }
        y_len += 1;
    }  // Finished populating map


    let mut y_offset: i64 = 0;
    for y in 0..y_len {
        let mut points: Vec<&Point> = universe.iter()
            .filter(|p| { p.y == y as i64 })
            .collect();

        if points.len() == 0 {
            y_offset += 1;
        } else {
            for p in points {
                expanded_y_universe.insert(Point{x: p.x, y: p.y + y_offset});
            }
        }
    }

    let mut x_offset: i64 = 0;
    for x in 0..x_len {
        let mut points: Vec<&Point> = expanded_y_universe.iter()
            .filter(|p| { p.x == x as i64 })
            .collect();

        if points.len() == 0 {
            x_offset += 1;
        } else {
            for p in points {
                expanded_universe.insert(Point{x: p.x + x_offset, y: p.y});
            }
        }
    }

    // display_map(&universe, x_len, y_len);
    // println!();
    // display_map(&expanded_universe, x_len + x_offset, y_len + y_offset);

    let mut sum = 0;
    let galaxies: Vec<Point> = expanded_universe.into_iter().collect();
    for i in 0..galaxies.len()-1 {
        let origin = &galaxies[i];
        for destination in &galaxies[i+1..] {
            let distance = origin.distance(destination);
            sum += distance;
            // println!("{:?} {:?}, {}", origin, destination, distance);
        }
    }
    println!("{}", sum);
}
