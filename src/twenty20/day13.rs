const INPUT: &str = include_str!("./inputs/13.txt");
use std::io::{self, Write};

lazy_static::lazy_static! {
    static ref LINES: Vec<&'static str> = INPUT.lines().collect();
    static ref TIMESTAMP: usize = LINES[0].parse::<usize>().unwrap();
    static ref BUSES: &'static str = LINES[1];
}

pub fn solve() -> crate::util::Result<()> {
    let (p1, p2) = (part1()?, part2()?);
    writeln!(io::stdout(), "Day 13 Part 1: {}\nDay 13 Part 2: {}", p1, p2)?;
    Ok(())
}

fn part1() -> crate::util::Result<usize> {
    BUSES
        .split_terminator(',')
        .filter(|&s| s != "x")
        .map(|s| {
            let id = s.parse::<usize>().unwrap();
            (id, id - *TIMESTAMP % id)
        })
        .min_by_key(|(_, wait)| *wait)
        .map(|(id, wait)| id * wait)
        .ok_or_else(|| "couldn't find minimum wait time for buses".into())
}

fn part2() -> crate::util::Result<usize> {
    let buses: Vec<(usize, usize)> = BUSES
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
