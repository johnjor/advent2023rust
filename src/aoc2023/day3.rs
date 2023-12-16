use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct PartNumber {
    number: i32,
    row: usize,
    col: usize,
    length: usize
}

#[derive(Debug)]
struct Symbol<'a> {
    character: char,
    part_numbers: Vec<&'a PartNumber>,
}

pub fn day3part1(lines: Lines<BufReader<File>>) {
    let mut symbols: HashMap<Point, char> = HashMap::new();
    let mut numbers: Vec<PartNumber> = vec![];

    for (row, line_result) in lines.enumerate() {
        let mut digit_buffer  = "".to_string();
        if let Ok(line) = line_result {
            for (col, c) in line.chars().enumerate() {
                if !c.is_digit(10) {
                    if digit_buffer.len() > 0 {
                        numbers.push(PartNumber{
                            number: digit_buffer.parse::<i32>().unwrap(),
                            row,
                            col: col - digit_buffer.len(),
                            length: digit_buffer.len()
                        })
                    }
                    digit_buffer = "".to_string();
                }

                if c != '.' && !c.is_digit(10) {
                    symbols.insert(Point{x: row as i32, y: col as i32}, c);
                } else if c.is_digit(10) {
                    digit_buffer.push(c)
                }
            }

            if digit_buffer.len() > 0 {
                numbers.push(PartNumber{
                    number: digit_buffer.parse::<i32>().unwrap(),
                    row,
                    col: line.len() - digit_buffer.len(),
                    length: digit_buffer.len()
                })
            }
        }
    }

    // for (point, symbol) in &symbols {
    //     println!("{}: {}", point, symbol);
    // }

    let sum: i32 = numbers.iter().map(|part| {

        for c in part.col as i32-1..part.col as i32+part.length as i32+1 {
            // println!("{:?}, {}, {}", part, 1, c);
            if symbols.contains_key(&Point{x: part.row as i32 + 1, y: c}) {
                // println!("IS: {}", part.number);
                return part.number
            }

            // println!("{:?}, {}, {}", part, -1, c);
            if symbols.contains_key(&Point{x: part.row as i32 - 1, y: c}) {
                // println!("IS: {}", part.number);
                return part.number
            }
        }

        // println!("{:?}, {}, {}", part, 0, part.col as i32-1);
        if symbols.contains_key(&Point{x: part.row as i32, y: part.col as i32-1}) {
            // println!("IS: {}", part.number);
            return part.number
        }
        // println!("{:?}, {}, {}", part, part.row, part.col+part.length);
        if symbols.contains_key(&Point{x: part.row as i32, y: part.col as i32+part.length as i32}) {
            // println!("IS: {}", part.number);
            return part.number
        }

        // println!("NOT: {}", part.number);
        0

    }).sum();

    println!("{}", sum);
}

pub fn day3part2(lines: Lines<BufReader<File>>) {
    let mut symbols: HashMap<Point, Symbol> = HashMap::new();
    let mut numbers: Vec<PartNumber> = vec![];

    for (row, line_result) in lines.enumerate() {
        let mut digit_buffer  = "".to_string();
        if let Ok(line) = line_result {
            for (col, c) in line.chars().enumerate() {
                if !c.is_digit(10) {
                    if digit_buffer.len() > 0 {
                        numbers.push(PartNumber{
                            number: digit_buffer.parse::<i32>().unwrap(),
                            row,
                            col: col - digit_buffer.len(),
                            length: digit_buffer.len()
                        })
                    }
                    digit_buffer = "".to_string();
                }

                if c != '.' && !c.is_digit(10) {
                    symbols.insert(
                        Point{x: row as i32, y: col as i32},
                        Symbol{character: c, part_numbers: vec![]}
                    );
                } else if c.is_digit(10) {
                    digit_buffer.push(c)
                }
            }

            if digit_buffer.len() > 0 {
                numbers.push(PartNumber{
                    number: digit_buffer.parse::<i32>().unwrap(),
                    row,
                    col: line.len() - digit_buffer.len(),
                    length: digit_buffer.len()
                })
            }
        }
    }

    // for (point, symbol) in &symbols {
    //     println!("{}: {}", point, symbol);
    // }

    for part in &numbers {
        for c in part.col as i32-1..part.col as i32+part.length as i32+1 {
            // println!("{:?}, {}, {}", part, 1, c);
            if let Some(symbol) = symbols.get_mut(&Point{x: part.row as i32 + 1, y: c}) {
                // println!("IS: {}", part.number);
                symbol.part_numbers.push(part)
            }

            // println!("{:?}, {}, {}", part, -1, c);
            if let Some(symbol) = symbols.get_mut(&Point{x: part.row as i32 - 1, y: c}) {
                // println!("IS: {}", part.number);
                symbol.part_numbers.push(part)
            }
        }

        // println!("{:?}, {}, {}", part, 0, part.col as i32-1);
        if let Some(symbol) = symbols.get_mut(&Point{x: part.row as i32, y: part.col as i32-1}) {
            // println!("IS: {}", part.number);
            symbol.part_numbers.push(part)
        }
        // println!("{:?}, {}, {}", part, part.row, part.col+part.length);
        if let Some(symbol) = symbols.get_mut(&Point{x: part.row as i32, y: part.col as i32+part.length as i32}) {
            // println!("IS: {}", part.number);
            symbol.part_numbers.push(part)
        }
    }

    let mut sum = 0;
    for (_, symbol) in symbols {
        if symbol.character == '*' && symbol.part_numbers.len() == 2 {
            sum += symbol.part_numbers[0].number * symbol.part_numbers[1].number
        }
    }
    println!("{}", sum);
}