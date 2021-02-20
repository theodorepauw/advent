const INPUT: &str = include_str!("../inputs/05.txt");

fn main() {
    let memory: Vec<i64> = INPUT.split(',').map(|s| s.parse::<i64>().expect("not an int")).collect();
}

enum ParameterMode {
    Position,
    Immediate,
}

fn eval_parammode(n: i64) -> ParameterMode {
    use ParameterMode::*;
    match n {
        0 => Position,
        1 => Immediate,
    }
}

fn process(memory: &mut [usize]) -> i64 {
    let mut pos = 0;

    loop {
        let op_code = memory[pos]%100;
        let params = memory[pos]/100;
        match op_code {
            1 => memory[memory[pos+3]] = memory[memory[pos+1]]
        }
    }
    while memory[pos] != 99 {

    }
    memory[0]
}