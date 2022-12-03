const INPUT: &str = include_str!("./inputs/12.txt");
use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Cave {
    connections: Vec<String>,
    small: bool,
}

pub fn solve() -> crate::util::Result<()> {
    let mut caves: HashMap<&str, Cave> = HashMap::new();

    // First, load all of the caves into the system
    for line in INPUT.lines() {
        let (id1, id2) = line.split_once('-').expect("invalid cave line");
        let (cave1, cave2) = (id1.parse::<Cave>()?, id2.parse::<Cave>()?);
        caves
            .entry(id1)
            .or_insert(cave1)
            .connections
            .push(id2.to_owned());
        caves
            .entry(id2)
            .or_insert(cave2)
            .connections
            .push(id1.to_owned());
    }

    let p1 = get_paths("start", &caves, Vec::with_capacity(32), false);
    let p2 = get_paths("start", &caves, Vec::with_capacity(32), true);

    writeln!(
        io::stdout(),
        "Day 12 Part 1: {:?}\nDay 12 Part 2: {:?}",
        p1,
        p2,
    )?;
    Ok(())
}

fn get_paths<'a>(
    cave_id: &'a str,
    graph: &HashMap<&str, Cave>,
    mut seen: Vec<&'a str>,
    mut allow_double_visit: bool,
) -> usize {
    let current_cave = &graph[cave_id];

    if cave_id == "end" {
        1
    } else {
        if current_cave.small {
            if seen.contains(&cave_id) {
                if allow_double_visit && cave_id != "start" {
                    allow_double_visit = false;
                } else {
                    return 0;
                }
            } else {
                seen.push(cave_id);
            }
        }

        current_cave
            .connections
            .iter()
            .map(|next_cave| get_paths(next_cave, graph, seen.clone(), allow_double_visit))
            .sum()
    }
}

impl FromStr for Cave {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let connections = vec![];
        let small = match s.chars().next().expect("no cave id to parse") {
            'a'..='z' => true,
            'A'..='Z' => false,
            _ => return Err("cave id doesn't start with letter"),
        };

        Ok(Cave { connections, small })
    }
}
