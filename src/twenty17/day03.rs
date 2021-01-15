const INPUT: usize = 277678;
const DX: &[i32] = &[0, 1, 0, -1]; // South -> East -> North -> West
const DY: &[i32] = &[-1, 0, 1, 0]; // South -> East -> North -> West
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut n = 1;
    while n * n < INPUT {
        n += 1;
    }
    let p1 = n - (n * n - INPUT) % n - 1;

    let mut spiral = std::collections::HashMap::new();
    spiral.insert((0, 0), 1);
    let calc = |(x, y), spiral: &std::collections::HashMap<(i32, i32), usize>| -> usize {
        let mut sum = 0;
        for dx in -1..2 {
            for dy in -1..2 {
                sum += *spiral.get(&(x + dx, y + dy)).unwrap_or(&0);
            }
        }
        sum
    };
    let turn = |facing: usize| -> usize { (facing + 1) & 3 };

    let (mut x, mut y, mut facing) = (0, 0, 0);
    let mut p2 = 0;
    while p2 < INPUT {
        let next_dir = turn(facing);
        if spiral.get(&(x + DX[next_dir], y + DY[next_dir])).is_none() {
            facing = next_dir;
        }
        x += DX[facing];
        y += DY[facing];
        p2 = calc((x, y), &spiral);
        spiral.insert((x, y), p2);
    }
    writeln!(io::stdout(), "Day 03 Part 1: {}\nDay 03 Part 2: {}", p1, p2)?;
    Ok(())
}
