use std::collections::HashMap;
use std::io::{self, Write};

const INPUT: &str = include_str!("./inputs/10.txt");

pub fn solve() -> crate::util::Result<()> {
    let mut adapters: Vec<usize> = INPUT
        .lines()
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()?;
    adapters.push(0);
    adapters.sort_unstable();
    adapters.push(adapters.last().expect("no last element") + 3);

    let (one, three) =
        adapters
            .windows(2)
            .fold((0, 0), |(o, t), slice| match slice[1] - slice[0] {
                1 => (o + 1, t),
                3 => (o, t + 1),
                _ => unreachable!(),
            });

    let mut paths: HashMap<usize, usize> = HashMap::new();
    let (p1, p2) = (one * three, dynamic_count(0, &mut paths, &adapters));

    writeln!(io::stdout(), "Day 10 Part 1: {}\nDay 10 Part 2: {}", p1, p2)?;
    Ok(())
}

fn dynamic_count(i: usize, paths: &mut HashMap<usize, usize>, chain: &[usize]) -> usize {
    if let Some(&p) = paths.get(&i) {
        return p; // path already evaluated
    } else if i == chain.len() - 1 {
        return 1; // only 1 way to get to the end
    }

    let mut count = 0;
    for j in i + 1..i + 4 {
        if j > chain.len() - 1 {
            break;
        }
        if chain[j] - chain[i] <= 3 {
            count += dynamic_count(j, paths, chain);
        }
    }
    paths.insert(i, count);
    count
}
