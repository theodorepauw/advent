use std::convert::TryInto;
use std::io::{self, Write};

const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const INPUT: &str = include_str!("./inputs/11.txt");
const CUTOFF: u32 = 10_000;

lazy_static::lazy_static! {
    static ref ROWS: usize = INPUT.lines().count();
    static ref COLS: usize = INPUT.lines().next().unwrap().len();
}

// this version was mostly copied from fornwall

pub fn solve() -> crate::util::Result<()> {
    let mut data: Vec<u8> = INPUT.lines().flat_map(|s| s.bytes()).collect();
    let (p1, p2) = (run(&mut data.clone(), 4, true)?, run(&mut data, 5, false)?);
    writeln!(io::stdout(), "Day 11 Part 1: {}\nDay 11 Part 2: {}", p1, p2)?;

    Ok(())
}

fn run(data: &mut [u8], threshold: usize, is_part_one: bool) -> crate::util::Result<usize> {
    let (data_pos_to_seat_idx, seats_counter) =
        data.iter().enumerate().filter(|(_, &c)| c == b'L').fold(
            (vec![0; data.len()], 0),
            |(mut pos_to_seat, seats_counter), (i, _)| {
                pos_to_seat[i] = seats_counter;
                (pos_to_seat, seats_counter + 1)
            },
        );

    // An extra proxy at end for pointing to.
    let mut seats = vec![false; seats_counter + 1];

    let mut visibility_map = Vec::with_capacity(seats_counter);
    for (id, _) in data.iter().enumerate().filter(|(_, &c)| c != b'.') {
        let (x, y) = (id % *COLS, id / *COLS);

        let mut visibility_entry = [seats_counter; 8];
        let mut visibility_count = 0;
        for (dx, dy) in &DELTAS {
            for (x, y) in std::iter::successors(Some((x, y)), move |&(x, y)| {
                Some((
                    (x as isize + dx).try_into().ok()?,
                    (y as isize + dy).try_into().ok()?,
                ))
            })
            .skip(1)
            .take_while(move |&(x, y)| x < *COLS && y < *ROWS)
            {
                let visited_id = x + *COLS * y;
                if data[visited_id] == b'L' {
                    visibility_entry[visibility_count] = data_pos_to_seat_idx[visited_id];
                    visibility_count += 1;
                    break;
                }
                if is_part_one {
                    break;
                }
            }
        }

        visibility_map.push(visibility_entry);
    }

    let mut changes: Vec<usize> = Vec::with_capacity(seats_counter);
    let mut to_visit = (0..seats_counter).collect::<Vec<usize>>();
    let mut iteration = 0;
    loop {
        to_visit.retain(|&idx| {
            let seen_from_here_count = visibility_map[idx]
                .iter()
                .filter(|&&idx| seats[idx as usize])
                .count();

            // Free seat that is now taken or ccupied seat that is now left:
            ((!seats[idx] && seen_from_here_count == 0)
                || (seats[idx] && seen_from_here_count >= threshold))
                && {
                    changes.push(idx);
                    true
                }
        });

        changes.drain(0..).for_each(|id| seats[id] ^= true);

        if to_visit.is_empty() {
            return Ok(seats.iter().filter(|&&occupied| occupied).count());
        } else {
            iteration += 1;
            if iteration >= CUTOFF {
                return Err(format!("Aborting after {} iterations", iteration).into());
            }
        }
    }
}
