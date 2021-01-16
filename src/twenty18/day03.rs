const INPUT: &str = include_str!("./inputs/03.txt");
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut fabric = HashMap::new();
    let mut ids = HashSet::new();
    for s in INPUT.lines() {
        let parts: Vec<_> = s.split_whitespace().collect();
        let id = &parts[0][1..];
        let offsets: Vec<usize> = parts[2]
            .split_terminator(|c| c == ',' || c == ':')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let dimensions: Vec<usize> = parts[3]
            .split('x')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        for x in offsets[0]..offsets[0] + dimensions[0] {
            for y in offsets[1]..offsets[1] + dimensions[1] {
                fabric.entry((x, y)).or_insert(vec![]).push(id);
            }
        }
        ids.insert(id);
    }

    let p1 = fabric
        .values()
        .filter(|f| f.len() > 1)
        .map(|f| {
            for id in f {
                ids.remove(id);
            }
            f
        })
        .count();
    let p2 = ids.iter().next().ok_or("no sol p2")?;
    writeln!(io::stdout(), "Day 03 Part 1: {}\nDay 03 Part 2: {}", p1, p2)?;

    Ok(())
}
