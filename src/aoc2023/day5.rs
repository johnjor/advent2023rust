use std::fs::File;
use std::io::{BufReader, Lines};

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