use std::fs::File;
use std::io::{BufReader, Lines};
use std::hash::Hash;

pub fn day1part1(lines: Lines<BufReader<File>>) {
    let mut sum = 0;
    let mut left: char = '0';
    let mut right: char = '0';

    for line_result in lines {
        if let Ok(line) = line_result {
            for c in line.chars() {
                if c.is_digit(10) {
                    left = c;
                    break
                }
            }
            for c in line.chars().rev() {
                if c.is_digit(10) {
                    right = c;
                    break
                }
            }
            let number = format!("{}{}", left, right).parse::<i32>().unwrap();
            // println!("{}", number);
            sum += number;
        }
    }

    println!("{}", sum);
}

fn day1part2matcher(slice: &str) -> Option<char> {
    return match slice {
        x if x.starts_with("one") => Some('1'),
        x if x.starts_with("two") => Some('2'),
        x if x.starts_with("three") => Some('3'),
        x if x.starts_with("four") => Some('4'),
        x if x.starts_with("five") => Some('5'),
        x if x.starts_with("six") => Some('6'),
        x if x.starts_with("seven") => Some('7'),
        x if x.starts_with("eight") => Some('8'),
        x if x.starts_with("nine") => Some('9'),
        // Is there a better way to do this?
        x if x.chars().next().unwrap().is_digit(10) => Some(x.chars().next().unwrap()),
        _ => None
    };
}

pub fn day1part2(lines: Lines<BufReader<File>>) {
    let mut sum = 0;
    let mut left: char = '0';
    let mut right: char = '0';

    for line_result in lines {
        if let Ok(line) = line_result {
            let len = line.len();

            for i in 0..len {
                if let Some(digit) = day1part2matcher(&line[i..]) {
                    left = digit;
                    break;
                }
            }

            for i in 0..len {
                if let Some(digit) = day1part2matcher(&line[(len-i-1)..]) {
                    right = digit;
                    break;
                }
            }

            let number = format!("{}{}", left, right).parse::<i32>().unwrap();
            sum += number;
        }
    }

    println!("{}", sum);
}