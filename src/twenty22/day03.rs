const INPUT: &[u8] = include_bytes!("./inputs/03.txt");
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let lines: Vec<&[u8]> = INPUT.split(|b| *b == b'\n').collect();
    let priority = |item: &u8| {
        (1 + if *item > (b'Z') {
            item - b'a'
        } else {
            item - b'A' + 26
        }) as u32
    };

    let p1: u32 = lines.iter().fold(0, |sum, line| {
        let (left, right) = line.split_at(line.len() >> 1);

        let common_item = left
            .iter()
            .find(|item| right.contains(item))
            .expect("no common item");

        sum + priority(common_item)
    });

    let p2: u32 = lines.chunks(3).fold(0, |sum, group| {
        let badge = group[0]
            .iter()
            .find(|item| group[1].contains(item) && group[2].contains(item))
            .expect("no badge");

        sum + priority(badge)
    });

    writeln!(
        io::stdout(),
        "Day 01 Part 1: {:?}\nDay 01 Part 2: {:?}",
        p1,
        p2
    )?;
    Ok(())
}
