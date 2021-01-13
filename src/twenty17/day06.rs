const INPUT: &str = include_str!("./inputs/06.txt");
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut memory_banks: Vec<usize> = INPUT
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let (mut configs, mut cycles, len) = (HashMap::new(), 0, memory_banks.len());

    let (p1, p2) = loop {
        if let Some(last_seen) = configs.insert(memory_banks.clone(), cycles) {
            break (cycles, cycles - last_seen);
        }
        let (index, &blocks) = memory_banks
            .iter()
            .enumerate()
            .max_by_key(|&(index, item)| (item, usize::MAX - index))
            .expect("no max");
        *memory_banks.get_mut(index).expect("index invalid") = 0;
        for b in 1..=blocks {
            *memory_banks.get_mut((index + b) % len).expect("OOB") += 1;
        }
        cycles += 1;
    };
    writeln!(io::stdout(), "Day 06 Part 1: {}\nDay 06 Part 2: {}", p1, p2)?;
    Ok(())
}
