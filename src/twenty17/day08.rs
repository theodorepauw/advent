const INPUT: &str = include_str!("./inputs/08.txt");
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut p2 = 0;
    for s in INPUT.lines() {
        let s: Vec<_> = s.split_whitespace().collect();
        let (&x, op, val) = (
            registers.get(s[4]).unwrap_or(&0),
            s[5],
            s[6].parse::<i32>().expect("cnd no num"),
        );
        let reg = registers.entry(s[0].to_owned()).or_default();
        let amt = s[2].parse::<i32>()? * if s[1] == "dec" { -1 } else { 1 };
        if match op {
            "==" => x == val,
            "!=" => x != val,
            "<=" => x <= val,
            ">=" => x >= val,
            ">" => x > val,
            "<" => x < val,
            _ => panic!("unrecognised condition"),
        } {
            *reg += amt;
            p2 = p2.max(*reg);
        }
    }
    let p1 = registers.values().max().expect("no sol p1");
    writeln!(io::stdout(), "Day 08 Part 1: {}\nDay 08 Part 2: {}", p1, p2)?;
    Ok(())
}
