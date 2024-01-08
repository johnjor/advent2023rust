use std::fs::File;
use std::io::{BufReader, Lines};
use regex::Regex;

pub const INPUT_PATH: &str = "inputs/day12/input.txt";

pub fn run(lines: Lines<BufReader<File>>) {
    day12part1(lines)
}

fn create_pattern(numbers: &Vec<i64>) -> Regex {

    let inner_part = numbers.iter().map(|i| {
        format!("1{{{}}}", i)
    }).collect::<Vec<String>>().join("0+");

    let pattern_string = format!("^0*{inner_part}0*$");

    Regex::new(&*pattern_string).expect("Pattern doesn't compile!")
}

fn create_pattern2(string: &str) -> Regex {
    let inner_part = string.replace("?", "[01]").replace(".", "0").replace("#", "1");
    let pattern_string = format!("^{inner_part}$");
    Regex::new(&*pattern_string).expect("Pattern2 doesn't compile!")
}

fn process_line(line: String) -> i64 {
    let splits: Vec<&str> = line.split(" ").collect();
    let n = splits[0].len();
    let pattern2 = create_pattern2(splits[0]);
    // let variable_bit_string = splits[0].replace("?", "1").replace(".", "0").replace("#", "0");
    // let fixed_bit_string = splits[0].replace("?", "0").replace(".", "0").replace("#", "1");
    let numbers: Vec<i64> = splits[1].split(',').map(|x| {
        x.parse::<i64>().unwrap()
    }).collect();
    // let variable_bits = u32::from_str_radix(&*variable_bit_string, 2).expect("Not a binary string");
    // let fixed_bits = u32::from_str_radix(&*fixed_bit_string, 2).expect("Not a binary string");

    let pattern = create_pattern(&numbers);

    let mut num_of_matches = 0;
    for i in 0..1<<n {
        let as_string = String::from(format!("{i:0n$b}"));
        let is_match = pattern.is_match(&*as_string);
        let is_match2 = pattern2.is_match(&*as_string);

        if is_match && is_match2 {
            num_of_matches += 1
        }

        // println!("{} {as_string} {is_match} {is_match2}", splits[0]);
    }

    // println!("{} {:?} {:?} {:b}", num_of_matches, pattern, pattern2, (1<<n)-1);

    num_of_matches
}

fn day12part1(mut lines: Lines<BufReader<File>>) {
    let mut sum: i64 = 0;
    for line_result in lines {
        let Ok(line) = line_result else {
            continue
        };

        sum += process_line(line);

    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(process_line("???.### 1,1,3".to_string()), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(process_line(".??..??...?##. 1,1,3".to_string()), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(process_line("?#?#?#?#?#?#?#? 1,3,1,6".to_string()), 1);
    }
}