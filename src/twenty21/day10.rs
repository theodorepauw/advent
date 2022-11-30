const INPUT: &str = include_str!("./inputs/10.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let (mut scores, p1) = INPUT
        .lines()
        .fold((vec![], 0), |(mut scores, mut p1), line| {
            let mut open: Vec<char> = vec![];
            let mut corrupt = false;

            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => open.push(c),
                    _ => {
                        let x = open.pop().unwrap();
                        p1 += match c {
                            ')' if x != '(' => 3,
                            ']' if x != '[' => 57,
                            '}' if x != '{' => 1197,
                            '>' if x != '<' => 25137,
                            _ => {
                                continue;
                            }
                        };
                        corrupt = true;
                        break;
                    }
                }
            }

            if !corrupt {
                scores.push(open.iter().rev().fold(0usize, |score, c| {
                    (score * 5)
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => panic!("Unexpected char: {}", c),
                        }
                }));
            }

            (scores, p1)
        });

    scores.sort_unstable();
    let p2 = scores.get(scores.len() >> 1).unwrap();

    writeln!(io::stdout(), "Day 10 Part 1: {}\nDay 10 Part 2: {}", p1, p2)?;
    Ok(())
}
