const MASK: usize = !(usize::MAX << 16);
const DIV: usize = 2147483647; // https://ariya.io/2007/02/modulus-with-mersenne-prime -> scored 100ms
const SHIFT: usize = usize::MIN.count_zeros() as usize - DIV.leading_zeros() as usize;
use std::io::{self, Write};

pub fn solve() -> crate::util::Result<()> {
    let (p1_gen_a, p1_gen_b) = (
        Generator::new(618, 16807, None),
        Generator::new(814, 48271, None),
    );
    let (p2_gen_a, p2_gen_b) = (
        Generator {
            criteria: Some(4),
            ..p1_gen_a
        },
        Generator {
            criteria: Some(8),
            ..p1_gen_b
        },
    );

    let p1 = count_matches(p1_gen_a, p1_gen_b, 40_000_000);
    let p2 = count_matches(p2_gen_a, p2_gen_b, 5_000_000);
    writeln!(io::stdout(), "Day 15 Part 1: {}\nDay 15 Part 2: {}", p1, p2,)?;
    Ok(())
}

fn count_matches(gen_a: Generator, gen_b: Generator, pairs: usize) -> usize {
    gen_a
        .zip(gen_b)
        .take(pairs)
        .filter(|(a, b)| a & MASK == b & MASK)
        .count()
}

struct Generator {
    current: usize,
    factor: usize,
    criteria: Option<usize>,
}

impl Generator {
    fn new(start: usize, factor: usize, criteria: Option<usize>) -> Self {
        Generator {
            current: start,
            factor,
            criteria,
        }
    }
}

impl Iterator for Generator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current *= self.factor;
            while self.current >= DIV {
                self.current = (self.current & DIV) + (self.current >> SHIFT)
            }

            if self
                .criteria
                .map_or_else(|| true, |x| self.current % x == 0)
            {
                return Some(self.current);
            }
        }
    }
}
