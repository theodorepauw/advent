const INPUT: &str = include_str!("./inputs/13.txt");
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut scanners: HashMap<usize, Scanner> = INPUT
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .splitn(2, ": ")
                .map(|x| x.parse::<usize>().expect("parse err"))
                .collect();
            (parts[0], Scanner::with_range(parts[1]))
        })
        .collect();
    let end = *scanners.keys().max().expect("no max");
    let mut severity = 0;
    for x in 0..=end {
        severity += scanners
            .get(&x)
            .filter(|s| s.pos == 0)
            .map_or_else(|| 0, |s| x * s.range);
        scanners.values_mut().for_each(|scanner| scanner.advance());
    }
    let mut delay = 1;
    while scanners
        .iter()
        .any(|(x, s)| ((x + delay) % (2 * (s.range - 1))) == 0)
    {
        delay += 1;
    }
    let (p1, p2) = (severity, delay);
    writeln!(io::stdout(), "Day 13 Part 1: {}\nDay 13 Part 2: {}", p1, p2)?;
    Ok(())
}
struct Scanner {
    range: usize,
    pos: usize,
    down: bool,
}

impl Scanner {
    fn with_range(range: usize) -> Self {
        Scanner {
            range,
            pos: 0,
            down: true,
        }
    }

    fn advance(&mut self) {
        if self.down {
            self.pos += 1;
            if self.pos == self.range - 1 {
                self.down = false;
            }
        } else {
            self.pos -= 1;
            if self.pos == 0 {
                self.down = true;
            }
        }
    }
}
