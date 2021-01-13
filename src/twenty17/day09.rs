const INPUT: &str = include_str!("./inputs/09.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let (mut layer, mut score, mut is_garbage, mut noncancelled_garbage) = (0, 0, false, 0);
    let mut stream = INPUT.chars();
    while let Some(c) = stream.next() {
        match c {
            '{' if !is_garbage => {
                layer += 1;
                score += layer
            }
            '}' if !is_garbage => layer -= 1,
            '<' if !is_garbage => is_garbage = true,
            '>' if is_garbage => is_garbage = false,
            '!' => {
                stream.next();
            }
            _ if is_garbage => noncancelled_garbage += 1,
            _ => {}
        }
    }
    let (p1, p2) = (score, noncancelled_garbage);

    writeln!(io::stdout(), "Day 09 Part 1: {}\nDay 09 Part 2: {}", p1, p2)?;
    Ok(())
}
