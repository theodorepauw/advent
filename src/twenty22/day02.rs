const INPUT: &[u8] = include_bytes!("./inputs/02.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    // writeln!(io::stdout(), "{:?}", INPUT)?;
    let hands: Vec<u8> = INPUT.iter().step_by(2).flat_map(parse).collect();
    let (p1, p2) = hands.chunks(2).fold((0, 0), |(p1, p2), hands| {
        (
            p1 + hands[1] as u32 + play(hands[0], hands[1]),
            p2 + score_per_strat(hands[0], hands[1]) as u32,
        )
    });

    writeln!(
        io::stdout(),
        "Day 01 Part 1: {:?}\nDay 01 Part 2: {:?}",
        p1,
        p2
    )?;
    Ok(())
}

fn play(opp_hand: u8, my_hand: u8) -> u32 {
    if my_hand == opp_hand {
        3
    } else if my_hand == opp_hand % 3 + 1 {
        6
        // e.g. Rock[1] wins Scissors[3] -> 3%3 = 0 + 1 = 1
        // e.g. Paper[2] wins Rock[1] -> 1%3 = 1 + 1 = 2
        // e.g. Scissors[3] wins Paper[2] -> 2%3 = 2 + 1 = 3
    } else {
        0
    }
}

fn score_per_strat(opp_hand: u8, outcome: u8) -> u8 {
    if outcome == 1 {
        (opp_hand + 1) % 3 + 1
    } else if outcome == 2 {
        3 + opp_hand
    } else if outcome == 3 {
        6 + (opp_hand % 3 + 1)
    } else {
        panic!("outcome not in 1..3!")
    }
}

fn parse(b: &u8) -> Result<u8, &'static str> {
    match b {
        b'A' | b'X' => Ok(1),
        b'B' | b'Y' => Ok(2),
        b'C' | b'Z' => Ok(3),
        _ => Err("hand not abc/xyz!"),
    }
}
