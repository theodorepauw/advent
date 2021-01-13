const INPUT: &str = include_str!("./inputs/12.txt");
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut pipes: HashMap<String, Vec<String>> = HashMap::new();
    for line in INPUT.lines() {
        let parts: Vec<_> = line.splitn(2, " <-> ").collect();
        let main_id = parts[0];
        for sub_id in parts[1].split(", ") {
            pipes
                .entry(main_id.to_owned())
                .or_default()
                .push(sub_id.to_owned());
            pipes
                .entry(sub_id.to_owned())
                .or_default()
                .push(main_id.to_owned());
        }
    }

    let mut done = HashSet::new();
    group("0", &mut pipes, &mut done);
    let p1 = done.len();
    let mut p2 = 1;
    while !pipes.is_empty() {
        let prog = pipes
            .keys()
            .next()
            .ok_or("not empty but no keys in pipes")?
            .to_owned();
        group(&prog, &mut pipes, &mut done);
        p2 += 1;
    }
    writeln!(io::stdout(), "Day 12 Part 1: {}\nDay 12 Part 2: {}", p1, p2)?;
    Ok(())
}

fn group(prog: &str, pipes: &mut HashMap<String, Vec<String>>, done: &mut HashSet<String>) {
    if done.insert(prog.to_owned()) {
        for p in pipes.remove(prog).expect("no pipe").iter() {
            group(p, pipes, done);
        }
    }
}
