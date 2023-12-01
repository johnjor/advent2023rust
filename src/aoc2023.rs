pub mod aoc2023 {
    use std::fs::File;
    use std::io::{BufReader, Lines};

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
}