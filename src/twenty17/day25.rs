const INPUT: &str = include_str!("./inputs/25.txt");
use std::convert::TryFrom;
use std::io::{self, Write};
use std::iter::once;
use crate::util;

pub fn solve() -> util::Result<()> {
    let info: Vec<Vec<&str>> = INPUT.split("\n\n").map(|s| s.lines().collect()).collect();
    let state = str_to_state(info[0][0])?;
    let cursor = info[0][1]
        .split_whitespace()
        .nth_back(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let tape: Vec<bool> = vec![false; cursor << 1];
    let rules: Vec<Rule> = info
        .into_iter()
        .skip(1)
        .map(|v| (Rule::try_from(&v[2..5]), Rule::try_from(&v[6..9])))
        .map(|(r1, r2)| once(r1).chain(once(r2)))
        .flatten()
        .collect::<Result<_, _>>()?;

    let mut machine = TuringMachine {
        tape,
        cursor,
        state,
    };

    for _ in 0..cursor {
        machine.tick(&rules);
    }

    let p1 = machine.tape.iter().filter(|&&v| v).count();
    writeln!(io::stdout(), "Day 25 Part 1: {}", p1)?;
    Ok(())
}

struct TuringMachine {
    tape: Vec<bool>,
    cursor: usize,
    state: usize,
}

impl TuringMachine {
    fn tick(&mut self, rules: &[Rule]) {
        let val = self.tape.get_mut(self.cursor).expect("cursor OOB");
        let r = &rules[self.state << 1 | if *val { 1 } else { 0 }];
        *val = r.write;
        self.cursor = match r.shift {
            Shift::Left => self.cursor - 1,
            Shift::Right => self.cursor + 1,
        };
        self.state = r.next_state;
    }
}
struct Rule {
    write: bool,
    shift: Shift,
    next_state: usize,
}

impl TryFrom<&[&str]> for Rule {
    type Error = &'static str;
    fn try_from(v: &[&str]) -> Result<Self, Self::Error> {
        let write = &v[0][v[0].len() - 2..] == "1.";
        let shift = if &v[1][v[1].len() - 5..] == "left." {
            Shift::Left
        } else {
            Shift::Right
        };
        let next_state = str_to_state(v[2])?;
        Ok(Rule {
            write,
            shift,
            next_state,
        })
    }
}
enum Shift {
    Left,
    Right,
}

fn str_to_state(s: &str) -> Result<usize, &'static str> {
    Ok((s
        .bytes()
        .nth_back(1)
        .ok_or("state name must be a single uppercase letter")?
        - b'A') as usize)
}
