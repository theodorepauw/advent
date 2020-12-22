use std::{collections::HashMap, iter};

const INPUT: &str = include_str!("../inputs/20.txt");

fn main() {
    let mut edges: HashMap<&String, usize> = HashMap::new();
    let tiles: Vec<Tile> = INPUT
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .expect("Couldn't parse tiles");
    for t in tiles.iter() {
        for e in t.edges() {
            *edges.entry(e).or_default() += 1;
        }
    }

    let corners: usize = tiles
        .iter()
        .filter(|t| {
            t.edges()
                .filter(|e| edges.get(e).unwrap() + edges.get(&rev(e)).unwrap_or(&0) == 1)
                .count()
                == 2
        })
        .map(|t| t.id)
        .product();
    println!("Day 20 Part 1: {}", corners);
}

struct Tile {
    id: usize,
    rows: Vec<String>,
    cols: Vec<String>,
}

impl std::str::FromStr for Tile {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.splitn(2, '\n');
        let id = s
            .next()
            .ok_or("no id")?
            .get(5..9)
            .ok_or("tile id fmt")?
            .parse::<usize>()?;
        let (rows, cols) = s.next().ok_or("no data")?.lines().fold(
            (vec![], vec![String::new(); 10]),
            |(mut rows, mut cols), line| {
                rows.push(line.to_owned());
                line.chars().enumerate().for_each(|(i, c)| cols[i].push(c));
                (rows, cols)
            },
        );

        Ok(Tile { id, rows, cols })
    }
}

impl Tile {
    fn edges(&self) -> impl Iterator<Item = &String> {
        let (t, b, l, r) = (
            iter::once(&self.rows[0]),
            iter::once(&self.rows[9]),
            iter::once(&self.cols[0]),
            iter::once(&self.cols[9]),
        );
        t.chain(b).chain(l).chain(r)
    }
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}