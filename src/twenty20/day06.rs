use std::io::{self, Write};
const INPUT: &str = include_str!("./inputs/06.txt");

pub fn solve() -> crate::util::Result<()> {
    let (p1, p2) = INPUT
        .split("\n\n")
        .map(|s| {
            s.lines().fold((0, u32::MAX), |(any, all), line| {
                let answered = line.bytes().fold(0, |a, q| a | (1 << (q - b'a')));
                (any | answered, all & answered)
            })
        })
        .fold((0, 0), |(p1, p2), (any, all)| {
            (p1 + any.count_ones(), p2 + all.count_ones())
        });
    writeln!(io::stdout(), "Day 06 Part 1: {}\nDay 06 Part 2: {}", p1, p2)?;
    Ok(())
}
