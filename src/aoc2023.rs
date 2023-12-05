pub mod aoc2023 {
    use std::fs::File;
    use std::io::{BufReader, Lines};
    use regex::Regex;
    use std::collections::{HashMap, HashSet};
    use std::fmt;
    use std::hash::Hash;

    // Day 1
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

    // Day 2
    struct MaxRGB {
        red: i32,
        green: i32,
        blue: i32
    }
    fn get_max_for_color(rounds: Vec<&str>, re: Regex) -> i32 {
        rounds.iter().map(|part| {
            if let Some(caps) = re.captures(part) {
                (&caps[1]).parse::<i32>().unwrap_or(0)
            } else {
                0
            }
        }).max().unwrap_or(0)
    }

    fn day2common(slice: &str) -> MaxRGB {
        let green_re = Regex::new(r"(\d+) green").unwrap();
        let blue_re = Regex::new(r"(\d+) blue").unwrap();
        let red_re = Regex::new(r"(\d+) red").unwrap();

        let mut rounds: Vec<_> = slice.split(";").collect();

        let max_greens = get_max_for_color(rounds.clone(), green_re);
        let max_blues = get_max_for_color(rounds.clone(), blue_re);
        let max_reds = get_max_for_color(rounds.clone(), red_re);

        MaxRGB{ red: max_reds, green: max_greens, blue: max_blues }
    }

    pub fn day2part1(lines: Lines<BufReader<File>>) {
        let game_id_re = Regex::new(r"Game (\d+):").unwrap();

        let red_threshold= 12;
        let green_threshold= 13;
        let blue_threshold= 14;

        let mut sum = 0;

        for line_result in lines {
            if let Ok(line) = line_result {
                let mut game_id ;
                if let Some(caps) = game_id_re.captures(line.as_str()) {
                    game_id = (&caps[1]).parse::<i32>().unwrap();
                    // println!("Game ID = {}", game_id)
                } else {
                    println!("Failed to capture Game ID on line: {}", line);
                    continue;
                };

                let rgb = day2common(&line);

                if rgb.red <= red_threshold && rgb.blue <= blue_threshold && rgb.green <= green_threshold {
                    sum += game_id;
                }
            }
        }

        println!("{}", sum);
    }

    pub fn day2part2(lines: Lines<BufReader<File>>) {
        let mut sum = 0;

        for line_result in lines {
            if let Ok(line) = line_result {
                let rgb = day2common(&line);
                let power = rgb.red * rgb.blue * rgb.green;
                sum += power;
            }
        }

        println!("{}", sum);
    }

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

    // Day 3
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

    // Day 4
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
}