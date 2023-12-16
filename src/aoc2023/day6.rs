use std::fs::File;
use std::io::{BufReader, Lines};

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