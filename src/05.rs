use std::convert::TryFrom;
const INPUT: &str = include_str!("../inputs/05.txt");
fn main() {
    let mut seat_ids: Vec<u16> = INPUT.lines().map(|s| {
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
    }).collect();
    seat_ids.sort_unstable();

    let p1 = seat_ids.last().expect("no solution for p1");
    println!("Day 5 Part 1: {:?}", p1);
    let p2 = seat_ids
        .windows(2) // stoked to have found out about these!
        .flat_map(<&[u16; 2]>::try_from) // necessary to appease the compiler (so I can deref the slice)
        .find_map(|&[curr, next]| (next - curr > 1).then(|| curr + 1))
        .expect("no solution for p2");
    println!("Day 5 Part 2: {}", p2);
}