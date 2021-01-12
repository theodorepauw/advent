const INPUT: usize = include!("./inputs/23.txt"); // -> 784235916
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let digits: Vec<usize> = crate::util::digits(INPUT).collect();
    writeln!(io::stdout(), "Day 23 Part 1: {}", part_1(&digits))?;
    writeln!(io::stdout(), "Day 23 Part 2: {}", part_2(&digits))?;
    Ok(())
}

fn play_cups(cups: &[usize], moves: usize) -> Vec<usize> {
    let min = *cups.iter().min().expect("no min");
    let max = *cups.iter().max().expect("no max");
    let mut links = vec![0; cups.len() + 1]; // +1 to avoid adapting to zero-index later
    for i in 0..cups.len() {
        links[cups[i]] = cups[(i + 1) % cups.len()];
    }

    let mut curr_cup = cups[0];
    for _ in 0..moves {
        let pick_1 = links[curr_cup];
        let pick_2 = links[pick_1];
        let pick_3 = links[pick_2];
        links[curr_cup] = links[pick_3]; //fix circle

        let mut dest_cup = if curr_cup > min { curr_cup - 1 } else { max };
        while [pick_1, pick_2, pick_3].contains(&dest_cup) || dest_cup < min || dest_cup > max {
            dest_cup = if dest_cup > min { dest_cup - 1 } else { max };
        }

        let temp = links[dest_cup];
        links[dest_cup] = pick_1;
        links[pick_1] = pick_2;
        links[pick_2] = pick_3;
        links[pick_3] = temp;
        curr_cup = links[curr_cup];
    }
    links
}

fn part_1(cups: &[usize]) -> usize {
    let links = play_cups(cups, 100);
    let (mut i, mut res) = (1, 0);
    while links[i] != 1 {
        i = links[i];
        res = (res * 10) + i;
    }
    res
}

fn part_2(cups: &[usize]) -> usize {
    let max = *cups.iter().max().expect("no max");
    let links = play_cups(
        &(cups
            .iter()
            .copied()
            .chain(max + 1..1_000_001usize)
            .collect::<Vec<_>>()),
        10_000_000,
    );
    let cup_2 = links[1];
    let cup_3 = links[cup_2];
    cup_2 * cup_3
}
