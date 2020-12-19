const INPUT: &str = include_str!("../inputs/16.txt");

use std::{num::ParseIntError, str::FromStr};
#[derive(Clone)]
struct Rule {
    title: String,
    ranges: ((usize, usize), (usize, usize)),
}

#[derive(Clone)]
struct Ticket {
    fields: Vec<usize>,
    invalid: Option<usize>,
}
fn main() {
    let sections: Vec<&str> = INPUT.split("\n\n").collect();
    let (rules, your_ticket, nearby_tickets) = (
        sections[0],
        sections[1].strip_prefix("your ticket:\n").unwrap(),
        sections[2].strip_prefix("nearby tickets:\n").unwrap(),
    );
    let rules: Vec<Rule> = rules.lines().map(|s| Rule::from(s)).collect();
    let your_ticket: Ticket = your_ticket.parse().expect("couldn't decode your ticket");

    let (valid, invalid): (Vec<Ticket>, Vec<Ticket>) = nearby_tickets
        .lines()
        .map(|s| {
            s.parse::<Ticket>()
                .expect("couldn't decode nearby ticket")
                .sum_invalid_fields(&rules)
        })
        .partition(|t| t.invalid.expect("ticket not checked") == 0);

    let p1: usize = invalid.iter().flat_map(|t| t.invalid).sum();
    println!("Day 16 Part 1: {}", p1);

    let mut columns: Vec<usize> = vec![usize::MAX; your_ticket.fields.len()];
    for t in valid {
        for (col, &field) in t.fields.iter().enumerate() {
            let score: usize = (0..columns.len()).fold(0, |score, i| {
                score | if rules[i].followed_by(field) { 1 } else { 0 } << i
            });
            if score != 0 { // i.e. if the ticket is valid
                columns[col] &= score;
            }
        }
    }

    let mut columns: Vec<(usize, usize)> = columns
        .iter()
        .enumerate()
        .map(|(index, &score)| (index, score))
        .collect();
    columns.sort_unstable_by_key(|(_, score)| score.count_ones());

    for i in 0..columns.len() {
        for j in i + 1..columns.len() {
            columns[j].1 &= !columns[i].1
        }
        columns[i].1 = columns[i].1.trailing_zeros() as usize;
    }

    let p2: usize = columns
        .iter()
        .filter(|(_, r)| rules[*r].title.starts_with("departure"))
        .map(|(ticket_column, _)| your_ticket.fields[*ticket_column])
        .product();
    println!("Day 16 Part 2: {}", p2);
}

impl FromStr for Ticket {
    type Err = ParseIntError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let fields = line
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        let invalid = None;
        Ok(Ticket { fields, invalid })
    }
}

impl Rule {
    fn from(s: &str) -> Self {
        let range_from = |s: &str| -> (usize, usize) {
            let mut s = s.split('-');
            (
                s.next().unwrap().parse::<usize>().unwrap(),
                s.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        let mut s = s.split(": ");
        let title: String = s.next().expect("invalid rule").to_owned();
        let mut s = s.next().expect("ranges not found").split(" or ");
        let ranges = (range_from(s.next().unwrap()), range_from(s.next().unwrap()));

        Rule { title, ranges }
    }

    fn followed_by(&self, n: usize) -> bool {
        let (r1, r2) = self.ranges;
        (n >= r1.0 && n <= r1.1) || (n >= r2.0 && n <= r2.1)
    }
}

impl Ticket {
    fn sum_invalid_fields(self, rules: &[Rule]) -> Self {
        Ticket {
            invalid: Some(
                self.fields
                    .iter()
                    .filter(|&f| !rules.iter().any(|r| r.followed_by(*f)))
                    .sum(),
            ),
            fields: self.fields,
        }
    }
}
