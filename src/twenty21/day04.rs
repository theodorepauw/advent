const INPUT: &str = include_str!("./inputs/04.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {

    let str_to_int = |s| usize::from_str_radix(s, 10).expect("err: str to usize");
    let mut paragraphs = INPUT.split_terminator("\n\n");
    let order = paragraphs.next().expect("no  numbers to draw").split(',').map(str_to_int);

    let mut boards: Vec<(Vec<(usize, bool)>, bool)> = paragraphs.map(|s| s.lines().flat_map(|s| s.split_whitespace().map(|s| (str_to_int(s), false))).collect()).map(|v| (v, false)).collect();

    let mut p1: Option<usize> = None;
    let mut p2 = 0;
    
    for x in order {
        for (b, bingo)  in boards.iter_mut() {    
            for i in 0..b.len() {
                let (n, marked) = b.get_mut(i).ok_or("key OOB")?;
                if n==&x {
                    *marked = true;
                    let row_start = i/5 * 5;

                    if (row_start..row_start+5).all(|j| b[j].1) || (1..5).all(|j| b[(i+j*5)%25].1) {
                        *bingo = true;
                        p2 = x * b.iter().filter(|&(_, marked)| !marked).map(|(n, _)| *n).sum::<usize>();
                        if p1.is_none() {
                            p1 = Some(p2);
                        }
                    }
                }
            }
        }
        boards.retain(|(_, bingo)| !bingo);
    };

    writeln!(io::stdout(), "Day 04 Part 1: {}\nDay 04 Part 2: {}", p1.expect("p1 not found"), p2)?;
    Ok(())
}