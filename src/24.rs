const INPUT: &str = include_str!("../inputs/24.txt");
const CENTRE_TILE: (i32, i32, i32) = (0, 0, 0);
const DIRS: [&str; 6] = ["e", "w", "se", "sw", "ne", "nw"];
use std::collections::HashMap;

fn main() {
    let mut tiles: HashMap<(i32, i32, i32), bool> = HashMap::new(); // default is white
    let p1 = part1(&mut tiles);
    println!("Day 24 Part 1: {}", p1);
    let p2 = part2(&mut tiles);
    println!("Day 24 Part 2: {}", p2);
}

fn part1(tiles: &mut HashMap<(i32, i32, i32), bool>) -> usize {
    for line in INPUT.lines() {
        let loc = find_coord(line, CENTRE_TILE);
        *tiles.entry(loc).or_default() ^= true;
    }
    count_black(&tiles)
}

fn part2(tiles: &mut HashMap<(i32, i32, i32), bool>) -> usize {
    for _ in 0..100 {
        let coords: Vec<(i32, i32, i32)> = tiles.keys().cloned().collect();
        for c in coords.iter() {
            for d in DIRS.iter() {
                tiles.entry(find_coord(d, *c)).or_default();
            }
        }
        let coords: Vec<(i32, i32, i32)> = tiles.keys().cloned().collect();
        let mut flips = vec![];
        for coord in coords {
            let mut adjacent_black: u8 = 0;
            let is_black = *tiles.get(&coord).expect("coord not stored");
            for d in DIRS.iter() {
                if *tiles.entry(find_coord(d, coord)).or_default() {
                    adjacent_black += 1;
                }
            }
            if adjacent_black == 2 {
                if !is_black {
                    flips.push(coord);
                }
            } else if is_black && adjacent_black != 1 {
                flips.push(coord);
            }
        }

        for coord in flips.into_iter() {
            *tiles.get_mut(&coord).expect("tile to flip not stored") ^= true;
        }
    }
    count_black(tiles)
}

fn find_coord(s: &str, ref_tile: (i32, i32, i32)) -> (i32, i32, i32) {
    let (mut x, mut y, mut z) = ref_tile;
    let mut dirs = s.chars().peekable();

    while dirs.peek().is_some() {
        let d = dirs.next().unwrap();
        match d {
            'e' => {
                x += 1;
                z -= 1;
            }
            'w' => {
                x -= 1;
                z += 1;
            }
            'n' | 's' => match (d, dirs.next().unwrap()) {
                ('n', 'e') => {
                    y += 1;
                    z -= 1;
                }
                ('s', 'w') => {
                    y -= 1;
                    z += 1;
                }
                ('n', 'w') => {
                    x -= 1;
                    y += 1;
                }
                ('s', 'e') => {
                    x += 1;
                    y -= 1;
                }
                _ => panic!("unrecognised instruction"),
            },
            _ => panic!("unrecognised instruction"),
        }
    }
    (x, y, z)
}

fn count_black(tiles: &HashMap<(i32, i32, i32), bool>) -> usize {
    tiles.values().filter(|&&b| b).count()
}
