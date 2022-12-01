const INPUT: &str = include_str!("./inputs/01.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let (mut most_cals, mut second_most_cals, mut third_most_cals) = (0, 0, 0);
    for elf_calories in INPUT.split("\n\n").map(|s| {
        s.lines()
            .map(|cals| cals.parse::<usize>().expect("cals not a number"))
            .sum::<usize>()
    }) {
        if elf_calories >= most_cals {
            third_most_cals = second_most_cals;
            second_most_cals = most_cals;
            most_cals = elf_calories;
        } else if elf_calories >= second_most_cals {
            third_most_cals = second_most_cals;
            second_most_cals = elf_calories;
        } else if elf_calories > third_most_cals {
            third_most_cals = elf_calories;
        }
    }

    let (p1, p2) = (most_cals, most_cals + second_most_cals + third_most_cals);
    writeln!(io::stdout(), "Day 01 Part 1: {}\nDay 01 Part 2: {}", p1, p2)?;
    Ok(())
}
