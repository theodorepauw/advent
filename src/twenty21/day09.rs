const INPUT: &str = include_str!("./inputs/09.txt");
const DELTAS: &[(i32, i32)] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

use std::convert::TryFrom;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let width = INPUT.find('\n').unwrap() as i32;

    let mut heightmap: Vec<Option<u32>> = INPUT
        .lines()
        .flat_map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|h| if h == 9 { None } else { Some(h) })
        })
        .collect();
    fn get_index(i: usize, c: i32, r: i32, width: i32) -> Option<usize> {
        usize::try_from(i as i32 + c + r * width).ok()
    }

    fn basin_size(heightmap: &mut Vec<Option<u32>>, i: usize, width: i32) -> u32 {
        match heightmap.get_mut(i) {
            Some(height) if height.is_some() => {
                *height = None;
                1 + DELTAS
                    .iter()
                    .filter_map(|&(c, r)| {
                        if (i as i32) % width == 0 && c == -1
                            || ((i as i32) + 1) % width == 0 && c == 1
                        {
                            None
                        } else {
                            get_index(i, c, r, width)
                        }
                    })
                    .map(|j| basin_size(heightmap, j, width))
                    .sum::<u32>()
            }
            _ => 0,
        }
    }

    let (indices, low_points): (Vec<usize>, Vec<Option<u32>>) = heightmap
        .iter()
        .enumerate()
        .filter(|&(i, height)| {
            height.map_or(false, |h| {
                DELTAS
                    .iter()
                    .filter_map(|&(c, r)| get_index(i, c, r, width).and_then(|j| heightmap.get(j)))
                    .all(|neighbour_height| neighbour_height.map_or(true, |nh| h < nh))
            })
        })
        .unzip();

    let p1: u32 = low_points.iter().map(|height| height.unwrap() + 1).sum();

    let mut basins: Vec<u32> = indices
        .iter()
        .map(|&i| basin_size(&mut heightmap, i, width))
        .collect();
    basins.sort_unstable();
    let p2: u32 = basins.iter().rev().take(3).product();

    writeln!(io::stdout(), "Day 09 Part 1: {}\nDay 09 Part 2: {}", p1, p2,)?;
    Ok(())
}
