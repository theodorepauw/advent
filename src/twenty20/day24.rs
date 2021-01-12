const INPUT: &str = include_str!("./inputs/24.txt");
type Coord = (i32, i32, i32);

const CENTRE_TILE: Coord = (0, 0, 0);
const DELTAS: [Coord; 6] = [
    (1, 0, -1), // EAST
    (-1, 0, 1), // WEST
    (0, 1, -1), // NORTH EAST
    (-1, 1, 0), // NORTH WEST
    (1, -1, 0), // SOUTH EAST
    (0, -1, 1), // SOUTH WEST
];

use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut tiles: HashMap<Coord, bool> = HashMap::new(); // default is white
    writeln!(
        io::stdout(),
        "Day 24 Part 1: {}\nDay 24 Part 2: {}",
        part1(&mut tiles),
        part2(tiles)
    )?;
    Ok(())
}

fn part1(tiles: &mut HashMap<Coord, bool>) -> usize {
    for s in INPUT.lines() {
        *tiles.entry(find_coord(s, CENTRE_TILE)).or_default() ^= true;
    }
    tiles.retain(|_, black| *black);
    tiles.len()
}

fn part2(mut tiles: HashMap<Coord, bool>) -> usize {
    let mut neighbours: HashMap<Coord, usize> = HashMap::new();
    for _ in 0..100 {
        neighbours.clear();
        for (x, y, z) in tiles.keys() {
            for (dx, dy, dz) in &DELTAS {
                *neighbours.entry((x + dx, y + dy, z + dz)).or_insert(0) += 1;
            }
        }

        tiles = neighbours
            .iter()
            .filter(|(t, &n)| n == 2 || (n == 1 && *tiles.get(t).unwrap_or(&false)))
            .map(|(&t, _)| (t, true))
            .collect();
    }
    tiles.len()
}

fn find_coord(s: &str, ref_tile: Coord) -> Coord {
    let (mut x, mut y, mut z) = ref_tile;
    let mut dirs = s.chars();
    while let Some(a) = dirs.next() {
        let (dx, dy, dz) = if a == 'e' {
            DELTAS[0]
        } else if a == 'w' {
            DELTAS[1]
        } else {
            let b = dirs.next().expect("dir ends with n/s");
            match (a, b) {
                ('n', 'e') => DELTAS[2],
                ('n', 'w') => DELTAS[3],
                ('s', 'e') => DELTAS[4],
                ('s', 'w') => DELTAS[5],
                _ => unreachable!(),
            }
        };
        x += dx;
        y += dy;
        z += dz;
    }
    (x, y, z)
}
