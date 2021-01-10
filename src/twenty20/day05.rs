use std::io::{self, Write};
const INPUT: &str = include_str!("./inputs/05.txt");

pub fn solve() -> crate::util::Result<()> {
    let mut seat_ids: Vec<u16> = INPUT
        .lines()
        .map(|s| {
            let calc = |s: &str| -> u16 {
                s.chars()
                    .map(|c| match c {
                        'F' | 'L' => 0,
                        'B' | 'R' => 1,
                        _ => panic!("invalid character!"),
                    })
                    .fold(0, |total, b| total << 1 | b)
            };
            let (row, col) = (calc(&s[0..7]), calc(&s[7..]));
            row << 3 | col
        })
        .collect();
    seat_ids.sort_unstable();

    let p1 = seat_ids.last().ok_or("no sol for p1")?;

    let p2 = (0..seat_ids.len() - 1)
        .map(|i| (seat_ids[i], seat_ids[i + 1]))
        .find_map(|(curr, next)| (next - curr > 1).then(|| curr + 1))
        .ok_or("no sol for p2")?;

    writeln!(io::stdout(), "Day 05 Part 1: {}\nDay 05 Part 2: {}", p1, p2)?;
    Ok(())
}
