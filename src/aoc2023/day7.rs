use std::fs::File;
use std::io::{BufReader, Lines};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum WinType {
    High,  // Lowest
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,  // Highest
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    wintype: WinType,
    hand: String,
    bid: usize,
}

impl Hand {
    fn new(input: String) -> Hand {
        let splits: Vec<&str> = input.as_str().split(' ').collect();
        let bid: usize = splits[1].parse::<usize>().unwrap();
        let hand = splits[0].to_owned();
        let wintype = Hand::determine_type(&hand);
        Hand{ hand, bid, wintype }
    }

    fn determine_type(input: &String) -> WinType {

        let card_counts_map= input.chars().fold(HashMap::new(), |mut acc, c| {
            let count: &mut usize = acc.entry(c).or_insert(0);
            *count += 1;
            acc
        });

        let j_count = *(card_counts_map.get(&'0').unwrap_or(&0));

        // Part 1
        //let mut card_counts: Vec<usize> = card_counts_map.into_values().collect();

        // Part 2
        let mut card_counts: Vec<usize> = card_counts_map.into_iter().filter_map(|(k, v)| {
            // Because we replaced 'J' with '0' for sorting
            if k != '0' {Some(v)} else {None}
        }).collect();

        card_counts.sort();
        card_counts.reverse();

        let card_counts = card_counts;  // Make it immutable now

        let highest_count = *(card_counts.get(0).unwrap_or(&0)) + j_count;
        let second_count =  if card_counts.len() > 1 {card_counts[1]} else {0};

        match (highest_count, second_count) {
            (5, _) => WinType::FiveOfAKind,
            (4, _) => WinType::FourOfAKind,
            (3, 2) => WinType::FullHouse,
            (3, _) => WinType::ThreeOfAKind,
            (2, 2) => WinType::TwoPair,
            (2, _) => WinType::OnePair,
            _ => WinType::High
        }
    }
}

pub fn day7part1(mut lines: Lines<BufReader<File>>) {
    let mut hands: Vec<Hand> = lines.into_iter().map(|line| {
        let input = line.unwrap()
            .replace("T", "a")
            .replace("J", "0")
            .replace("Q", "c")
            .replace("K", "d")
            .replace("A", "e");
        Hand::new(input)
    }).collect();

    hands.sort();

    println!("{:?}", hands);

    let mut sum: usize = 0;

    for (i, hand) in hands.iter().enumerate() {
        sum += (hand.bid * (i + 1));
    }

    println!("{}", sum);
}