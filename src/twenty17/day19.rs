const INPUT: &str = include_str!("./inputs/19.txt");
use std::collections::HashMap;
use std::io::{self, Write};
const DX: &[i32; 4] = &[0, 1, 0, -1]; // Down -> Right -> Up -> Left
const DY: &[i32; 4] = &[1, 0, -1, 0]; // Down -> Right -> Up -> Left (y-axis is flipped)

pub fn solve() -> crate::util::Result<()> {
    let mut diagram: HashMap<(i32, i32), Item> = HashMap::new();
    let (mut packet_x, mut packet_y, mut d) =
        (INPUT.find('|').ok_or("start not found")? as i32, 0i32, 0);
    let (_, _) = INPUT.chars().fold((0, 0), |(mut x, mut y), c| {
        if c == '\n' {
            y += 1;
            x = 0;
        } else {
            if c != ' ' && c != '\r' {
                diagram.insert((x, y), Item::from(c));
            }
            x += 1;
        }
        (x, y)
    });

    let (mut p1, mut p2) = (String::new(), 0usize);
    while let Some(item) = diagram.get(&(packet_x, packet_y)) {
        match item {
            Item::Letter(c) => p1.push(*c),
            Item::Turn => {
                let (turn_left, turn_right) = ((d + 1) & 3, (d + 3) & 3);
                if diagram.contains_key(&(packet_x + DX[turn_left], packet_y + DY[turn_left])) {
                    d = turn_left;
                } else {
                    d = turn_right;
                }
            }
            _ => {}
        };
        packet_x += DX[d];
        packet_y += DY[d];
        p2 += 1;
    }

    writeln!(io::stdout(), "Day 19 Part 1: {}\nDay 19 Part 2: {}", p1, p2)?;
    Ok(())
}
enum Item {
    Path,
    Turn,
    Letter(char),
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        match c {
            '-' | '|' => Item::Path,
            '+' => Item::Turn,
            _ => Item::Letter(c),
        }
    }
}
