const INPUT: &str = include_str!("./inputs/13.txt");
use crate::util;
use std::io::{self, Write};

pub fn solve() -> util::Result<()> {
    let lines: Vec<&str> = INPUT.lines().collect();
    let timestamp = lines[0].parse::<usize>().unwrap();
    let buses: &str = lines[1];

    let (p1, p2) = (part1(buses, timestamp)?, part2(buses)?);
    writeln!(io::stdout(), "Day 13 Part 1: {}\nDay 13 Part 2: {}", p1, p2)?;
    Ok(())
}

fn part1(buses: &str, timestamp: usize) -> util::Result<usize> {
    buses
        .split_terminator(',')
        .filter(|&s| s != "x")
        .map(|s| {
            let id = s.parse::<usize>().unwrap();
            (id, id - timestamp % id)
        })
        .min_by_key(|(_, wait)| *wait)
        .map(|(id, wait)| id * wait)
        .ok_or_else(|| "couldn't find minimum wait time for buses".into())
}

fn part2(buses: &str) -> util::Result<usize> {
    let buses: Vec<(usize, usize)> = buses
        .split_terminator(',')
        .enumerate()
        .filter(|&(_, bus)| bus != "x")
        .map(|(offset, id)| id.parse::<usize>().map(|id| (id, offset)))
        .collect::<Result<_, _>>()?;

    let (timestamp, _) = buses.iter().skip(1).fold(
        (buses[0].0, buses[0].0),
        |(mut time, step), (id, offset)| {
            while (time + offset) % id != 0 {
                time += step;
            }
            (time, step * id) // unncessary to do the whole lcm rigmarole for the steps & bus ids because ids are prime
        },
    );

    Ok(timestamp)
}
