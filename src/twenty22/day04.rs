const INPUT: &str = include_str!("./inputs/04.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let parse_range = |s: &str| -> (u32, u32) {
        let (min, max) = s.split_once('-').unwrap();
        (min.parse::<u32>().unwrap(), max.parse::<u32>().unwrap())
    };

    let (p1, p2) = INPUT
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(left, right)| (parse_range(left), parse_range(right)))
        .fold(
            (0usize, 0usize),
            |(total_overlaps, partial_overlaps), ((min1, max1), (min2, max2))| {
                (
                    total_overlaps
                        + usize::from(
                            (min1 <= min2 && max2 <= max1) || (min2 <= min1 && max1 <= max2),
                        ),
                    partial_overlaps + usize::from(!(max1 < min2 || max2 < min1)),
                )
            },
        );

    writeln!(
        io::stdout(),
        "Day 01 Part 1: {:?}\nDay 01 Part 2: {:?}",
        p1,
        p2
    )?;
    Ok(())
}
