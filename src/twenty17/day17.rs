const INPUT: usize = include!("./inputs/17.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut buffer = Vec::with_capacity(2018);
    buffer.push(0);
    let next_pos = |pos, x| -> usize { (pos + INPUT) % x + 1 };
    let mut pos = 0;
    for x in 1..2018 {
        pos = next_pos(pos, x);
        buffer.insert(pos, x);
    }
    let (p1, mut p2) = (buffer[pos + 1], buffer[1]);
    for x in 2018..50_000_000 {
        pos = next_pos(pos, x);
        if pos == 1 {
            p2 = x;
        }
    }
    writeln!(io::stdout(), "Day 17 Part 1: {}\nDay 17 Part 2: {}", p1, p2)?;
    Ok(())
}
