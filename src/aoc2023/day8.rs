use std::fs::File;
use std::io::{BufReader, Lines};
use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
use num::integer::lcm;

pub static INPUT_PATH: &str = "inputs/day8/input.txt";

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

pub fn day8part1(mut lines: Lines<BufReader<File>>) {
    let instruction = lines.next().unwrap().unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let re = Regex::new(r"\((\w+), (\w+)\)").unwrap();

    for line_result in lines {
        if let Ok(line) = line_result {
            if line == "" {
                continue
            }
            let splits: Vec<_> = line.split(" = ").collect();
            let name = splits[0].to_owned();
            let left: String;
            let right: String;

            if let Some(caps) = re.captures(splits[1]) {
                left = caps[1].to_owned();
                right = caps[2].to_owned();
            } else {
                panic!("Failed to parse destinations: {}", line)
            }

            // println!("name={}, left={}, right={}", name, left, right);

            let node = Node{ left, right };
            nodes.insert(name, node);
        }
    }

    // println!("{:?}", nodes);

    let mut current_node: String = "AAA".to_string();
    let terminal_node: String = "ZZZ".to_string();
    let mut steps = 0;
    let instructions: Vec<char> = instruction.chars().collect();

    while current_node != terminal_node {
        for c in &instructions {
            match c {
                'R' => current_node = nodes.get(&*current_node).unwrap().right.to_owned(),
                'L' => current_node = nodes.get(&*current_node).unwrap().left.to_owned(),
                _ => panic!("Bad instruction: {}", c)
            }
            // println!("{}", current_node);
            steps += 1;
            if current_node == terminal_node {
                break;
            }
        }
    }

    println!("steps={}", steps);
}

pub fn day8part2(mut lines: Lines<BufReader<File>>) {
    let instruction = lines.next().unwrap().unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let re = Regex::new(r"\((\w+), (\w+)\)").unwrap();

    for line_result in lines {
        if let Ok(line) = line_result {
            if line == "" {
                continue
            }
            let splits: Vec<_> = line.split(" = ").collect();
            let name = splits[0].to_owned();
            let left: String;
            let right: String;

            if let Some(caps) = re.captures(splits[1]) {
                left = caps[1].to_owned();
                right = caps[2].to_owned();
            } else {
                panic!("Failed to parse destinations: {}", line)
            }

            // println!("name={}, left={}, right={}", name, left, right);

            let node = Node{ left, right };
            nodes.insert(name, node);
        }
    }

    let nodes = nodes;

    let mut current_nodes: Vec<String> = vec![];

    for i in nodes.keys() {
        if i.ends_with("A") {
            current_nodes.push(i.clone());
        }
    }

    let instructions: Vec<char> = instruction.chars().collect();
    let mut each_steps: Vec<u128> = vec![];
    let n = current_nodes.len();

    // TODO: calculate the number of steps for each starting spot to reach an end spot independently
    // And then multiply them together.  For example, if one reaches a Z in 3 steps, and the other in 4 steps,
    // then they should both be on Z at the same time in 3*4 steps, or 12


    for i in 0..n {
        let mut steps: u128 = 0;
        'outer: loop {
            for c in &instructions {
                match c {
                    'R' => current_nodes[i] = nodes.get(current_nodes[i].as_str()).unwrap().right.to_owned(),
                    'L' => current_nodes[i] = nodes.get(current_nodes[i].as_str()).unwrap().left.to_owned(),
                    _ => panic!("Bad instruction: {}", c)
                }
                steps += 1;
                // Check for terminal state
                if current_nodes[i].ends_with("Z") {
                    each_steps.push(steps);
                    break 'outer;
                }
            }
        }
    }
    let result: u128 = each_steps.into_iter().reduce(|acc, e| { lcm(acc, e) }).unwrap();
    println!("{}", result);
}