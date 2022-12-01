const MIN: usize = 136760;
const MAX: usize = 595730;
use crate::util::{digits, Result};
use std::io::{self, Write};

pub fn solve() -> Result<()> {
    let p1 = (MIN..=MAX)
        .filter_map(|pwd| {
            digits(pwd).try_fold((0, 0), |(adjacent, prev), d| {
                (d >= prev).then_some((if d == prev { adjacent + 1 } else { adjacent }, d))
            })
        })
        .filter(|(adj, _)| *adj > 0)
        .count();

    let p2 = (MIN..=MAX)
        .filter_map(|pwd| {
            digits(pwd).try_fold((0, (0, 0, 0)), |(mut adjacent, prev), d| {
                (d >= prev.0).then(|| {
                    if d == prev.0 && d != prev.1 {
                        adjacent += 1;
                    }
                    if d == prev.1 && prev.1 != prev.2 {
                        adjacent -= 1;
                    }
                    (adjacent, (d, prev.0, prev.1))
                })
            })
        })
        .filter(|(adj, _)| *adj > 0)
        .count();

    writeln!(io::stdout(), "Day 04 Part 1: {}\nDay 04 Part 2: {}", p1, p2)?;
    Ok(())
}
