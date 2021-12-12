const INPUT: &str = include_str!("./inputs/09.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let row_length = INPUT.find('\n').unwrap();
    let heightmap: Vec<u32> = INPUT.chars().filter_map(|c| c.to_digit(10)).collect();

    let p1: u32 = heightmap
        .iter()
        .enumerate()
        .map(|(i, height)| {
            let adjacent = [
                (i > row_length-1).then(|| i - row_length),
                ((i + 1) % row_length != 0).then(|| i + 1),
                (i + row_length < heightmap.len()).then(|| i + row_length),
                (i % row_length != 0).then(|| i - 1),
            ];

            if adjacent
                .iter()
                .all(|&exists| exists.map(|j| height < &heightmap[j]).unwrap_or(true))
            {
                height + 1
            } else {
                0
            }
        })
        .sum();

    writeln!(io::stdout(), "Day 08 Part 1: {}\nDay 08 Part 2: {}", p1, p1,)?;
    Ok(())
}
