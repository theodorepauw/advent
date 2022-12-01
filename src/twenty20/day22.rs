use std::{
    collections::{HashSet, VecDeque},
    io::{self, Write},
};
type Card = usize;

lazy_static::lazy_static! {
    static ref PLAYERS: Vec<Deck> = include_str!("./inputs/22.txt").splitn(2, "\n\n").map(Deck::from).collect();
}

pub fn solve() -> crate::util::Result<()> {
    let p1 = Combat::from(&PLAYERS).play(false).score();
    writeln!(io::stdout(), "Day 22 Part 1: {}", p1)?;
    let p2 = Combat::from(&PLAYERS).play(true).score();
    writeln!(io::stdout(), "Day 22 Part 2: {}", p2,)?;
    Ok(())
}

#[derive(Clone)]
struct Deck {
    id: Option<usize>,
    cards: VecDeque<Card>,
    states: HashSet<VecDeque<usize>>,
}

impl Deck {
    fn from(s: &str) -> Self {
        // Not in the mood for Result shenanigans with FromStr
        Deck {
            cards: s
                .lines()
                .skip(1)
                .map(|s| s.parse().expect("card err"))
                .collect(),
            states: HashSet::new(),
            id: None,
        }
    }

    fn score(&self) -> usize {
        self.cards.iter().enumerate().fold(0, |score, (i, card)| {
            score + ((self.cards.len() - i) * card)
        })
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    fn take(&mut self, winning_card: Card, losing_card: Card) {
        self.cards.push_back(winning_card);
        self.cards.push_back(losing_card);
    }

    fn seen(&mut self) -> bool {
        !self.states.insert(self.cards.clone())
    }

    fn copy(&self, many: usize) -> Self {
        let mut cards = self.cards.clone();
        cards.truncate(many);
        Deck {
            cards,
            states: HashSet::new(),
            id: self.id,
        }
    }
}

struct Combat {
    p1: Deck,
    p2: Deck,
}

impl Combat {
    fn from(players: &[Deck]) -> Self {
        let (mut p1, mut p2) = (players[0].clone(), players[1].clone());
        p1.id = Some(1);
        p2.id = Some(2);
        Self { p1, p2 }
    }

    fn recurse(&self, p1_many: usize, p2_many: usize) -> Option<usize> {
        Combat::from(&[self.p1.copy(p1_many), self.p2.copy(p2_many)])
            .play(true)
            .id
    }

    fn play(mut self, recursive: bool) -> Deck {
        while !(self.p1.cards.is_empty() || self.p2.cards.is_empty()) {
            if recursive && self.p1.seen() && self.p2.seen() {
                return self.p1;
            }
            let (c1, c2) = (self.p1.draw().unwrap(), self.p2.draw().unwrap());

            if !recursive || self.p1.cards.len() < c1 || self.p2.cards.len() < c2 {
                if c1 > c2 {
                    self.p1.take(c1, c2);
                } else {
                    self.p2.take(c2, c1);
                }
            } else if self.recurse(c1, c2) == self.p1.id {
                self.p1.take(c1, c2);
            } else {
                self.p2.take(c2, c1);
            }
        }

        match self.p1.cards.is_empty() {
            false => self.p1,
            _ => self.p2,
        }
    }
}
