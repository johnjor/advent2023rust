use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashSet;
use std::cmp::{min, max};

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

fn distance(origin: &Point, destination: &Point, x_expansions: &HashSet<i64>, y_expansions: &HashSet<i64>, expansion_factor: i64) -> i64 {
    let mut x_distance = (destination.x - origin.x).abs();
    for i in min(origin.x, destination.x)..max(origin.x, destination.x) {
        if x_expansions.contains(&i) {
            x_distance += expansion_factor - 1;
        }
    }

    let mut y_distance = (destination.y - origin.y).abs();
    for i in min(origin.y, destination.y)..max(origin.y, destination.y) {
        if y_expansions.contains(&i) {
            y_distance += expansion_factor - 1;
        }
    }

    x_distance + y_distance
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
    let mut x_expansions: HashSet<i64> = HashSet::new();
    let mut y_expansions: HashSet<i64> = HashSet::new();
    let mut x_len: i64 = 0;
    let mut y_len: i64 = 0;
    // const EXPANSION_FACTOR: i64 = 2;  // Part 1
    const EXPANSION_FACTOR: i64 = 1_000_000;  // Part 2

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

    for x in 0..x_len {
        let mut points: Vec<&Point> = universe.iter()
            .filter(|p| { p.x == x as i64 })
            .collect();

        if points.len() == 0 {
            x_expansions.insert(x);
        }
    }

    for y in 0..y_len {
        let mut points: Vec<&Point> = universe.iter()
            .filter(|p| { p.y == y as i64 })
            .collect();

        if points.len() == 0 {
            y_expansions.insert(y);
        }
    }

    // display_map(&universe, x_len, y_len);
    // println!();
    // display_map(&expanded_universe, x_len + x_offset, y_len + y_offset);

    let mut sum = 0;
    let galaxies: Vec<Point> = universe.into_iter().collect();
    for i in 0..galaxies.len()-1 {
        let origin = &galaxies[i];
        for destination in &galaxies[i+1..] {
            let distance = distance(
                &origin, &destination, &x_expansions, &y_expansions, EXPANSION_FACTOR
            );
            sum += distance;
            // println!("{:?} {:?}, {}", origin, destination, distance);
        }
    }
    println!("{}", sum);
}
