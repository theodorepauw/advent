const INPUT: &str = include_str!("./inputs/07.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let positions: Vec<usize> = INPUT
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let fuel_consumption = |fuel_equation: &dyn Fn(usize) -> usize| -> usize {
        (min..max)
            .map(|a| {
                positions
                    .iter()
                    .map(|&b| fuel_equation(if b > a { b - a } else { a - b }))
                    .sum()
            })
            .min()
            .unwrap()
    };

    let p1 = fuel_consumption(&|n| n);
    let p2 = fuel_consumption(&|n| n * (n + 1) / 2);
    writeln!(io::stdout(), "Day 07 Part 1: {}\nDay 07 Part 2: {}", p1, p2,)?;
    Ok(())
}
