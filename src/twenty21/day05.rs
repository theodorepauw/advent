const INPUT: &str = include_str!("./inputs/05.txt");
use std::{
    collections::HashMap,
    io::{self, Write},
    str::SplitN,
};

type Point = (i32, i32);

pub fn solve() -> crate::util::Result<()> {
    let parse_point = |line: &mut SplitN<&str>| {
        let v: Vec<i32> = line
            .next()
            .expect("invalid line")
            .splitn(2, ',')
            .map(|s| s.parse::<i32>().expect("s to usize err"))
            .collect();
        (v[0], v[1])
    };
    let min_max = |a, b| if a > b { (b, a) } else { (a, b) };
    let add_line = |map: &mut HashMap<Point, usize>, p: Point| *map.entry(p).or_insert(0) += 1;
    let count_overlaps =
        |map: HashMap<Point, usize>| map.values().filter(|&&overlaps| overlaps > 1).count();
    let mut p1: HashMap<Point, usize> = HashMap::new();
    let mut p2 = p1.clone();

    for s in INPUT.lines() {
        let mut line = s.splitn(2, " -> ");
        let (x1, y1) = parse_point(&mut line);
        let (x2, y2) = parse_point(&mut line);

        if x1 == x2 {
            let (y1, y2) = min_max(y1, y2);
            for y in y1..=y2 {
                add_line(&mut p1, (x1, y));
                add_line(&mut p2, (x1, y));
            }
        } else if y1 == y2 {
            let (x1, x2) = min_max(x1, x2);
            for x in x1..=x2 {
                add_line(&mut p1, (x, y1));
                add_line(&mut p2, (x, y1));
            }
        } else {
            let m = (y2 - y1) / (x2 - x1);
            let c = y1 - (m * x1);
            let (x1, x2) = min_max(x1, x2);
            for x in x1..=x2 {
                add_line(&mut p2, (x, m * x + c));
            }
        }
    }

    writeln!(
        io::stdout(),
        "Day 05 Part 1: {}\nDay 05 Part 2: {}",
        count_overlaps(p1),
        count_overlaps(p2)
    )?;
    Ok(())
}
