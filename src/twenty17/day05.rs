const INPUT: &str = include_str!("./inputs/05.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut jumps: Vec<i32> = INPUT
        .lines()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()?;
    let mut p2_jumps = jumps.clone();
    let (mut index, mut p1) = (0, 0);
    while let Some(j) = jumps.get_mut(index as usize) {
        index += *j;
        *j += 1;
        p1 += 1;
    }
    let (mut index, mut p2) = (0, 0);
    while let Some(j) = p2_jumps.get_mut(index as usize) {
        index += *j;
        *j += if *j > 2 { -1 } else { 1 };
        p2 += 1;
    }
    writeln!(io::stdout(), "Day 05 Part 1: {}\nDay 05 Part 2: {}", p1, p2)?;
    Ok(())
}
