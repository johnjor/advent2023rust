use std::fs::File;
use std::io::{BufReader, Lines};
use std::hash::Hash;

pub const INPUT_PATH: &str = "inputs/day9/input.txt";

pub fn run(lines: Lines<BufReader<File>>) {
    day9part1(lines)
}

fn get_difference(numbers: &Vec<i64>) -> i64 {
    let n = numbers.len();
    let mut differences: Vec<i64> = vec![];

    for i in 0..n-1 {
        differences.push(numbers[i+1] - numbers[i])
    };

    let result: i64;
    if differences.iter().all(|x| {*x == 0}) {
        result = 0;
    } else {
        // Part 1
        // result = differences[differences.len() - 1] + get_difference(&differences);

        // Part 2
        result = differences[0] - get_difference(&differences);
    }

    println!("{} {:?}", result, differences);
    result

}

fn day9part1(mut lines: Lines<BufReader<File>>) {
    let mut sum: i64 = 0;

    for line_result in lines {
        let Ok(line) = line_result else {
            continue
        };

        let numbers: Vec<i64> = line.as_str().split_ascii_whitespace()
            .map(|x| { x.parse::<i64>().unwrap() }).collect();

        // Part 1
        // let prediction = numbers[numbers.len()-1] + get_difference(&numbers);

        // Part 2
        let prediction = numbers[0] - get_difference(&numbers);

        println!("{} {:?}\n", prediction, numbers);

        sum += prediction;
    }

    println!("sum = {sum}");
}