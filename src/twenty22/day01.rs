const INPUT: &str = include_str!("./inputs/01.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut elves_inventories: Vec<usize> = INPUT
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|cals| cals.parse::<usize>().expect("cals not a number"))
                .sum()
        })
        .collect();

    elves_inventories.sort_unstable();

    let p1: &usize = elves_inventories.last().expect("elves' inventory empty");
    let p2: usize = elves_inventories.iter().rev().take(3).sum();

    writeln!(io::stdout(), "Day 01 Part 1: {}\nDay 01 Part 2: {}", p1, p2)?;
    Ok(())
}
