use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../advent2022/day1/official.txt") {

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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
