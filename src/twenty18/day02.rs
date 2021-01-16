const INPUT: &str = include_str!("./inputs/02.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut lines: Vec<&str> = INPUT.lines().collect();
    lines.sort_unstable();
    let (mut twos, mut threes, mut p2) = (0, 0, None);
    let mut prev = ".".repeat(lines[0].len());

    for s in lines {
        let mut letters = [0; 26];
        let (diffs, common): (Vec<_>, Vec<_>) =
            s.bytes().zip(prev.bytes()).partition(|(c1, c2)| {
                letters[(c1 - b'a') as usize] += 1;
                c1 != c2
            });

        if diffs.len() == 1 {
            p2 = Some(String::from_utf8(
                common.into_iter().map(|c| c.0).collect(),
            )?);
        }

        if letters.contains(&2) {
            twos += 1;
        }
        if letters.contains(&3) {
            threes += 1;
        }

        prev = s.to_owned();
    }
    let p1 = twos * threes;
    writeln!(
        io::stdout(),
        "Day 02 Part 1: {}\nDay 02 Part 2: {}",
        p1,
        p2.ok_or("no p2 sol found")?
    )?;
    Ok(())
}
