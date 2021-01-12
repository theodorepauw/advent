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
        // use std::io::{self, Write};
        // writeln!(io::stdout(), "{}", d).unwrap();
        *self = (0..self.len())
            .map(|x| self[d * (x % d) + d - x / d - 1])
            .collect();
    }

    fn rotate_right(&mut self) {
        todo!()
    }

    fn flip_horizontal(&mut self) {
        let d = sqrt(self.len());
        // use std::io::{self, Write};
        // writeln!(io::stdout(), "{}", d).unwrap();
        *self = (0..self.len())
            .map(|x| self[(x / d) * d + d - 1 - (x % d)])
            .collect();
    }

    fn flip_vertical(&mut self) {
        let d = sqrt(self.len());
        // use std::io::{self, Write};
        // writeln!(io::stdout(), "{}", d).unwrap();
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
