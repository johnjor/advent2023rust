pub mod aoc2023 {
    use std::cell::RefCell;
    use std::fs::File;
    use std::io::{BufReader, Lines};
    use regex::Regex;
    use std::collections::{HashMap, HashSet};
    use std::fmt;
    use std::hash::Hash;
    use std::rc::Rc;
    use num::integer::lcm;

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

    // Day 5
    fn process_rules(input: i64, rules: &Vec<Vec<i64>>) -> i64 {
        for rule in rules {
            if input >= rule[1] && input < rule[1] + rule[2] {
                return input - rule[1] + rule[0];
            }
        }
        return input;
    }

    fn process_rules_reverse(input: i64, rules: &Vec<Vec<i64>>) -> i64 {
        for rule in rules {
            if input >= rule[0] && input < rule[0] + rule[2] {
                return input - rule[0] + rule[1]
            }
        }
        return input
    }

    pub fn day5part1(mut lines: Lines<BufReader<File>>) {
        let binding = lines.next().unwrap().unwrap();
        let seeds: Vec<i64> = binding.as_str()
            .split(':').nth(1).unwrap().trim().split(' ')
            .map(|x| { x.parse::<i64>().unwrap() })
            .collect();

        let mut seed_to_soil: Vec<Vec<i64>> = vec![];
        let mut soil_to_fertilizer: Vec<Vec<i64>> = vec![];
        let mut fertilizer_to_water: Vec<Vec<i64>> = vec![];
        let mut water_to_light: Vec<Vec<i64>> = vec![];
        let mut light_to_temperature: Vec<Vec<i64>> = vec![];
        let mut temperature_to_humidity: Vec<Vec<i64>> = vec![];
        let mut humidity_to_location: Vec<Vec<i64>> = vec![];

        let mut collect_seed_to_soil = false;
        let mut collect_soil_to_fertilizer = false;
        let mut collect_fertilizer_to_water = false;
        let mut collect_water_to_light = false;
        let mut collect_light_to_temperature = false;
        let mut collect_temperature_to_humidity = false;
        let mut collect_humidity_to_location = false;

        for line_result in lines {

            if let Ok(line) = line_result {
                if line == "seed-to-soil map:" {
                    collect_seed_to_soil = true;
                    continue
                } else if line == "soil-to-fertilizer map:" {
                    collect_soil_to_fertilizer = true;
                    continue
                } else if line == "fertilizer-to-water map:" {
                    collect_fertilizer_to_water = true;
                    continue
                } else if line == "water-to-light map:" {
                    collect_water_to_light = true;
                    continue
                } else if line == "light-to-temperature map:" {
                    collect_light_to_temperature = true;
                    continue
                } else if line == "temperature-to-humidity map:" {
                    collect_temperature_to_humidity = true;
                    continue
                } else if line == "humidity-to-location map:" {
                    collect_humidity_to_location = true;
                    continue
                } else if line == "" {
                    collect_seed_to_soil = false;
                    collect_soil_to_fertilizer = false;
                    collect_fertilizer_to_water = false;
                    collect_water_to_light = false;
                    collect_light_to_temperature = false;
                    collect_temperature_to_humidity = false;
                    collect_humidity_to_location = false;
                    continue
                }

                if collect_seed_to_soil {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    seed_to_soil.push(temp);
                }

                if collect_soil_to_fertilizer {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    soil_to_fertilizer.push(temp);
                }

                if collect_fertilizer_to_water {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    fertilizer_to_water.push(temp);
                }

                if collect_water_to_light {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    water_to_light.push(temp);
                }

                if collect_light_to_temperature {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    light_to_temperature.push(temp);
                }

                if collect_temperature_to_humidity {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    temperature_to_humidity.push(temp);
                }

                if collect_humidity_to_location {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    humidity_to_location.push(temp);
                }
            }
        }

        let soil: Vec<i64> = seeds.into_iter().map(|x| {
            process_rules(x, &seed_to_soil)
        }).collect();

        let fertilizer: Vec<i64> = soil.into_iter().map(|x| {
            process_rules(x, &soil_to_fertilizer)
        }).collect();

        let water: Vec<i64> = fertilizer.into_iter().map(|x| {
            process_rules(x, &fertilizer_to_water)
        }).collect();

        let light: Vec<i64> = water.into_iter().map(|x| {
            process_rules(x, &water_to_light)
        }).collect();

        let temperature: Vec<i64> = light.into_iter().map(|x| {
            process_rules(x, &light_to_temperature)
        }).collect();

        let humidity: Vec<i64> = temperature.into_iter().map(|x| {
            process_rules(x, &temperature_to_humidity)
        }).collect();

        let location: Vec<i64> = humidity.into_iter().map(|x| {
            process_rules(x, &humidity_to_location)
        }).collect();

        println!("{}", location.into_iter().min().unwrap())
    }

    pub fn day5part2(mut lines: Lines<BufReader<File>>) {
        let binding = lines.next().unwrap().unwrap();
        let seeds: Vec<i64> = binding.as_str()
            .split(':').nth(1).unwrap().trim().split(' ')
            .map(|x| { x.parse::<i64>().unwrap() })
            .collect();

        let mut seed_to_soil: Vec<Vec<i64>> = vec![];
        let mut soil_to_fertilizer: Vec<Vec<i64>> = vec![];
        let mut fertilizer_to_water: Vec<Vec<i64>> = vec![];
        let mut water_to_light: Vec<Vec<i64>> = vec![];
        let mut light_to_temperature: Vec<Vec<i64>> = vec![];
        let mut temperature_to_humidity: Vec<Vec<i64>> = vec![];
        let mut humidity_to_location: Vec<Vec<i64>> = vec![];

        let mut collect_seed_to_soil = false;
        let mut collect_soil_to_fertilizer = false;
        let mut collect_fertilizer_to_water = false;
        let mut collect_water_to_light = false;
        let mut collect_light_to_temperature = false;
        let mut collect_temperature_to_humidity = false;
        let mut collect_humidity_to_location = false;

        for line_result in lines {

            if let Ok(line) = line_result {
                if line == "seed-to-soil map:" {
                    collect_seed_to_soil = true;
                    continue
                } else if line == "soil-to-fertilizer map:" {
                    collect_soil_to_fertilizer = true;
                    continue
                } else if line == "fertilizer-to-water map:" {
                    collect_fertilizer_to_water = true;
                    continue
                } else if line == "water-to-light map:" {
                    collect_water_to_light = true;
                    continue
                } else if line == "light-to-temperature map:" {
                    collect_light_to_temperature = true;
                    continue
                } else if line == "temperature-to-humidity map:" {
                    collect_temperature_to_humidity = true;
                    continue
                } else if line == "humidity-to-location map:" {
                    collect_humidity_to_location = true;
                    continue
                } else if line == "" {
                    collect_seed_to_soil = false;
                    collect_soil_to_fertilizer = false;
                    collect_fertilizer_to_water = false;
                    collect_water_to_light = false;
                    collect_light_to_temperature = false;
                    collect_temperature_to_humidity = false;
                    collect_humidity_to_location = false;
                    continue
                }

                if collect_seed_to_soil {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    seed_to_soil.push(temp);
                }

                if collect_soil_to_fertilizer {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    soil_to_fertilizer.push(temp);
                }

                if collect_fertilizer_to_water {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    fertilizer_to_water.push(temp);
                }

                if collect_water_to_light {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    water_to_light.push(temp);
                }

                if collect_light_to_temperature {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    light_to_temperature.push(temp);
                }

                if collect_temperature_to_humidity {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    temperature_to_humidity.push(temp);
                }

                if collect_humidity_to_location {
                    let temp = line.as_str().split(' ')
                        .map(|x| { x.parse::<i64>().unwrap() })
                        .collect();
                    humidity_to_location.push(temp);
                }
            }
        }

        let location_has_seed = |loc: i64| -> bool {
            let humidity = process_rules_reverse(loc, &humidity_to_location);
            let temperature = process_rules_reverse(humidity, &temperature_to_humidity);
            let light = process_rules_reverse(temperature, &light_to_temperature);
            let water = process_rules_reverse(light, &water_to_light);
            let fertilizer = process_rules_reverse(water, &fertilizer_to_water);
            let soil = process_rules_reverse(fertilizer, &soil_to_fertilizer);
            let seed = process_rules_reverse(soil, &seed_to_soil);

            let mut i = 0;
            while i < seeds.len() {
                if seed >= seeds[i] && seed < seeds[i] + seeds[i+1] {
                    return true
                }
                i += 2;
            }

            false
        };

        // Verify sample
        // let result = location_has_seed(46);
        // println!("{} = true", result);
        // let result = location_has_seed(45);
        // println!("{} = false", result);
        // let result = location_has_seed(0);
        // println!("{} = false", result);

        let start: i64 = 137517046;
        let mut check_loc: i64 = start;

        loop {
            let result = location_has_seed(check_loc);
            if result {
                println!("{} {}", check_loc, result);
            }
            check_loc -= 10;
            if check_loc < 0 {
                break;
            }
        }
    }

    // Day 6
    pub fn day6part1(mut lines: Lines<BufReader<File>>) {
        let time_line = lines.next().unwrap().unwrap();
        let times: Vec<usize> = time_line.as_str()
            .split(':').nth(1).unwrap().trim().split(' ')
            .filter_map(|x| { x.parse::<usize>().ok() })
            .collect();

        let distance_line = lines.next().unwrap().unwrap();
        let distances: Vec<usize> = distance_line.as_str()
            .split(':').nth(1).unwrap().trim().split(' ')
            .filter_map(|x| { x.parse::<usize>().ok() })
            .collect();

        // println!("{:?}", times);
        // println!("{:?}", distances);

        let mut result: usize = 0;

        for (time, dist) in times.into_iter().zip(distances.into_iter()) {
            let start = (time as f32 / 2f32).floor() as usize;
            let mut i: usize = 0;
            let mut d: usize = (time - (start - i)) * (start - i);
            let mut count: usize = 0;

            while d > dist {
                count += 2;
                i += 1;
                d = (time - (start - i)) * (start - i);
            }

            if time % 2 == 0 {
                count -= 1;
            }

            // println!("time={}, dist={}, count={}", time, dist, count);
            if result == 0 {
                result = count;
            } else {
                result *= count;
            }
        }
        println!("{}", result);
    }

    pub fn day6part2(mut lines: Lines<BufReader<File>>) {
        let time_line = lines.next().unwrap().unwrap();
        let time = time_line.as_str()
            .split(':').nth(1).unwrap().trim().split(' ')
            .filter(|x| { x.len() > 0 })
            .collect::<Vec<_>>().join("").parse::<usize>().unwrap();

        let distance_line = lines.next().unwrap().unwrap();
        let dist = distance_line.as_str()
            .split(':').nth(1).unwrap().trim().split(' ')
            .filter(|x| { x.len() > 0 })
            .collect::<Vec<_>>().join("").parse::<usize>().unwrap();

        println!("{:?}", time);
        println!("{:?}", dist);

        let start = (time as f64 / 2f64).floor() as usize;
        let mut i: usize = 0;
        let mut d: usize = (time - (start - i)) * (start - i);
        let mut count: usize = 0;

        while d > dist {
            count += 2;
            i += 1;
            d = (time - (start - i)) * (start - i);
        }

        if time % 2 == 0 {
            count -= 1;
        }

        println!("{}", count);
    }

    // Day 7
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

    // Day 8
    type TreeNodeRef = Rc<RefCell<Node>>;

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
}