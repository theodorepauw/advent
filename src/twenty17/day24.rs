const INPUT: &str = include_str!("./inputs/24.txt");
use crate::util;
use std::cmp::Ordering;
use std::io::{self, Write};

type Route = (usize, Port);

pub fn solve() -> util::Result<()> {
    let ports: Vec<Route> = INPUT
        .lines()
        .map(str::parse)
        .enumerate()
        .map(|(i, res)| res.map(|p| (i, p)))
        .collect::<util::Result<Vec<Route>>>()?;

    let (p1, (_, p2)) = Bridge::new().build_on(&ports).fold(
        (0, (0, 0)),
        |(strongest, (longest, strongest_for_length)), b| {
            (
                strongest.max(b.strength),
                match b.length.cmp(&longest) {
                    Ordering::Greater => (b.length, b.strength),
                    Ordering::Equal => (b.length, b.strength.max(strongest_for_length)),
                    Ordering::Less => (longest, strongest_for_length),
                },
            )
        },
    );

    writeln!(io::stdout(), "Day 24 Part 1: {}\nDay 24 Part 2: {}", p1, p2)?;
    Ok(())
}

struct Port {
    front: usize,
    back: usize,
    weight: usize,
}

impl std::str::FromStr for Port {
    type Err = util::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_at(s.find('/').expect("no / sep"));
        let (front, back) = (s1.parse::<usize>()?, s2[1..].parse::<usize>()?);
        let weight = front + back;
        Ok(Port {
            front,
            back,
            weight,
        })
    }
}

#[derive(Clone)]
struct Bridge {
    next_pins: usize,
    length: usize,
    strength: usize,
    used: u64, // replacement for HashSet because input only contains 57 entries
}

impl Bridge {
    fn new() -> Self {
        Bridge {
            next_pins: 0,
            length: 0,
            strength: 0,
            used: 0,
        }
    }

    fn build_on<'a>(mut self, ports: &'a [Route]) -> Box<dyn Iterator<Item = Self> + 'a> {
        let (single, double): (Vec<_>, Vec<_>) = ports
            .iter()
            .filter(|(i, p)| {
                (self.used >> i) & 1 == 0 && (p.front == self.next_pins || p.back == self.next_pins)
            })
            .partition(|(_, p)| p.front != p.back);

        for (i, _) in double {
            // eagerly advance entries like 23/23 to prevent excessive branching
            self.used |= 1 << i;
            self.strength += self.next_pins << 1;
            self.length += 1;
        }

        if single.is_empty() {
            Box::new(std::iter::once(self)) as Box<dyn Iterator<Item = Bridge> + 'a>
        } else {
            Box::new(
                single
                    .into_iter()
                    .map(move |(i, p)| Bridge {
                        next_pins: if self.next_pins == p.front {
                            p.back
                        } else {
                            p.front
                        },
                        strength: self.strength + p.weight,
                        length: self.length + 1,
                        used: (self.used | (1 << i)),
                    })
                    .flat_map(move |b| b.build_on(ports)),
            )
        }
    }
}
