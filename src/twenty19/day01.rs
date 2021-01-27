const INPUT: &str = include_str!("./inputs/01.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let masses = INPUT
        .lines()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .expect("parse err");
    let calc_fuel = |mass: usize| -> Option<usize> { (mass / 3).checked_sub(2) };
    let p1: usize = masses.iter().filter_map(|m| calc_fuel(*m)).sum();
    let p2: usize = masses
        .iter()
        .map(|&m| std::iter::successors(calc_fuel(m), |fm| calc_fuel(*fm)).sum::<usize>())
        .sum();

    writeln!(io::stdout(), "Day 01 Part 1: {}\nDay 01 Part 2: {}", p1, p2)?;
    Ok(())
}
