const INPUT: &str = include_str!("./inputs/21.txt");
const START: &str = ".#...####";
const SEP: &str = " => ";
use crate::util::{self, SquareVec};
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> util::Result<()> {
    // precompute all possible orientations for rule
    let enhancement_rules: HashMap<Vec<char>, Vec<char>> = INPUT
        .lines()
        .map(|s| s.replace('/', ""))
        .map(|s| {
            let x = s.find(SEP).expect("invalid rule fmt");
            (
                s[..x].chars().collect::<Vec<char>>(),
                s[x + SEP.len()..].chars().collect::<Vec<char>>(),
            )
        })
        .flat_map(|(k, v)| {
            (0..12)
                .scan(k, |orientation, i| {
                    if i == 4 {
                        orientation.flip_horizontal();
                    } else if i == 8 {
                        orientation.flip_vertical();
                    } else if i != 0 {
                        orientation.rotate_left();
                    }
                    Some(orientation.clone())
                })
                .zip(std::iter::repeat(v))
        })
        .collect();

    let mut fractal_art: Vec<char> = START.chars().collect();
    let mut enhanced = vec![];
    for x in 0..18 {
        let len = fractal_art.len();
        let size = util::sqrt(len);
        let step = 2 + (size & 1);
        let row_stride = step * size;
        enhanced.clear();
        for r in (0..=len - row_stride).step_by(row_stride) {
            for c in (0..size).step_by(step) {
                let mut sq = Vec::with_capacity(row_stride);
                for x in (r..r + row_stride).step_by(size) {
                    sq.extend_from_slice(&fractal_art[c + x..c + x + step]);
                }
                enhanced.extend(enhancement_rules.get(&sq));
            }
        }
        fractal_art = stitch(&enhanced, step);
        if x == 4 {
            writeln!(
                io::stdout(),
                "Day 21 Part 1: {}",
                fractal_art.iter().filter(|&&c| c == '#').count()
            )?;
        }
    }
    let p2 = fractal_art.into_iter().filter(|&c| c == '#').count();
    writeln!(io::stdout(), "Day 21 Part 2: {}", p2,)?;
    Ok(())
}

fn stitch(v: &[&Vec<char>], step: usize) -> Vec<char> {
    let (size, step) = (util::sqrt(v.len()) + 1, step + 1);
    let mut big_picture = Vec::with_capacity(size * size);
    for ch in v.chunks(size - 1) {
        for r in 0..step {
            for t in ch {
                big_picture.extend(t[r * step..].iter().take(step));
            }
        }
    }
    big_picture
}
