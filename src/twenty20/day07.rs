use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
const INPUT: &str = include_str!("./inputs/07.txt");

pub fn solve() -> crate::util::Result<()> {
    let mut bottom_up: HashMap<String, HashSet<String>> = HashMap::new();
    let mut top_down: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut bottom_most = vec![];

    for l in INPUT.lines() {
        let splits: Vec<String> = l.split_whitespace().map(|s| s.to_owned()).collect();
        let upper = splits[0..2].join("");
        if &splits[4] == "no" {
            bottom_most.push(upper);
        } else {
            let children = top_down.entry(upper.clone()).or_default();

            let mut i = 4;
            while i + 2 < splits.len() {
                let (qty, color) = (splits[i].parse::<usize>()?, splits[i + 1..i + 3].join(""));
                children.insert(color.clone(), qty);

                bottom_up.entry(color).or_default().insert(upper.clone());
                i += 4;
            }
        }
    }

    let mut ups: Vec<&String> = bottom_up["shinygold"].iter().collect();
    let mut i = 0;
    while let Some(bag) = ups.get(i) {
        if let Some(b) = bottom_up.get(*bag) {
            ups.extend_from_slice(
                &b.iter()
                    .filter(|b| !ups.contains(b))
                    .collect::<Vec<&String>>(),
            );
        }
        i += 1;
    }

    let (p1, p2) = (ups.len(), count_bags("shinygold", &top_down) - 1);
    writeln!(io::stdout(), "Day 07 Part 1: {}\nDay 07 Part 2: {}", p1, p2)?;
    Ok(())
}

fn count_bags(color: &str, top_down: &HashMap<String, HashMap<String, usize>>) -> usize {
    let contents = if let Some(children) = top_down.get(color) {
        children
            .iter()
            .map(|(c, qty)| qty * count_bags(c, &top_down))
            .sum()
    } else {
        0
    };

    contents + 1
}
