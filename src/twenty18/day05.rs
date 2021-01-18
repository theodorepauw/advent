const INPUT: &[u8] = include_bytes!("./inputs/05.txt");
const CAP_DIFF: u8 = b'a' - b'A';
use crate::util::Result;
use std::io::{self, Write};

pub fn solve() -> Result<()> {
    let react = |polymers: &[u8]| -> Result<Vec<bool>> {
        let mut active = vec![true; polymers.len()];
        let mut i = 0;
        while let Some(j) = active[i + 1..]
            .iter()
            .position(|&is_active| is_active)
            .map(|x| i + 1 + x)
        {
            let diff = if polymers[i] > polymers[j] {
                polymers[i] - polymers[j]
            } else {
                polymers[j] - polymers[i]
            };

            if diff == CAP_DIFF {
                active[i] = false;
                active[j] = false;

                if let Some(prev_active) = active[0..i].iter().rposition(|&is_active| is_active) {
                    i = prev_active;
                } else if let Some(fwd_offset) = if j + 1 < active.len() {
                    active[j + 1..].iter().position(|&is_active| is_active)
                } else {
                    None
                } {
                    i += fwd_offset
                } else {
                    break;
                }
            } else {
                i = j;
            }
            if i == active.len() - 1 {
                break;
            }
        }
        Ok(active)
    };

    let reduced_len = |active: &[bool]| active.iter().filter(|&&is_active| is_active).count();
    let still_active: Vec<bool> = react(INPUT)?;
    let p1 = reduced_len(&still_active);
    let p2 = (b'a'..=b'z')
        .map(|unit| {
            (0..INPUT.len())
                .filter(|&i| still_active[i])
                .map(|i| INPUT[i])
                .filter(|&x| !(x == unit || x == unit - CAP_DIFF))
                .collect::<Vec<_>>()
        })
        .filter_map(|p| react(&p).ok())
        .map(|active| reduced_len(&active))
        .min_by_key(|&len| len)
        .ok_or("no min polymer length")?;

    writeln!(io::stdout(), "Day 05 Part 1: {}\nDay 05 Part 2: {}", p1, p2)?;
    Ok(())
}
