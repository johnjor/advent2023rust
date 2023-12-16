use std::fs::File;
use std::io::{BufReader, Lines};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn day4_parse_cards(line: String) -> (HashSet<i32>, Vec<i32>, i32) {
    let splits: Vec<&str> = line.as_str().split('|').collect();
    let winning_numbers: HashSet<i32> = HashSet::from_iter(splits[0]
        .split(':').nth(1).unwrap()
        .split_ascii_whitespace().into_iter().map(|x| {
        x.parse::<i32>().unwrap()
    }));

    let my_numbers: Vec<i32> = splits[1].split_ascii_whitespace().into_iter()
        .map(|x| { x.parse::<i32>().unwrap() }).collect();

    let card_id_re = Regex::new(r"Card\s*(\d+):").unwrap();
    let card_id = card_id_re.captures(line.as_str())
        .unwrap()[1].parse::<i32>().unwrap();

    (winning_numbers, my_numbers, card_id)
}

pub fn day4part1(lines: Lines<BufReader<File>>) {
    let mut sum = 0;

    for line_result in lines {
        if let Ok(line) = line_result {

            let (winning_numbers, my_numbers, _) = day4_parse_cards(line);

            // println!("{:?} - {:?}", winning_numbers, my_numbers);

            let mut row_total = 0;
            for i in my_numbers {
                if winning_numbers.contains(&i) {
                    match row_total {
                        0 => row_total += 1,
                        _ => row_total *= 2
                    };
                };
            };
            sum += row_total;
        }
    }

    println!("{}", sum);
}

pub fn day4part2(lines: Lines<BufReader<File>>) {
    let mut card_copies: HashMap<i32, i32> = HashMap::new();
    for line_result in lines {
        if let Ok(line) = line_result {

            let (winning_numbers, my_numbers, card_id) = day4_parse_cards(line);

            let card_winnings = my_numbers.iter().filter(|x| {
                winning_numbers.contains(x)
            }).count();

            // Number of copies this card has.  I.e. number of times it should be replayed.
            let reruns = card_copies.get(&card_id).unwrap_or(&0).clone();

            // Add the original card to the total count, for the final sum
            if let Some(card_count) = card_copies.get_mut(&card_id) {
                *card_count += 1;
            } else {
                card_copies.insert(card_id, 1);
            }

            // Play this card once for each copy, and once for the original
            for _ in 0..reruns + 1 {
                for i in 0..card_winnings as i32 {
                    // winnings add to the next n cards, so start with current card_id + 1
                    if let Some(card_count) = card_copies.get_mut(&(card_id + i + 1)) {
                        *card_count += 1;
                    } else {
                        card_copies.insert(card_id + i + 1, 1);
                    }
                }
            }
        }
    }

    let sum: i32 = card_copies.into_values().sum();
    println!("{}", sum);
}