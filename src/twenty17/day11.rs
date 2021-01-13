const INPUT: &str = include_str!("./inputs/11.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let (mut x, mut y, mut z, mut p2) = (0, 0, 0, 0);
    for dir in INPUT.split(',') {
        match dir {
            "n" => {
                y += 1;
                z -= 1;
            }
            "s" => {
                y -= 1;
                z += 1;
            }
            "nw" => {
                x -= 1;
                y += 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "ne" => {
                x += 1;
                z -= 1;
            }
            "sw" => {
                x -= 1;
                z += 1;
            }
            _ => panic!("unexpected direction"),
        }
        p2 = p2.max(x).max(y).max(z);
    }
    let p1 = x.max(y).max(z);
    writeln!(io::stdout(), "Day 11 Part 1: {}\nDay 11 Part 2: {}", p1, p2)?;
    Ok(())
}
