const INPUT: &str = include_str!("./inputs/01.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {

    let depths: Vec<usize> = INPUT
        .lines()
        .map(|s| s.parse().expect("Depths not all numbers."))
        .collect::<Vec<_>>();

    let sums_of_windows_of_three: Vec<usize> = depths.windows(3).map(|d| d.iter().sum()).collect();
    let slider = | depths: Vec<usize> | depths.windows(2).filter(|&d| d[1] > d[0]).count();        
    let p1 = slider(depths);
    let p2 = slider(sums_of_windows_of_three);

    writeln!(io::stdout(), "Day 01 Part 1: {}\nDay 01 Part 2: {}", p1, p2)?;
    Ok(())
}