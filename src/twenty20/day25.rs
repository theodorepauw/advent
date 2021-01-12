const INPUT: &str = include_str!("./inputs/25.txt");
const INITIAL_SUBJECT_NUMBER: usize = 7;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let public_keys: Vec<_> = INPUT
        .lines()
        .map(|s| s.parse::<usize>().expect("parse err"))
        .collect();
    let card_loop = calc_loop(public_keys[0]);
    let p1 = (0..card_loop).fold(1, |val, _| transform(val, public_keys[1]));
    writeln!(io::stdout(), "Day 25 Part 1: {}", p1)?;
    Ok(())
}

fn transform(val: usize, sn: usize) -> usize {
    (val * sn) % 20201227
}

fn calc_loop(public_key: usize) -> usize {
    let mut loop_size = 1;
    let mut val = 1;
    loop {
        val = transform(val, INITIAL_SUBJECT_NUMBER);
        if val == public_key {
            return loop_size;
        } else {
            loop_size += 1;
        }
    }
}
