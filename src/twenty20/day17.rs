const INPUT: &str = include_str!("./inputs/17.txt");
const P1_DIMS: usize = 3;
const P2_DIMS: usize = 4;
const CYCLES: usize = 6;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{self, Write};

lazy_static! {
    static ref P1_DELTAS: Vec<Ternary> = generate_deltas(P1_DIMS);
    static ref P2_DELTAS: Vec<Ternary> = generate_deltas(P2_DIMS);
}

type Ternary = Vec<i32>; // consists of values of -1/0/1
type Pocket = HashMap<Vec<i32>, (bool, usize)>;

fn get_ternary(mut n: i32, mut trits: usize) -> Ternary {
    let mut v = vec![-1; trits];
    while n != 0 {
        trits -= 1;
        v[trits] += n % 3;
        n /= 3;
    }
    v
}

fn generate_deltas(dimensions: usize) -> Vec<Ternary> {
    let n_deltas = 3i32.pow(dimensions as u32);
    let centre = n_deltas >> 1; // this element will be [0; d], i.e. the central point, hence it is skipped
    (0..centre)
        .chain(centre + 1..n_deltas)
        .map(|x| get_ternary(x, dimensions))
        .collect()
}

fn get_pocket(dims: usize) -> Pocket {
    let mut pocket = HashMap::new();
    let (_, _) = INPUT.chars().fold((0, 0), |(x, y), c| {
        if c == '\n' {
            (0, y + 1)
        } else {
            let mut coords = vec![x, y];
            coords.extend(std::iter::repeat(0).take(dims - 2));
            pocket.insert(coords, (c == '#', 0));
            (x + 1, y)
        }
    });
    pocket
}

pub fn solve() -> crate::util::Result<()> {
    let (mut p1_pocket, mut p2_pocket) = (get_pocket(P1_DIMS), get_pocket(P2_DIMS));
    let run_cycle = |pocket: &mut Pocket, deltas: &[Ternary]| {
        let map: Vec<Vec<i32>> = pocket.keys().cloned().collect();
        for coord in map {
            if pocket
                .get(&coord)
                .map_or(false, |(is_active, _)| *is_active)
            {
                for d in deltas {
                    let shifted_coord = coord.iter().zip(d.iter()).map(|(x, dx)| x + dx).collect();
                    pocket.entry(shifted_coord).or_insert((false, 0)).1 += 1;
                }
            }
        }

        for (is_active, active_neighbours) in pocket.values_mut() {
            if (*is_active && *active_neighbours != 2 && *active_neighbours != 3)
                || (!*is_active && *active_neighbours == 3)
            {
                *is_active ^= true;
            }
            *active_neighbours = 0;
        }
    };

    for _ in 0..CYCLES {
        run_cycle(&mut p1_pocket, &P1_DELTAS);
        run_cycle(&mut p2_pocket, &P2_DELTAS);
    }

    let count = |pocket: Pocket| pocket.values().filter(|&&b| b.0).count();
    let (p1, p2) = (count(p1_pocket), count(p2_pocket));

    writeln!(io::stdout(), "Day 17 Part 1: {}\nDay 17 Part 2: {}", p1, p2)?;
    Ok(())
}
