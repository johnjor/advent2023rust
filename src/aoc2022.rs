pub mod aoc2022 {
    use std::fs::File;
    use std::io::{BufReader, Lines};

    pub fn day1part1(lines: Lines<BufReader<File>>) {
        let mut current_elf = 0;
        let mut current_elfs_calories = 0;
        let mut max_elf = 0;
        let mut max_elfs_calories = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                match ip.as_str() {
                    "" => {
                        if current_elfs_calories > max_elfs_calories {
                            max_elfs_calories = current_elfs_calories;
                            max_elf = current_elf;
                        }
                        current_elf += 1;
                        current_elfs_calories = 0
                    },
                    _ => current_elfs_calories += ip.parse::<i32>().unwrap()
                }
            }
        }

        // Check once more against the last elf
        if current_elfs_calories > max_elfs_calories {
            max_elfs_calories = current_elfs_calories;
            max_elf = current_elf;
        }

        println!("{}", max_elfs_calories)
    }

}