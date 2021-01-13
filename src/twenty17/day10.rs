const INPUT: &str = include_str!("./inputs/10.txt");
const MARKS: usize = 256;
use crate::util;
use std::io::{self, Write};

pub fn solve() -> util::Result<()> {
    let p1_lengths: Vec<_> = INPUT
        .split(',')
        .map(|s| s.parse::<usize>().expect("parse err"))
        .filter(|&x| x <= MARKS)
        .collect();
    let mut p2_lengths: Vec<_> = INPUT.bytes().map(|b| b as usize).collect();
    p2_lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let p1_circle = util::tie_knots(&p1_lengths, 1);
    let p2_circle = util::tie_knots(&p2_lengths, 64);

    let p1 = p1_circle[0] * p1_circle[1];
    let p2 = util::knot_hash(&p2_circle);
    writeln!(
        io::stdout(),
        "Day 10 Part 1: {}\nDay 10 Part 2: {:x?}",
        p1,
        p2,
    )?;
    Ok(())
}
