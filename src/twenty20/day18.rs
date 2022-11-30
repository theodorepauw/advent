const INPUT: &str = include_str!("./inputs/18.txt");
use crate::util;
use std::io::{self, Write};

enum Version {
    One,
    Two,
}

pub fn solve() -> util::Result<()> {
    let (p1, p2) = INPUT
        .lines()
        .map(|s| (group(s, &Version::One), group(s, &Version::Two)))
        .try_fold((0, 0), |(p1, p2), (l1, l2)| match (l1, l2) {
            (Ok(a), Ok(b)) => Some((p1 + a, p2 + b)),
            _ => None,
        })
        .ok_or("couldn't solve")?;
    writeln!(io::stdout(), "Day 18 Part 1: {}\nDay 18 Part 2: {}", p1, p2)?;
    Ok(())
}

fn group(s: &str, v: &Version) -> util::Result<u64> {
    if !s.contains('(') {
        return match v {
            Version::One => solve_v1(s),
            Version::Two => solve_v2(s),
        };
    }

    let mut stack = vec![];
    let mut result = vec![];
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            result.push((stack.pop().expect(") unmatched"), i));
        }
    }
    let (start, end) = result.get(0).expect("no op");
    let expression = format!("{}", group(&s[start + 1..*end], v)?);
    let simplified = &[&s[..*start], &expression, &s[end + 1..]].join("");
    group(&simplified, v)
}

fn solve_v1(s: &str) -> crate::util::Result<u64> {
    Ok(s.split_whitespace()
        .try_fold((0, '+'), |(result, operator), s| {
            util::Result::Ok(match (s.chars().next().ok_or("no chars in s")?, operator) {
                ('+', _) => (result, '+'),
                ('*', _) => (result, '*'),
                (_, '+') => (result + s.parse::<u64>()?, operator),
                (_, '*') => (result * s.parse::<u64>()?, operator),
                _ => panic!("unrecognised str: {}!", s),
            })
        })?
        .0)
}

fn solve_v2(s: &str) -> crate::util::Result<u64> {
    s.split('*')
        .map(|expr| solve_v1(expr))
        .try_fold(1, |product, sol| sol.map(|x| x * product))
}
