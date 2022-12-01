const INPUT: &str = include_str!("./inputs/08.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    // 1 uses 2 signal lines -> should be segments c and f
    // 7 uses 3 signal lines -> should be segments a, c, and f -> find a
    // 4 uses 4 signal lines

    let p1 = INPUT
        .lines()
        .flat_map(|s| s.rsplit(" | ").next().unwrap().split_whitespace())
        .filter(|&s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count();

    // copied timvisee's solution because I'm way too lazy

    let p2 = include_bytes!("./inputs/08.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut part = line.splitn(2, |&b| b == b'|');
            let mut input = part.next().unwrap().split(|&b| b == b' ');
            let one = input.clone().find(|d| d.len() == 2).unwrap();
            let four = input.find(|d| d.len() == 4).unwrap();
            part.next()
                .unwrap()
                .split(|&b| b == b' ')
                .skip(1)
                .map(|d| match d.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    len => match (
                        len,
                        d.iter().filter(|&b| one.contains(b)).count(),
                        d.iter().filter(|&b| four.contains(b)).count(),
                    ) {
                        (5, 1, 3) => 5,
                        (5, 2, 3) => 3,
                        (5, _, 2) => 2,
                        (6, 1, _) => 6,
                        (6, _, 3) => 0,
                        (6, _, 4) => 9,
                        _ => unreachable!(),
                    },
                })
                .enumerate()
                .fold(0, |sum, (i, n)| sum + n * 10_u32.pow(3 - i as u32))
        })
        .sum::<u32>();

    writeln!(io::stdout(), "Day 08 Part 1: {}\nDay 08 Part 2: {}", p1, p2,)?;
    Ok(())
}
