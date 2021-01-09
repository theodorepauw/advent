use crate::util;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Write};

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").expect("couldn't compile regex");
}

const INPUT: &str = include_str!("./inputs/02.txt");

pub fn solve() -> util::Result<()> {
    let passwords: Vec<Line> = INPUT
        .lines()
        .map(|line| line.parse())
        .collect::<util::Result<_>>()?;
    let (p1, p2) = (part1(&passwords), part2(&passwords));
    writeln!(io::stdout(), "Day 02 Part 1: {}\nDay 02 Part 2: {}", p1, p2)?;
    Ok(())
}

struct Line {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl std::str::FromStr for Line {
    type Err = util::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = RE.captures(s).ok_or("line doesn't match regex")?;
        Ok(Line {
            min: m[1].parse()?,
            max: m[2].parse()?,
            letter: m[3].chars().next().ok_or("no letter specified")?,
            password: m[4].to_owned(),
        })
    }
}

fn part1(passwords: &[Line]) -> usize {
    passwords
        .iter()
        .filter(|line| {
            let count = line.password.chars().filter(|c| *c == line.letter).count();
            line.min <= count && count <= line.max
        })
        .count()
}

fn part2(passwords: &[Line]) -> usize {
    passwords
        .iter()
        .filter(|line| {
            let x = line.password.chars().nth(line.min - 1).unwrap();
            let y = line.password.chars().nth(line.max - 1).unwrap();
            (x == line.letter) ^ (y == line.letter)
        })
        .count()
}
