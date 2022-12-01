const INPUT: &str = include_str!("./inputs/04.txt");
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut records: Vec<&str> = INPUT.lines().collect();
    records.sort_unstable();

    let mut guards: HashMap<&str, usize> = HashMap::new();
    let mut minutes: HashMap<&str, HashMap<usize, usize>> = HashMap::new();
    let mut guard = "None";
    let mut start = 60;
    for r in records {
        let parts: Vec<_> = r.split_whitespace().collect();
        match parts[2] {
            "Guard" => guard = &parts[3][1..],
            "falls" => start = parts[1][3..5].parse::<usize>()?,
            "wakes" => {
                let end = parts[1][3..5].parse::<usize>()?;
                *guards.entry(guard).or_default() += end - start;
                for min in start..end {
                    *minutes.entry(guard).or_default().entry(min).or_default() += 1;
                }
            }
            _ => panic!("unknown fmt"),
        }
    }

    let sleepiest_guard = guards
        .iter()
        .max_by_key(|(_, &v)| v)
        .ok_or("no sleepy guard")?
        .0;
    let g = sleepiest_guard.parse::<usize>()?;

    let m = minutes
        .get(sleepiest_guard)
        .ok_or("no minutes for sleepy guard")?
        .iter()
        .max_by_key(|(_, &v)| v)
        .map(|(k, _)| k)
        .ok_or("no max min")?;

    let p1 = g * m;
    let p2: usize = minutes
        .into_iter()
        .flat_map(|(k, v)| std::iter::repeat(k).zip(v.into_iter()))
        .max_by_key(|(_, (_, v))| *v)
        .and_then(|(id, (min, _))| id.parse::<usize>().ok().map(|id| id * min))
        .ok_or("no sol p2")?;

    writeln!(io::stdout(), "Day 04 Part 1: {}\nDay 04 Part 2: {}", p1, p2)?;
    Ok(())
}
