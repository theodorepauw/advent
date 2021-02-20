const INPUT: &str = include_str!("./inputs/06.txt");
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let (points, min_x, min_y, max_x, max_y) = INPUT
        .lines()
        .map(|s| {
            let parts: Vec<_> = s
                .split(", ")
                .map(|x| x.parse::<usize>().expect("parse err"))
                .collect();
            (parts[0], parts[1])
        })
        .enumerate()
        .fold(
            (
                Vec::with_capacity(INPUT.len()),
                usize::MAX,
                usize::MAX,
                0,
                0,
            ),
            |(mut points, min_x, min_y, max_x, max_y), (i, (x, y))| {
                points.push((i, (x, y)));
                (
                    points,
                    min_x.min(x),
                    min_y.min(y),
                    max_x.max(x),
                    max_y.max(y),
                )
            },
        );

    let mut counts: HashMap<usize, usize> = (0..points.len()).map(|i| (i, 0)).collect();
    let p2 = (min_y..=max_y)
        .flat_map(|y| (min_x..=max_x).zip(std::iter::repeat(y)))
        .filter(|&(x, y)| {
            let (_, id, total_dist) = points.iter().fold(
                (usize::MAX, None, 0),
                |(prev_dist, id, sum), &(i, (px, py))| {
                    let dist =
                        if x > px { x - px } else { px - x } + if y > py { y - py } else { py - y };
                    match dist.cmp(&prev_dist) {
                        Ordering::Less => (dist, Some(i), sum + dist),
                        Ordering::Equal => (dist, None, sum + dist),
                        Ordering::Greater => (prev_dist, id, sum + dist),
                    }
                },
            );

            if let Some(id) = id {
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    counts.remove(&id);
                } else if let Some(count) = counts.get_mut(&id) {
                    *count += 1;
                }
            }

            total_dist < 10000
        })
        .count();

    let p1 = counts.values().max().ok_or("no finite area!")?;
    writeln!(io::stdout(), "Day 06 Part 1: {}\nDay 06 Part 2: {}", p1, p2)?;

    Ok(())
}
