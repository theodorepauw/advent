const INPUT: &str = include_str!("./inputs/07.txt");
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let (mut children, mut weights): (HashMap<String, Vec<String>>, HashMap<&str, i32>) =
        (HashMap::new(), HashMap::new());
    let (mut bottom, mut not_bottom) = (HashSet::new(), HashSet::new());
    for line in INPUT.lines() {
        let s: Vec<&str> = line.splitn(4, char::is_whitespace).collect();
        let name = s[0];
        let weight = (s[1][1..s[1].len() - 1]).to_owned().parse::<i32>()?;
        bottom.insert(name.to_owned());
        weights.insert(name, weight);
        if let Some(s) = s.get(3) {
            for c in s.split(", ") {
                not_bottom.insert(c.to_owned());
                children
                    .entry(name.to_owned())
                    .or_default()
                    .push(c.to_owned());
            }
        }
    }
    let p1 = bottom.difference(&not_bottom).next().ok_or("no p1 sol")?;
    writeln!(io::stdout(), "Day 07 Part 1: {}", p1)?;

    fn get_weight(
        prog: &str,
        weights: &HashMap<&str, i32>,
        children: &HashMap<String, Vec<String>>,
    ) -> i32 {
        weights[prog]
            + children.get(prog).map_or_else(
                || 0,
                |c| {
                    c.iter()
                        .map(|x| get_weight(x, weights, children))
                        .sum::<i32>()
                },
            )
    }

    let mut prog = p1.to_owned();
    let mut imbalance: i32 = 0;
    let p2 = loop {
        let mut w: Vec<_> = children
            .remove(&prog)
            .ok_or("no children")?
            .into_iter()
            .map(|p| (get_weight(&p, &weights, &children), p))
            .collect();
        w.sort_unstable_by_key(|(w, _)| *w);

        if w[0].0 != w[w.len() - 1].0 {
            if w[0].0 != w[1].0 {
                imbalance = w[1].0 - w[0].0;
                prog = w[0].1.clone();
            } else {
                imbalance = w[0].0 - w[w.len() - 1].0;
                prog = w[w.len() - 1].1.clone();
            }
        } else {
            break weights[prog.as_str()] + imbalance;
        }
    };
    writeln!(io::stdout(), "Day 07 Part 2: {}", p2)?;
    Ok(())
}
