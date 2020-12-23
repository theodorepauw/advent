use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

const INPUT: &str = include_str!("../inputs/22.txt");

fn main() {
    let players: Vec<Deck> = INPUT.splitn(2, "\n\n").map(|s| Deck::from(&s)).collect();
    let p1 = Combat::from(&players).play(false).score();
    let p2 = Combat::from(&players).play(true).score();
    println!("\nDay 22 Part 1: {}\nDay 22 Part 2: {}", p1, p2);
}

#[derive(Clone)]
struct Deck {
    id: Option<usize>,
    cards: VecDeque<usize>,
    states: HashSet<String>,
}

impl Deck {
    fn from(s: &str) -> Self {
        // Not in the mood for Result shenanigans with FromStr
        Deck {
            cards: VecDeque::from_iter(s.lines().skip(1).map(|s| s.parse().expect("card err"))),
            states: HashSet::new(),
            id: None,
        }
    }

    fn score(&self) -> usize {
        self.cards.iter().enumerate().fold(0, |score, (i, card)| {
            score + ((self.cards.len() - i) * card)
        })
    }

    fn draw(&mut self) -> Option<usize> {
        self.cards.pop_front()
    }

    fn take(&mut self, winning_card: usize, losing_card: usize) {
        self.cards.push_back(winning_card);
        self.cards.push_back(losing_card);
    }

    fn seen(&mut self) -> bool {
        !self.states.insert(self.to_string())
    }

    fn copy(&self, many: usize) -> Self {
        let mut cards = self.cards.clone();
        cards.truncate(many);
        Deck {
            cards: cards,
            states: HashSet::new(),
            id: self.id,
        }
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut cards = self.cards.iter();
        write!(fmt, "{}", cards.next().unwrap_or(&0))?;
        for c in cards {
            write!(fmt, " {}", c)?;
        }
        write!(fmt, "")
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
        let players = [self.p1.copy(p1_many), self.p2.copy(p2_many)];
        let winner = Combat::from(&players).play(true).id;
        winner
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
            } else {
                if self.recurse(c1, c2) == self.p1.id {
                    self.p1.take(c1, c2);
                } else {
                    self.p2.take(c2, c1);
                }
            }
        }
        if self.p1.cards.len() != 0 {
            self.p1
        } else {
            self.p2
        }
    }
}
