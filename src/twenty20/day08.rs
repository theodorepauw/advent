const INPUT: &str = include_str!("./inputs/08.txt");
use crate::util::*;
use std::io::{self, Write};
use std::str::FromStr;

enum Operation {
    Accumulate(isize),
    Jump(isize),
    Nop(isize), // Valid nops always shift the cursor by +1, but the args are kept because a jump could be disguised as a nop
}
struct State {
    cursor: usize,
    accumulation: isize,
}

pub fn solve() -> Result<()> {
    let mut state = State::new();
    let mut instructions: Vec<(Operation, bool, bool)> = INPUT
        .lines()
        .map(|s| s.parse::<Operation>().map(|op| (op, false, false)))
        .collect::<Result<_>>()?;
    let mut part_2: Option<isize> = None;

    while !instructions[state.cursor].1 {
        instructions[state.cursor].1 = true; // seen in main loop
        let current_op = &instructions[state.cursor].0;
        let mut altered_state = state.apply_operation(&current_op.convert());

        while part_2.is_none()
            && !instructions[altered_state.cursor].1
            && !instructions[altered_state.cursor].2
        {
            instructions[altered_state.cursor].2 = true; // seen in second loop (separated to preserve the main loop's integrity)

            altered_state = altered_state.take_next(&instructions);
            part_2 = altered_state.terminates(&instructions);
        }

        state = state.take_next(&instructions);
    }
    let (p1, p2) = (state.accumulation, part_2.ok_or("no sol for p2")?);
    writeln!(io::stdout(), "Day 08 Part 1: {}\nDay 08 Part 2: {}", p1, p2)?;
    Ok(())
}

impl FromStr for Operation {
    type Err = Error;
    fn from_str(instruction: &str) -> Result<Self> {
        let mut s = instruction.split_whitespace();
        let operation = s.next().ok_or("no operation found")?;
        let argument = s
            .next()
            .ok_or("no arg found")
            .map(|a| a.parse::<isize>())??;
        Ok(match operation {
            "acc" => Operation::Accumulate(argument),
            "jmp" => Operation::Jump(argument),
            "nop" => Operation::Nop(argument),
            _ => return Err("Operation not recognised!".into()),
        })
    }
}

impl Operation {
    fn convert(&self) -> Self {
        match self {
            Operation::Accumulate(arg) => Operation::Accumulate(*arg),
            Operation::Jump(arg) => Operation::Nop(*arg),
            Operation::Nop(arg) => Operation::Jump(*arg),
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            cursor: 0,
            accumulation: 0,
        }
    }

    fn apply_operation(&self, op: &Operation) -> Self {
        let (mut c, mut a) = (self.cursor, self.accumulation);
        match op {
            Operation::Accumulate(arg) => {
                c += 1;
                a += arg;
            }
            Operation::Jump(arg) => {
                c = std::convert::TryInto::try_into(c as isize + arg).expect("Cursor negative!");
            }
            Operation::Nop(_) => c += 1,
        }

        State {
            cursor: c,
            accumulation: a,
        }
    }

    fn take_next(&self, instructions: &[(Operation, bool, bool)]) -> Self {
        let op = &instructions[self.cursor].0;
        self.apply_operation(op)
    }

    fn terminates(&self, instructions: &[(Operation, bool, bool)]) -> Option<isize> {
        if self.cursor >= instructions.len() {
            Some(self.accumulation)
        } else {
            None
        }
    }
}
