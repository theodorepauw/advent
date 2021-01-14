const DELTAS: [(i8, i8); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
const INPUT: &str = include_str!("./inputs/14.txt");
use crate::util;
use std::collections::HashSet;
use std::io::{self, Write};

pub fn solve() -> util::Result<()> {
    let (mut p1, mut p2, mut hashes) = (0, 0, vec![]);
    for x in 0usize..128 {
        let mut lengths: Vec<_> = format!("{}-{}", INPUT, x)
            .bytes()
            .map(|b| b as usize)
            .collect();
        lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
        let k = util::tie_knots(&lengths, 64);
        let h = util::knot_hash(&k);
        p1 += h.count_ones();
        hashes.push(h);
    }
    let mut grid: HashSet<(i8, i8)> = HashSet::new();

    for (y, h) in hashes.iter().enumerate() {
        for x in 0..=127i8 {
            if h & (1 << x) != 0 {
                grid.insert((x, y as i8));
            }
        }
    }

    while !grid.is_empty() {
        let coord = *grid.iter().next().expect("len>0 but no entries");
        remove_group(coord, &mut grid);
        p2 += 1;
    }

    writeln!(io::stdout(), "Day 14 Part 1: {}\nDay 14 Part 2: {}", p1, p2)?;
    Ok(())
}

fn remove_group((x, y): (i8, i8), grid: &mut HashSet<(i8, i8)>) {
    if grid.remove(&(x, y)) {
        DELTAS
            .iter()
            .filter_map(|&(dx, dy)| Some((x.checked_sub(dx)?, y.checked_sub(dy)?)))
            .for_each(|coord| remove_group(coord, grid));
    }
}
