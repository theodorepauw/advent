const INPUT: &str = include_str!("./inputs/20.txt");
use std::io::{self, Write};
use std::{collections::HashMap, num::ParseIntError};

pub fn solve() -> crate::util::Result<()> {
    let mut particles: HashMap<usize, Particle> = INPUT
        .lines()
        .enumerate()
        .map(|(i, s)| (i, s.parse::<Particle>().expect("couldn't parse particle")))
        .collect();

    let (p1, _, _) = particles.iter().fold(
        // min by acceleration; break ties with velocity
        (0, i32::MAX, i32::MAX),
        |(index_min, acc_min, vel_min), (i, p)| {
            let a = p.acceleration.iter().map(|a| a.abs()).sum();
            let v = p.velocity.iter().map(|v| v.abs()).sum();
            if a < acc_min || (a == acc_min && v < vel_min) {
                (*i, a, v)
            } else {
                (index_min, acc_min, vel_min)
            }
        },
    );

    let p2 = loop {
        let mut grid: HashMap<Vec<i32>, Vec<usize>> = HashMap::new();
        for (i, p) in particles.iter_mut() {
            p.tick();
            grid.entry(p.position.clone()).or_default().push(*i);
        }
        for (_, ids) in grid.into_iter().filter(|(_, ids)| ids.len() > 1) {
            for id in ids {
                particles.remove(&id);
            }
        }

        if particles.iter().all(|(_, p)| p.receding) {
            break particles.len();
        }
    };

    writeln!(io::stdout(), "Day 20 Part 1: {}\nDay 20 Part 2: {}", p1, p2)?;
    Ok(())
}
struct Particle {
    position: Vec<i32>,
    velocity: Vec<i32>,
    acceleration: Vec<i32>,
    receding: bool,
}

impl Particle {
    fn tick(&mut self) {
        let (dims, mut is_rec) = (self.acceleration.len(), true);
        for d in 0..dims {
            self.velocity[d] += self.acceleration[d];
            self.position[d] += self.velocity[d];
            is_rec = is_rec
                && (self.position[d].is_positive() == self.velocity[d].is_positive()
                    && (self.velocity[d].is_positive() == self.acceleration[d].is_positive()
                        || self.acceleration[d] == 0)
                    || self.acceleration[d] == 0 && self.velocity[d] == 0);
        }
        self.receding = is_rec;
    }
}

impl std::str::FromStr for Particle {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let properties: Vec<_> = s.split(", ").collect();
        Ok(Particle {
            position: grab_coords(properties[0])?,
            velocity: grab_coords(properties[1])?,
            acceleration: grab_coords(properties[2])?,
            receding: false,
        })
    }
}

fn grab_coords(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s[3..s.len() - 1]
        .split(',')
        .map(|x| x.parse::<i32>())
        .collect()
}
