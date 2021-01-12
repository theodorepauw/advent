const INPUT: &str = include_str!("./inputs/21.txt");
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut allergen_names = std::collections::HashSet::new();
    let mut allergens: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut ingredients: HashMap<&str, usize> = HashMap::new();
    for l in INPUT.lines() {
        let s: Vec<&str> = l[..l.len() - 1].splitn(2, '(').collect();
        let ings: Vec<&str> = s
            .get(0)
            .expect("no ingredients")
            .split_whitespace()
            .collect();
        let alls: Vec<&str> = s.get(1).expect("no allergens")["contains ".len()..]
            .split(", ")
            .collect();

        ings.iter().for_each(|i| {
            *ingredients.entry(i).or_default() += 1;
        });
        for a in alls {
            let contained_in = allergens.entry(a).or_default();
            match contained_in.len() {
                0 => contained_in.extend_from_slice(&ings),
                _ => contained_in.retain(|i| ings.contains(i)),
            };
            allergen_names.insert(a.to_string());
        }
    }

    let mut matches: Vec<(&str, String)> = vec![];
    let mut allergen_names: Vec<String> = allergen_names.into_iter().collect();
    allergen_names.sort_by_key(|a| allergens.get::<str>(a).expect("no a").len());

    while matches.len() < allergen_names.len() {
        for a in allergen_names.iter() {
            let ings = allergens.get::<str>(a).expect("no ings for a");
            if ings.len() == 1 {
                let i = ings.get(0).expect("no ing at 0").to_string();
                ingredients.remove::<str>(&i);
                for (_, ings) in allergens.iter_mut() {
                    ings.retain(|that_ing| that_ing != &i);
                }
                matches.push((a, i));
            }
        }
    }
    matches.sort_by_key(|(a, _)| *a);
    let p2 = matches
        .into_iter()
        .map(|(_, i)| i)
        .collect::<Vec<_>>()
        .join(",");

    let p1 = ingredients.values().sum::<usize>();
    writeln!(io::stdout(), "Day 21 Part 1: {}\nDay 21 Part 2: {}", p1, p2)?;
    Ok(())
}
