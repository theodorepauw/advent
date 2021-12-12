const INPUT: &str = include_str!("./inputs/06.txt");
use std::{
    collections::HashMap,
    io::{self, Write},
};

pub fn solve() -> crate::util::Result<()> {
    let mut timer_values = HashMap::new();
    INPUT
        .split(',')
        .map(|s| s.parse::<usize>().expect("str to usize err"))
        .for_each(|spawn_timer| *timer_values.entry(spawn_timer).or_insert(0usize) += 1);

    let tick = move |map: &HashMap<usize, usize>| {
        let mut next_iteration = HashMap::new();
        for (k, v) in map {
            if *k == 0 {
                *next_iteration.entry(6).or_insert(0) += v;
                *next_iteration.entry(8).or_insert(0) += v;
            } else {
                *next_iteration.entry(k - 1).or_insert(0) += v;
            }
        }
        next_iteration
    };

    let fish_after_iterations =
        |mut map: HashMap<usize, usize>, n: usize| -> (usize, HashMap<usize, usize>) {
            for _ in 0..n {
                map = tick(&map);
            }

            (map.values().sum(), map)
        };

    let (p1, timer_values) = fish_after_iterations(timer_values, 80);
    let (p2, _) = fish_after_iterations(timer_values, 256 - 80);
    writeln!(io::stdout(), "Day 06 Part 1: {}\nDay 06 Part 2: {}", p1, p2,)?;
    Ok(())
}
