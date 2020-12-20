use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/17.txt");

fn main() {
    let mut deltas: Vec<(i32, i32, i32, i32)> = Vec::with_capacity(80);
    for w in -1..2 {
        for z in -1..2 {
            for y in -1..2 {
                for x in -1..2 {
                    match (x, y, z, w) {
                        (0, 0, 0, 0) => {}
                        _ => deltas.push((x, y, z, w)),
                    }
                }
            }
        }    
    }
    
    let mut pocket: HashMap<(i32, i32, i32, i32), (bool, usize)> = HashMap::new();
    let (_, _) = INPUT.chars().fold((0, 0), |(x, y), c| {
        if c == '\n' {
            (0, y + 1)
        } else {
            pocket.insert((x, y, 0, 0), (c == '#', 0));
            (x + 1, y)
        }
    });

    for _ in 0..6 {
        let coords: Vec<(i32, i32, i32, i32)> = pocket.keys().cloned().collect();
        for (x, y, z, w) in coords {
            if pocket.get(&(x, y, z, w)).map_or(false, |v| v.0) {
                for (dx, dy, dz, dw) in deltas.iter() {
                    let coord = (x + dx, y + dy, z + dz, w+dw);
                    let (_, count) = pocket.entry(coord).or_insert((false, 0));
                    *count += 1;
                }
            }
        }

        for (_, (is_active, active_neighbours)) in pocket.iter_mut() {
            if (*is_active && !(*active_neighbours == 2 || *active_neighbours == 3))
                || (!*is_active && *active_neighbours == 3)
            {
                *is_active = !*is_active;
            }
            *active_neighbours = 0;
        }
    }
    

    let p1 = pocket.values().filter(|&&b| b.0).count();
    println!("Day 17 Part 2: {}", p1);
}
