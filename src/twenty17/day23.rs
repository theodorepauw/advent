const INPUT: &str = include_str!("./inputs/23.txt");
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let instr: Vec<Vec<_>> = INPUT
        .lines()
        .map(|s| s.split_whitespace().take(3).collect())
        .collect();

    let mut prog_p1 = Program::with_switch('p', 0);
    prog_p1.run(&instr);
    let p1 = prog_p1.muls;

    let mut p2 = 0; // p2 == h
    for x in (107900..=124900).step_by(17) {
        let mut n = 2;
        while n * n <= x {
            if x % n == 0 {
                p2 += 1;
                break;
            }
            n += 1;
        }
    }

    writeln!(io::stdout(), "Day 23 Part 1: {}\nDay 23 Part 2: {}", p1, p2)?;
    Ok(())
}

struct Program {
    register: HashMap<char, i64>,
    muls: usize,
}

impl Program {
    fn run(&mut self, instr: &[Vec<&str>]) {
        let mut cursor = 0;
        while cursor < instr.len() {
            let i = &instr[cursor];
            if i[0] == "jnz" {
                if self.get_val(i[1]) != 0 {
                    cursor = (cursor as i64 + self.get_val(i[2])) as usize;
                } else {
                    cursor += 1;
                }
            } else {
                self.adjust(i[0], i[1], i[2]);
                cursor += 1;
            }
        }
    }

    fn with_switch(reg: char, val: i64) -> Self {
        let mut register = HashMap::new();
        register.insert(reg, val);
        Program { register, muls: 0 }
    }

    fn adjust(&mut self, cmd: &str, reg: &str, val: &str) {
        let (reg, val) = (grab_char(reg), self.get_val(val));
        match cmd {
            "set" => self.set(reg, val),
            "sub" => self.sub(reg, val),
            "mul" => self.mul(reg, val),
            _ => panic!(format!("{} is invalid", cmd)),
        }
    }

    fn set(&mut self, reg: char, val: i64) {
        self.register.insert(reg, val);
    }

    fn sub(&mut self, reg: char, val: i64) {
        *self.register.entry(reg).or_default() -= val;
    }

    fn mul(&mut self, reg: char, val: i64) {
        *self.register.entry(reg).or_default() *= val;
        self.muls += 1;
    }

    fn get_val(&mut self, s: &str) -> i64 {
        match s.parse::<i64>() {
            Ok(v) => v,
            Err(_) => *self.register.entry(grab_char(s)).or_default(),
        }
    }
}

fn grab_char(s: &str) -> char {
    s.chars().next().expect("no chars in str")
}
