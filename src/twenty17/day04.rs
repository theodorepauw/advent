const INPUT: &str = include_str!("./inputs/04.txt");
use std::collections::HashSet;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let p1 = INPUT
        .lines()
        .filter(|line| {
            let mut memory = HashSet::new();
            line.split_whitespace().all(|word| memory.insert(word))
        })
        .count();
    let p2 = INPUT
        .lines()
        .filter(|line| {
            let mut memory = HashSet::new();
            line.split_whitespace()
                .map(|w| {
                    let mut chars = w.chars().collect::<Vec<_>>();
                    chars.sort_unstable();
                    chars
                })
                .all(|word| memory.insert(word))
        })
        .count();
    writeln!(io::stdout(), "Day 04 Part 1: {}\nDay 04 Part 2: {}", p1, p2)?;
    Ok(())
}
