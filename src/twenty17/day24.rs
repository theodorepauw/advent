const INPUT: &str = include_str!("./inputs/24.txt");
use std::io::{self, Write};
use crate::util;

// My original (slow) version.

pub fn solve() -> util::Result<()> {
    let ports: Vec<Port> = INPUT
        .lines()
        .map(|s| Port::from_str_tuple(s.split_at(s.find('/').expect("no / sep"))))
        .collect();

    let (tx, rx) = std::sync::mpsc::channel::<Bridge>();
    std::thread::spawn(move || {
        Bridge { ports }.build(0, 0, tx);
    });

    let (p1, (_, p2)) = rx.iter().fold(
        (0, (0, 0)),
        |(overall_max, (mut max_len, mut max_for_len)), b| {
            let (len, strength) = (b.len(), b.strength());
            if len > max_len {
                max_len = len;
                max_for_len = strength;
            } else if len == max_len {
                max_for_len = max_for_len.max(strength);
            }
            (overall_max.max(strength), (max_len, max_for_len))
        },
    );
    writeln!(io::stdout(), "Day 24 Part 1: {}\nDay 24 Part 2: {}", p1, p2)?;
    Ok(())
}

#[derive(Clone)]
struct Port {
    front: usize,
    back: usize,
    weight: usize,
}

impl Port {
    fn from_str_tuple((s1, s2): (&str, &str)) -> Self {
        let front = s1.parse::<usize>().unwrap();
        let back = s2[1..].parse::<usize>().unwrap();
        Port {
            front,
            back,
            weight: front + back,
        }
    }

    fn rev(&mut self) {
        let temp = self.front;
        self.front = self.back;
        self.back = temp;
    }
}

struct Bridge {
    ports: Vec<Port>,
}

impl Bridge {
    fn len(&self) -> usize {
        self.ports.len()
    }

    fn strength(&self) -> usize {
        self.ports.iter().map(|p| p.weight).sum()
    }

    fn build(mut self, cursor: usize, pins: usize, tx: std::sync::mpsc::Sender<Self>) {
        if cursor < self.ports.len() {
            for i in cursor..self.ports.len() {
                if self.ports[i].front == pins || {
                    self.ports[i].rev();
                    self.ports[i].front == pins
                } {
                    let mut next_bridge = self.ports.clone();
                    next_bridge.swap(cursor, i);
                    let pins = next_bridge[cursor].back;
                    Bridge { ports: next_bridge }.build(cursor + 1, pins, tx.clone());
                }
            }
            self.ports.truncate(cursor);
            tx.send(self).expect("send failure")
        }
    }
}
