use std::io::{self, Write};
const INPUT: &str = include_str!("./inputs/01.txt");

pub fn solve() -> crate::util::Result<()> {
    let mut numbers: Vec<i64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    numbers.sort_unstable();
    let (p1, p2) = (
        part1(&numbers).expect("no sol p1"),
        part2(&numbers).expect("no sol p2"),
    );
    writeln!(io::stdout(), "Day 01 Part 1: {}\nDay 01 Part 2: {}", p1, p2)?;
    Ok(())
}

fn part1(numbers: &[i64]) -> Option<i64> {
    numbers.iter().enumerate().find_map(|(i, n)| {
        numbers[i..]
            .binary_search(&(2020 - n))
            .map(|j| n * numbers[i + j])
            .ok()
    })
}

fn part2(numbers: &[i64]) -> Option<i64> {
    numbers.iter().enumerate().find_map(|(i, n)| {
        numbers[i..].iter().enumerate().find_map(|(j, m)| {
            numbers[i + j..]
                .binary_search(&(2020 - n - m))
                .map(|k| n * m * numbers[i + j + k])
                .ok()
        })
    })
}
