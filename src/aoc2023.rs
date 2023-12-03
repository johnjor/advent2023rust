pub mod aoc2023 {
    use std::fs::File;
    use std::io::{BufReader, Lines};
    use regex::Regex;

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

    // Day 3
}