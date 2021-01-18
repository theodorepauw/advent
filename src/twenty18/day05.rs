const INPUT: &[u8] = include_bytes!("./inputs/05.txt");
const CAP_DIFF: u8 = b'a' - b'A';
use crate::util::Result;
use std::io::{self, Write};

pub fn solve() -> Result<()> {
    let polymers = INPUT.to_vec();

    let react = move |mut polymers: Vec<u8>| -> Result<usize> {
        let mut i = 0;
        loop {
            let j = i + 1;
            if j >= polymers.len() {
                break;
            }

            let diff = polymers[i]
                .checked_sub(polymers[j])
                .or_else(|| polymers[j].checked_sub(polymers[i]))
                .ok_or("couldn't calc diff")?;

            if diff == CAP_DIFF {
                polymers.drain(i..=j);
                i = i.saturating_sub(1);
            } else {
                i = j;
            }
        }
        Ok(polymers.len())
    };

    let p2 = (b'a'..=b'z')
        .map(|unit| {
            polymers
                .iter()
                .filter(|&&x| !(x == unit || x == (unit - CAP_DIFF)))
                .cloned()
                .collect()
        })
        .filter_map(|p| react(p).ok())
        .min_by_key(|&len| len)
        .ok_or("no min polymer length")?;
    let p1 = react(polymers)?;

    writeln!(io::stdout(), "Day 05 Part 1: {}\nDay 05 Part 2: {}", p1, p2)?;
    Ok(())
}
