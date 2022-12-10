const INPUT: &str = include_str!("./inputs/0.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let parse_range = |s: &str| -> (u32, u32) {
        let (min, max) = s.split_once('-').unwrap();
        (min.parse::<u32>().unwrap(), max.parse::<u32>().unwrap())
    };

    let p1 = INPUT
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(left, right)| (parse_range(left), parse_range(right)))
        .filter(|&((min1, max1), (min2, max2))| {
            if min2 >= min1 {
                max2 <= max1
            } else {
                max1 <= max2
            }
        })
        .count();

    writeln!(
        io::stdout(),
        "Day 01 Part 1: {:?}\nDay 01 Part 2: {:?}",
        p1,
        p1
    )?;
    Ok(())
}
