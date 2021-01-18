const INPUT: &[u8] = include_bytes!("./inputs/05.txt");
const CAP_DIFF: u8 = b'a' - b'A'; // ASCII laid out such that this == b'a' b'A' == 32
use crate::util::Result;
use std::io::{self, Write};

pub fn solve() -> Result<()> {
    let react = |polymers: &[u8]| -> Vec<u8> {
        let mut stack = Vec::with_capacity(polymers.len());

        for unit in polymers {
            if stack.last().map_or(false, |&last| unit ^ last == CAP_DIFF) {
                stack.pop();
            } else {
                stack.push(*unit);
            }
        }

        stack
    };

    let reduced = react(INPUT);
    let p1 = reduced.len();
    let p2 = (b'a'..=b'z')
        .map(|unit| {
            reduced
                .iter()
                .filter(|&&x| !(x == unit || x == unit - CAP_DIFF))
                .cloned()
                .collect::<Vec<_>>()
        })
        .map(|p| react(&p).len())
        .min()
        .ok_or("no min polymer length")?;

    writeln!(io::stdout(), "Day 05 Part 1: {}\nDay 05 Part 2: {}", p1, p2)?;
    Ok(())
}
