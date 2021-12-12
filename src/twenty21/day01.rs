const INPUT: &str = include_str!("./inputs/01.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let depths: Vec<usize> = INPUT
        .lines()
        .map(|s| s.parse().expect("Depths not all numbers."))
        .collect::<Vec<_>>();

    let p1 = depths.windows(2).filter(|&d| d[1] > d[0]).count();
    let p2 = depths.windows(4).filter(|&d| d[3] > d[0]).count();

    writeln!(io::stdout(), "Day 01 Part 1: {}\nDay 01 Part 2: {}", p1, p2)?;
    Ok(())
}
