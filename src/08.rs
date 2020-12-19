use std::convert::TryInto;
const INPUT: &str = include_str!("../inputs/08.txt");
enum Operation {
    Accumulate(isize),
    Jump(isize),
    Nop(isize), // Valid nops always shift the cursor by +1, but the args must be retained because of the corruption (a jump could be disguised as a nop)
}
struct State {
    cursor: usize,
    accumulation: isize,
}

fn main() {
    let mut state = State::new();
    let mut instructions: Vec<(Operation, bool, bool)> = INPUT.lines().map(|s| (Operation::from(s), false, false)).collect();
    let mut part_2: Option<isize> = None;

    while !instructions[state.cursor].1 {
        instructions[state.cursor].1 = true; // seen in main loop
        let current_op = &instructions[state.cursor].0;
        let mut altered_state = state.apply_operation(&current_op.convert());

        while part_2.is_none() && !instructions[altered_state.cursor].1 && !instructions[altered_state.cursor].2 {
            instructions[altered_state.cursor].2 = true; // seen in second loop (separated to preserve the main loop's integrity)

            altered_state = altered_state.take_next(&instructions);
            part_2 = altered_state.terminates(&instructions);
        }

        state = state.take_next(&instructions);
    }

    println!("Day 8 Part 1: {}", state.accumulation);
    println!("Day 8 Part 2: {}", part_2.expect("Part 2 not found"));
}

impl Operation {
    fn from(instruction: &str) -> Self {
        let mut s = instruction.split_whitespace();
        let operation = s.next().expect("No operation found!");
        let argument = s.next().expect("No argument found!").parse::<isize>().expect("Couldn't parse argument as isize!");

        match operation {
            "acc" => Operation::Accumulate(argument),
            "jmp" => Operation::Jump(argument),
            "nop" => Operation::Nop(argument),
            _ => panic!("Operation not recognised!"),
        }
    }

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
        State { cursor: 0, accumulation: 0 }
    }

    fn apply_operation(&self, op: &Operation) -> Self {
        let (mut c, mut a) = (self.cursor, self.accumulation);
        match op {
            Operation::Accumulate(arg) => { c += 1; a += arg; },
            Operation::Jump(arg) => { c = (c as isize + arg).try_into().expect("Cursor negative!"); },
            Operation::Nop(_) => c += 1,
        }

        State {cursor: c, accumulation: a }
    }

    fn take_next(&self, instructions: &[(Operation, bool, bool)]) -> Self {
        let op = &instructions[self.cursor].0;
        self.apply_operation(op)
    }

    fn terminates(&self, instructions: &[(Operation, bool, bool)]) -> Option<isize> {
        if self.cursor > instructions.len()-1 {
            Some(self.accumulation)
        } else {
            None
        }
    }
}