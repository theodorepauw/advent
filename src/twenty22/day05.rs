const INPUT: &[u8] = include_bytes!("./inputs/05.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let usize_from_digitbytes =
        |b: &[u8]| b.iter().fold(0usize, |n, i| n * 10 + (i - b'0') as usize);
    let splitpoint = INPUT
        .windows(2)
        .position(|b| b[0] == b'\n' && b[1] == b'\n')
        .unwrap();
    let top_crates = |stacks: Vec<Vec<u8>>| {
        stacks
            .into_iter()
            .map(|stack| *stack.last().unwrap() as char)
            .collect::<String>()
    };

    let (starting_stacks, instructions) = INPUT.split_at(splitpoint);
    let mut starting_stacks = starting_stacks.rsplit(|&b| b == b'\n');
    let n_stacks: usize = starting_stacks
        .next()
        .unwrap()
        .rsplit(|&b| b == b' ')
        .skip_while(|s| s.is_empty())
        .map(usize_from_digitbytes)
        .next()
        .unwrap();

    let mut p1_stacks: Vec<Vec<u8>> = vec![Vec::with_capacity(10); n_stacks];
    for line in starting_stacks {
        for (i, supply_crate) in line
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, sc)| **sc != b' ')
        {
            p1_stacks[i].push(*supply_crate);
        }
    }
    let mut p2_stacks = p1_stacks.clone();

    for instr in instructions.split(|&b| b == b'\n').skip(2) {
        let codes: Vec<usize> = instr
            .split(|&b| b == b' ')
            .skip(1)
            .step_by(2)
            .map(usize_from_digitbytes)
            .collect();

        let (qty, origin, destination) = (codes[0], (codes[1] - 1), (codes[2] - 1));
        let split_index = p2_stacks[origin].len() - qty;

        let mut p2_move = p2_stacks[origin].split_off(split_index);
        p2_stacks[destination].append(&mut p2_move);

        for _ in 0..qty {
            let supply_crate = p1_stacks[origin].pop().unwrap();
            p1_stacks[destination].push(supply_crate);
        }
    }

    let (p1, p2) = (top_crates(p1_stacks), top_crates(p2_stacks));

    writeln!(
        io::stdout(),
        "Day 05 Part 1: {:?}\nDay 05 Part 2: {:?}",
        p1,
        p2
    )?;
    Ok(())
}
