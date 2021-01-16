const INPUT: &str = include_str!("./inputs/01.txt");
use crate::util;
use std::collections::HashSet;
use std::io::{self, Write};

pub fn solve() -> util::Result<()> {
    let changes: Vec<isize> = INPUT
        .lines()
        .map(|s| s.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;

    let (mut p1, mut p2) = (None, None);
    let mut seen = HashSet::new();

    for (i, freq) in changes
        .iter()
        .cycle()
        .scan(0, |sum, elem| {
            *sum += *elem;
            Some(*sum) // is the initial 0 considered a 'reached' freq?
        })
        .enumerate()
    {
        if i == changes.len() - 1 {
            p1 = Some(freq);
        }
        if !seen.insert(freq) && p2.is_none() {
            p2 = Some(freq)
        }
        if let (Some(p1), Some(p2)) = (p1, p2) {
            writeln!(io::stdout(), "Day 01 Part 1: {}\nDay 01 Part 2: {}", p1, p2)?;
            return Ok(());
        }
    }

    writeln!(io::stdout(), "No solution for Day 01 found.")?;
    Ok(())
}
