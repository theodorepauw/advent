const INPUT: &str = include_str!("./inputs/16.txt");
use std::collections::HashMap;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let mut progs: Vec<usize> = (0..16).collect();
    fn dance(mut progs: Vec<usize>) -> Vec<usize> {
        for mv in INPUT.split(',') {
            match &mv[0..1] {
                "s" => progs.rotate_right(mv[1..].parse::<usize>().expect("spin parse err")),
                "x" => {
                    let pos: Vec<_> = mv[1..]
                        .splitn(2, '/')
                        .map(|s| s.parse::<usize>().expect("exchange parse err"))
                        .collect();
                    progs.swap(pos[0], pos[1]);
                }
                "p" => {
                    let pos: Vec<_> = mv[1..]
                        .splitn(2, '/')
                        .map(|s| {
                            progs
                                .iter()
                                .position(|&x| x == ((s.as_bytes()[0] - b'a') as usize))
                                .expect("prog not found")
                        })
                        .collect();
                    progs.swap(pos[0], pos[1]);
                }
                _ => panic!("unknown dance move"),
            }
        }
        progs
    }
    progs = dance(progs);
    let order = progs.clone();
    let to_letters = |order: Vec<usize>| -> String {
        let letters = order
            .into_iter()
            .map(|x| b'a' + x as u8)
            .collect::<Vec<_>>();
        String::from_utf8(letters).expect("couldn't convert [u8] to String")
    };

    let mut memory = HashMap::new();
    let mut index = 1;
    let p2 = loop {
        progs = dance(progs);
        if progs == order {
            break memory
                .remove(&((1_000_000_000usize - 1) % index))
                .expect("no sol p2");
        }
        memory.insert(index, progs.clone());
        index += 1;
    };
    let (p1, p2) = (to_letters(order), to_letters(p2));

    writeln!(io::stdout(), "Day 16 Part 1: {}\nDay 16 Part 2: {}", p1, p2)?;
    Ok(())
}
