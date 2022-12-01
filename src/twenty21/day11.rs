const INPUT: &str = include_str!("./inputs/11.txt");
use std::collections::VecDeque;
use std::io::{self, Write};

const YX_STEPS: &[(i8, i8)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solve() -> crate::util::Result<()> {
    let mut rows: Vec<Vec<Option<u8>>> = INPUT
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).map(|x| x as u8)).collect())
        .collect();

    let mut flash_queue = VecDeque::new();
    let mut flashes = 0usize;
    let mut p1 = 0;
    let mut i = 0;

    let p2 = 'allflash: loop {
        i += 1;
        if i == 101 {
            p1 = flashes;
        }

        for (y, row) in rows.iter_mut().enumerate() {
            for (x, octopus) in row.iter_mut().enumerate() {
                *octopus = match *octopus {
                    Some(9) => {
                        flash_queue.push_back((y, x));
                        None
                    }
                    Some(energy) => Some(energy + 1),
                    _ => Some(1),
                };
            }
        }

        while let Some((y, x)) = flash_queue.pop_front() {
            flashes += 1;
            for (dy, dx) in YX_STEPS.iter() {
                let (adj_y, adj_x) = (y as i8 + dy, x as i8 + dx);
                if (0..10).contains(&adj_y) && (0..10).contains(&adj_x) {
                    let (adj_y, adj_x) = (adj_y as usize, adj_x as usize);
                    let octopus = &mut rows[adj_y][adj_x];
                    if let Some(energy) = *octopus {
                        *octopus = if energy == 9u8 {
                            flash_queue.push_back((adj_y, adj_x));
                            None
                        } else {
                            Some(energy + 1)
                        };
                    }
                }
            }
        }

        if rows.iter().all(|r| r.iter().all(|f| f.is_none())) {
            break 'allflash i;
        }
    };

    writeln!(io::stdout(), "Day 11 Part 1: {}\nDay 11 Part 2: {}", p1, p2)?;
    Ok(())
}
