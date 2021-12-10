const INPUT: &str = include_str!("./inputs/02.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    
    // Part 1
    let (depth, hpos) = INPUT.lines().map(|s| s.splitn(2, ' ').collect::<Vec<&str>>()).fold((0, 0), | (depth, hpos), instruction | {
        let magnitude: i32 = instruction[1].parse().expect("input error: magnitude not int");
        match instruction[0] {
            "forward" => (depth, hpos+magnitude),
            "down" => (depth+magnitude, hpos),
            _ => (depth-magnitude, hpos),
        }
    });

    let p1 = depth * hpos;

    let (depth, hpos, _) = INPUT.lines().map(|s| s.splitn(2, ' ').collect::<Vec<&str>>()).fold((0, 0, 0), | (depth, hpos, aim), instruction | {
        let magnitude: i32 = instruction[1].parse().expect("input error: magnitude not int");
        match instruction[0] {
            "down" => (depth, hpos, aim+magnitude),
            "up" => (depth, hpos, aim-magnitude),
            _ => (depth+magnitude*aim, hpos+magnitude, aim),
        }
    });

    let p2 = depth * hpos;

    writeln!(io::stdout(), "Day 02 Part 1: {}\nDay 02 Part 2: {}", p1, p2)?;
    Ok(())
}