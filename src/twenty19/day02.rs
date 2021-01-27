const INPUT: &str = include_str!("./inputs/02.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let memory: Vec<_> = INPUT
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let p1 = part1(&mut memory.clone());
    let p2 = part2(memory);
    writeln!(io::stdout(), "Day 02 Part 1: {}\nDay 02 Part 2: {}", p1, p2)?;
    Ok(())
}

fn part1(memory: &mut [usize]) -> usize {
    process(12, 2, memory)
}

fn process(noun: usize, verb: usize, memory: &mut [usize]) -> usize {
    let mut pos = 0;
    memory[1] = noun;
    memory[2] = verb;
    while memory[pos] != 99 {
        let (a, b, dest) = (
            memory[memory[pos + 1]],
            memory[memory[pos + 2]],
            memory[pos + 3],
        );
        memory[dest] = match memory[pos] {
            1 => a + b,
            2 => a * b,
            _ => panic!("invalid opcode"),
        };
        pos += 4;
    }
    memory[0]
}

fn part2(memory: Vec<usize>) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut fresh_memory = memory.clone();
            if process(noun, verb, &mut fresh_memory) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
