const INPUT: &str = include_str!("./inputs/03.txt");
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut wires_steps: [HashMap<(i16, i16), usize>; 2] = [HashMap::new(), HashMap::new()];
    INPUT.lines().enumerate().for_each(|(i, s)| {
        s.split(',')
            .fold(((0, 0), 0), |((mut x, mut y), mut z), s| {
                for _ in 0..s[1..].parse::<usize>().expect("parse err") {
                    match &s[0..1] {
                        "R" => x += 1,
                        "L" => x -= 1,
                        "U" => y += 1,
                        "D" => y -= 1,
                        _ => panic!("invalid mv"),
                    }
                    z += 1;
                    wires_steps[i].insert((x, y), z);
                }
                ((x, y), z)
            });
    });

    let (w1, w2) = (
        wires_steps[0].keys().collect::<HashSet<&(i16, i16)>>(),
        wires_steps[1].keys().collect::<HashSet<&(i16, i16)>>(),
    );
    let in_both = w1.intersection(&w2);
    let p1 = in_both
        .clone()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .ok_or("min failed")?;
    let p2 = in_both
        .map(|coord| wires_steps[0][coord] + wires_steps[1][coord])
        .min()
        .ok_or("min failed")?;

    writeln!(io::stdout(), "Day 03 Part 1: {}\nDay 03 Part 2: {}", p1, p2)?;

    Ok(())
}
