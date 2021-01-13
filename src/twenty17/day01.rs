const INPUT: &str = include_str!("./inputs/01.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let digits: &[u32] = &INPUT
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let sum_matches_offset = |offset: usize| -> u32 {
        digits
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| {
                if n == digits[(i + offset) % digits.len()] {
                    Some(n)
                } else {
                    None
                }
            })
            .sum()
    };
    let (p1, p2) = (sum_matches_offset(1), sum_matches_offset(digits.len() / 2));

    writeln!(io::stdout(), "Day 01 Part 1: {}\nDay 01 Part 2: {}", p1, p2)?;
    Ok(())
}
