const INPUT: &[u8] = include_bytes!("./inputs/06.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    fn pos_n_distinct<const N: u32>() -> usize {
        if !(0..26).contains(&N) {
            panic!("only up to 26 (a to z) distinct chars");
        }
        INPUT
            .windows(N as usize)
            .position(|b| {
                b.iter()
                    .fold(0u32, |set, item| set | (1 << (item - b'a') as u32))
                    .count_ones()
                    == N
            })
            .unwrap()
            + N as usize
    }
    let (p1, p2) = (pos_n_distinct::<4>(), pos_n_distinct::<14>());
    writeln!(
        io::stdout(),
        "Day 06 Part 1: {:?}\nDay 06 Part 2: {:?}",
        p1,
        p2
    )?;
    Ok(())
}
