const INPUT: &str = include_str!("./inputs/10.txt");
use std::collections::HashSet;
use std::io::{self, Write};

type been_visited = bool;

enum CaveSize {
    Big,
    Small(been_visited), // if small, has the cave been visited yet?
}

struct Cave<'a> {
    connections: Vec<&'a str>,
    size: CaveSize,
}

pub fn solve() -> crate::util::Result<()> {
    // let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut caves: HashSet<Cave> = HashSet::new();
    let mut ends: Vec<&str> = vec![];
    let mut starts: Vec<&str> = vec![];

    // for line in INPUT.lines() {
    //     let splits: Vec<&str> = line.splitn(2, "-").collect();
    //     match splits[0] {
    //         "start" => starts.push(splits[1]),
    //         "end" => ends.push(splits[1]),
    //         _ => {
    //             caves.entry(splits[0]).or_default().push(splits[1]);
    //             caves.entry(splits[1]).or_default().push(splits[1]);
    //         }
    //     }
    // }

    for line in INPUT.lines() {}

    // writeln!(io::stdout(), "Day 12 Part 1: {}\nDay 12 Part 2: {}", p1, p2)?;
    Ok(())
}
