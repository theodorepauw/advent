const INPUT: &str = include_str!("./inputs/02.txt");
use std::io::{self, Write};
use std::iter::repeat;

pub fn solve() -> crate::util::Result<()> {
    let spreadsheet = INPUT
        .lines()
        .map(|s| s.split_whitespace().filter_map(|s| s.parse::<usize>().ok()));

    let p1: usize = spreadsheet
        .clone()
        .map(|row| {
            let (min, max) = row.fold((usize::MAX, 0), |(min, max), n| (min.min(n), max.max(n)));
            max - min
        })
        .sum();

    let p2: usize = spreadsheet
        .map(|row| {
            let row: Vec<usize> = row.collect();
            (0..row.len() - 1)
                .flat_map(|i| repeat(i).zip(i + 1..row.len()))
                .find_map(|(i, j)| {
                    if row[i] % row[j] == 0 {
                        Some(row[i] / row[j])
                    } else if row[j] % row[i] == 0 {
                        Some(row[j] / row[i])
                    } else {
                        None
                    }
                })
                .expect("no divisible nums")
        })
        .sum();

    writeln!(io::stdout(), "Day 02 Part 1: {}\nDay 02 Part 2: {}", p1, p2)?;
    Ok(())
}
