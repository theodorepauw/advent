const INPUT: &str = include_str!("./inputs/15.txt");
use std::io::{self, Write};

const P1_END: usize = 2021;
const P2_END: usize = 30000001;

pub fn solve() -> crate::util::Result<()> {
    let calc_num = |spoken: Option<usize>, curr_turn: usize| -> (usize, usize) {
        (
            match spoken {
                Some(prev_turn) => curr_turn - prev_turn - 1,
                None => 0,
            },
            curr_turn,
        )
    };

    let mut record = vec![None; P2_END]; // can't use array becuase of stack overflow. could use the nightly box syntax, but meh
    let mut speak =
        |(n, i)| -> (Option<usize>, usize) { (std::mem::replace(&mut record[n], Some(i)), n) };

    let ((spoken_before, last_num), cursor) = INPUT
        .split(',')
        .map(|s| s.parse::<usize>().expect("parsing err"))
        .fold(((None, 0), 1), |((_, _), cursor), number| {
            (speak((number, cursor)), cursor + 1)
        });

    let (p1, (_, p2)) = (cursor..P2_END).fold(
        (None, (spoken_before, last_num)),
        |(mut p1, (spoken_before, last_num)), cursor| {
            if cursor == P1_END {
                p1 = Some(last_num);
            }
            (p1, speak(calc_num(spoken_before, cursor)))
        },
    );

    writeln!(
        io::stdout(),
        "Day 15 Part 1: {:?}\nDay 15 Part 2: {}",
        p1.ok_or("no sol day 15 p1")?,
        p2
    )?;
    Ok(())
}
