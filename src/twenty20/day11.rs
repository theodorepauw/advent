use std::convert::TryInto;
use std::io::{self, Write};
use std::iter::successors;

// copied from https://github.com/zookini/aoc-2020/blob/master/src/bin/11.rs
// old version: 8289b31

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

type Direction = fn(&Grid, (usize, usize), (isize, isize)) -> Option<(usize, usize)>;
type Grid = Vec<Vec<u8>>;

pub fn solve() -> crate::util::Result<()> {
    let start = std::time::Instant::now();
    let grid: Grid = include_str!("./inputs/11.txt")
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    writeln!(
        io::stdout(),
        "Part 1: {}",
        seated(grid.clone(), 4, |grid, s, n| steps(&grid, s, n).next())
    )?;
    writeln!(
        io::stdout(),
        "Part 2: {}",
        seated(grid, 5, |grid, s, n| steps(&grid, s, n)
            .find(|&(x, y)| grid[y][x] != b'.'))
    )?;
    writeln!(io::stdout(), "\n{} μs", start.elapsed().as_micros())?;
    Ok(())
}

fn seated(start: Grid, threshold: usize, direction: Direction) -> usize {
    successors(Some(start), |grid| {
        round(&grid, threshold, direction).filter(|next| next != grid)
    })
    .last()
    .unwrap()
    .iter()
    .flat_map(|row| row.iter().filter(|&&b| b == b'#'))
    .count()
}

fn round(grid: &Grid, threshold: usize, direction: Direction) -> Option<Grid> {
    Some(
        grid.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, &b)| {
                        let neighbours = DELTAS
                            .iter()
                            .filter_map(|&step| {
                                direction(grid, (x, y), step).filter(|&(x, y)| grid[y][x] == b'#')
                            })
                            .count();

                        match b {
                            b'L' if neighbours == 0 => b'#',
                            b'#' if neighbours >= threshold => b'L',
                            _ => b,
                        }
                    })
                    .collect()
            })
            .collect(),
    )
}

fn steps(
    grid: &Grid,
    start: (usize, usize),
    step: (isize, isize),
) -> impl Iterator<Item = (usize, usize)> {
    let size = (grid[0].len(), grid.len());

    successors(Some(start), move |&(x, y)| {
        Some((
            (x as isize + step.0).try_into().ok()?,
            (y as isize + step.1).try_into().ok()?,
        ))
    })
    .skip(1)
    .take_while(move |&pos| pos.0 < size.0 && pos.1 < size.1)
}
