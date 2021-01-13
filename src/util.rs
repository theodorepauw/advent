use std::iter::{Cloned, Skip, StepBy, Take};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub trait SquareVec<'a, T> {
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
    fn left_edge(&self) -> Cloned<StepBy<std::slice::Iter<'_, T>>>;
    fn right_edge(&self) -> Cloned<StepBy<Skip<std::slice::Iter<'_, T>>>>;
    fn top_edge(&self) -> Cloned<Take<std::slice::Iter<'_, T>>>;
    fn bottom_edge(&self) -> Cloned<Skip<std::slice::Iter<'_, T>>>;
}

impl<T: Copy> SquareVec<'_, T> for Vec<T> {
    fn rotate_left(&mut self) {
        let d = sqrt(self.len());
        *self = (0..self.len())
            .map(|x| self[d * (x % d) + d - x / d - 1])
            .collect();
    }

    fn rotate_right(&mut self) {
        todo!()
    }

    fn flip_horizontal(&mut self) {
        let d = sqrt(self.len());
        *self = (0..self.len())
            .map(|x| self[(x / d) * d + d - 1 - (x % d)])
            .collect();
    }

    fn flip_vertical(&mut self) {
        let d = sqrt(self.len());
        *self = (0..self.len())
            .map(|x| self[d * (d - 1 - x / d) + x % d])
            .collect()
    }

    fn left_edge(&self) -> Cloned<StepBy<std::slice::Iter<'_, T>>> {
        let d = sqrt(self.len());
        self.iter().step_by(d).cloned()
    }

    fn right_edge(&self) -> Cloned<StepBy<Skip<std::slice::Iter<'_, T>>>> {
        let d = sqrt(self.len());
        self.iter().skip(d - 1).step_by(d).cloned()
    }

    fn top_edge(&self) -> Cloned<Take<std::slice::Iter<'_, T>>> {
        let d = sqrt(self.len());
        self.iter().take(d).cloned()
    }

    fn bottom_edge(&self) -> Cloned<Skip<std::slice::Iter<'_, T>>> {
        let d = sqrt(self.len());
        self.iter().skip(self.len() - d).cloned()
    }
}

pub fn sqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}

pub fn digits(mut n: usize) -> impl Iterator<Item = usize> {
    let log10 = ((0usize.count_zeros() + 1 - n.leading_zeros()) * 1233) >> 12; // base10 log 2 = 0.30102999566 -> approx 1233/4096
    let mut splitter = 10usize.pow(log10);
    if n < splitter {
        splitter /= 10;
    }
    std::iter::from_fn(move || match splitter > 0 {
        true => {
            let digit = n / splitter;
            n %= splitter;
            splitter /= 10;
            Some(digit)
        }
        _ => None,
    })
}

pub fn tie_knots(lengths: &[usize], rounds: usize) -> Vec<usize> {
    const MARKS: usize = 256;
    let (mut circle, mut curr_pos, mut skip) = ((0..MARKS).collect::<Vec<_>>(), 0, 0);
    for _ in 0..rounds {
        for len in lengths {
            for i in 0..len / 2 {
                circle.swap((curr_pos + i) % MARKS, (curr_pos + len - 1 - i) % MARKS);
            }
            curr_pos += len + skip;
            skip += 1;
        }
    }
    circle
}

pub fn knot_hash(sparse_hash: &[usize]) -> u128 {
    sparse_hash.chunks(16).fold(0, |dense_hash, chunk| {
        (dense_hash << 8) | chunk.iter().fold(0, |num, d| num ^ d) as u128
    })
}
