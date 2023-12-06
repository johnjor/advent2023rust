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
}