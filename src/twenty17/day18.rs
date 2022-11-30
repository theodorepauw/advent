const INPUT: &str = include_str!("./inputs/18.txt");
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::mpsc::{channel, Receiver, Sender};

pub fn solve() -> crate::util::Result<()> {
    let instr: Vec<Vec<_>> = INPUT
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();
    let (mut cursor, mut prog) = (0, Program::with_id(0));
    let p1 = loop {
        let i = &instr[cursor];
        if i[0] == "jgz" {
            if prog.get_val(i[1]) > 0 {
                cursor = (cursor as i64 + prog.get_val(i[2])) as usize;
            } else {
                cursor += 1;
            }
        } else {
            if i[0] == "snd" {
                prog.play(i[1]);
            } else if i[0] == "rcv" {
                if let Some(freq) = prog.rcv(i[1]) {
                    break freq;
                }
            } else {
                prog.adjust(i[0], i[1], i[2]);
            }
            cursor += 1;
        }
    };
    writeln!(io::stdout(), "Day 18 Part 1: {}", p1,)?;

    let (mut prog_a, mut prog_b) = DuetProgram::new_duet(0, 1);
    let p2 = loop {
        if prog_a.run_instr(&instr[prog_a.cursor]) && prog_b.run_instr(&instr[prog_b.cursor]) {
            break prog_b.sent_count;
        }
    };
    writeln!(io::stdout(), "Day 18 Part 2: {}", p2,)?;
    Ok(())
}

struct DuetProgram {
    prog: Program,
    transmission: Sender<i64>,
    reception: Receiver<i64>,
    sent_count: usize,
    cursor: usize,
}

impl DuetProgram {
    fn run_instr(&mut self, i: &[&str]) -> bool {
        if i[0] == "jgz" {
            if self.prog.get_val(i[1]) > 0 {
                self.cursor = (self.cursor as i64 + self.prog.get_val(i[2])) as usize;
            } else {
                self.cursor += 1;
            }
        } else {
            if i[0] == "snd" {
                self.send(i[1]);
            } else if i[0] == "rcv" {
                match self.reception.try_recv() {
                    Ok(val) => self.prog.set(grab_char(i[1]), val),
                    Err(_) => return true,
                }
            } else {
                self.prog.adjust(i[0], i[1], i[2]);
            }
            self.cursor += 1;
        }
        false
    }

    fn new_duet(id_a: i64, id_b: i64) -> (Self, Self) {
        let (prog_a, prog_b) = (DuetProgram::new(id_a), DuetProgram::new(id_b));
        prog_a.connect(prog_b)
    }

    fn new(id: i64) -> Self {
        let prog = Program::with_id(id);
        let (transmission, reception) = channel();
        DuetProgram {
            prog,
            transmission,
            reception,
            sent_count: 0,
            cursor: 0,
        }
    }

    fn send(&mut self, s: &str) {
        self.transmission
            .send(self.prog.get_val(s))
            .expect("send on closed channel");
        self.sent_count += 1;
    }

    fn connect(self, other: DuetProgram) -> (Self, Self) {
        (
            DuetProgram {
                reception: other.reception,
                ..self
            },
            DuetProgram {
                reception: self.reception,
                ..other
            },
        )
    }
}

struct Program {
    register: HashMap<char, i64>,
    last_played: Option<i64>,
}

impl Program {
    fn with_id(id: i64) -> Self {
        let mut register = HashMap::new();
        register.insert('p', id);
        Program {
            register,
            last_played: None,
        }
    }

    fn adjust(&mut self, cmd: &str, reg: &str, val: &str) {
        let (reg, val) = (grab_char(reg), self.get_val(val));
        match cmd {
            "set" => self.set(reg, val),
            "add" => self.add(reg, val),
            "mul" => self.mul(reg, val),
            "mod" => self.rem(reg, val),
            _ => panic!("{} is invalid", cmd),
        }
    }

    fn set(&mut self, reg: char, val: i64) {
        self.register.insert(reg, val);
    }

    fn add(&mut self, reg: char, val: i64) {
        *self.register.entry(reg).or_default() += val;
    }

    fn mul(&mut self, reg: char, val: i64) {
        *self.register.entry(reg).or_default() *= val;
    }

    fn rem(&mut self, reg: char, val: i64) {
        *self.register.entry(reg).or_default() %= val;
    }

    fn get_val(&mut self, s: &str) -> i64 {
        match s.parse::<i64>() {
            Ok(v) => v,
            Err(_) => *self.register.entry(grab_char(s)).or_default(),
        }
    }

    fn play(&mut self, s: &str) {
        self.last_played = Some(self.get_val(s));
    }

    fn rcv(&mut self, reg: &str) -> Option<i64> {
        match self.get_val(reg) {
            0 => None,
            _ => self.last_played,
        }
    }
}

fn grab_char(s: &str) -> char {
    s.chars().next().expect("no chars in str")
}
