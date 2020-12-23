use std::{collections::HashMap, iter};

const INPUT: &str = include_str!("../inputs/20.txt");

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TOP: Regex = Regex::new(r"..................#.").expect("top err");
    static ref MID: Regex = Regex::new(r"#....##....##....###").expect("mid err");
    static ref BOT: Regex = Regex::new(r".#..#..#..#..#..#...").expect("bot err");
}

fn main() {
    let mut edges: HashMap<String, Vec<usize>> = HashMap::new();
    let mut tiles: HashMap<usize, Tile> = INPUT
        .split("\n\n")
        .map(|s| s.parse::<Tile>().map(|t| (t.id, t)))
        .collect::<Result<HashMap<_, _>, _>>()
        .expect("Couldn't parse tiles");

    tiles.values().for_each(|t| {
        t.edges()
            .for_each(|e| edges.entry(e.clone()).or_default().push(t.id))
    });
    let mut img: Vec<Tile> = Vec::with_capacity(144);
    let tl_id = tiles
        .values()
        .find(|&t| {
            t.edges()
                .filter(|&e| {
                    edges.get(e).unwrap().len() + edges.get(&rev(e)).map_or(0, |e| e.len()) == 1
                })
                .count()
                == 2
        })
        .expect("part 2 is whack")
        .id;
    let mut top_left = tiles.remove(&tl_id).expect("no ref tile"); // 1st corner found is made top left.
    while !(edges.get(&top_left.rows[0]).map_or(0, |e| e.len())
        + edges.get(&rev(&top_left.rows[0])).map_or(0, |e| e.len())
        == 1
        && edges.get(&top_left.cols[0]).map_or(0, |e| e.len())
            + edges.get(&rev(&top_left.cols[0])).map_or(0, |e| e.len())
            == 1)
    {
        println!("{}\n", top_left.rows.join("\n"));
        top_left.turn_left();
    }
    img.push(top_left);

    for x in 1..144 {
        if x % 12 == 0 {
            let e = &img[x - 12].rows[9]; // thus try to match right side of anchored tile
            println!("{} {:?}{:?}", x, edges.get(e), edges.get(&rev(e)));
            let id: usize = *edges
                .get(e)
                .iter()
                .chain(edges.get(&rev(e)).iter())
                .flat_map(|v| v.into_iter())
                .filter(|&&id| id != img[x - 12].id)
                .next()
                .expect("no id");

            let mut next_tile = tiles.remove(&id).expect("no ref tile");
            while &next_tile.rows[0] != e {
                println!("matching {}", next_tile.id);
                if &rev(&next_tile.rows[0]) == e {
                    println!("flipping h");
                    next_tile.flip_h();
                } else {
                    next_tile.turn_left()
                }
            }
            img.push(next_tile);
        } else {
            let e = &img[x - 1].cols[9]; // thus try to match right side of anchored tile
            println!("{} {:?}{:?}", x, edges.get(e), edges.get(&rev(e)));
            let id: usize = *edges
                .get(e)
                .iter()
                .chain(edges.get(&rev(e)).iter())
                .flat_map(|v| v.into_iter())
                .filter(|&&id| id != img[x - 1].id)
                .next()
                .expect("no id");

            let mut next_tile = tiles.remove(&id).expect("no ref tile");
            while &next_tile.cols[0] != e {
                println!("matching {}", next_tile.id);
                if &rev(&next_tile.cols[0]) == e {
                    println!("flipping v");
                    next_tile.flip_v()
                } else {
                    next_tile.turn_left()
                }
            }
            img.push(next_tile);
        }
    }
    let p1 = img[0].id * img[11].id * img[132].id * img[143].id;

    let mut img = Image::from(&img);
    img.rows_and_cols();
    let p2 = img.count_roughness();
    println!("Day 20 Part 1: {}\nDay 20 Part 2: {}", p1, p2);
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

    fn turn_left(&mut self) {
        let new_cols = self.rows.iter().map(|s| rev(s)).collect();
        self.rows = self.cols.clone().into_iter().rev().collect();
        self.cols = new_cols;
    }

    fn flip_v(&mut self) {
        self.rows = self.rows.clone().into_iter().rev().collect();
        self.cols = self.cols.clone().into_iter().map(|s| rev(&s)).collect();
    }

    fn flip_h(&mut self) {
        self.cols = self.cols.clone().into_iter().rev().collect();
        self.rows = self.rows.clone().into_iter().map(|s| rev(&s)).collect();
    }
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

struct Image {
    waters: String,
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Image {
    fn from(tiles: &[Tile]) -> Self {
        let mut waters = String::new();
        for block in tiles.chunks(12) {
            for r in 1..9 {
                for t in block {
                    waters.push_str(&t.rows[r][1..9]);
                }
                waters.push('\n');
            }
        }
        let rows = vec![];
        let cols = vec![];

        Image { waters, rows, cols }
    }

    fn count_roughness(&mut self) -> usize {
        self.rows
            .iter()
            .map(|x| x.chars().filter(|&c| c == '#').count())
            .sum::<usize>()
            - self.find_sea_monsters() * 15
    }

    fn rows_and_cols(&mut self) {
        let (rows, cols) = self.waters.lines().fold(
            (vec![], vec![String::new(); 96]),
            |(mut rows, mut cols), line| {
                rows.push(line.to_owned());
                line.chars().enumerate().for_each(|(i, c)| cols[i].push(c));
                (rows, cols)
            },
        );
        self.rows = rows;
        self.cols = cols;
    }

    fn turn_left(&mut self) {
        let new_cols: Vec<String> = self.rows.iter().map(|s| rev(s)).collect();
        self.rows = self.cols.clone().into_iter().rev().collect();
        self.cols = new_cols;
    }

    fn flip_v(&mut self) {
        self.rows = self.rows.clone().into_iter().rev().collect();
        self.cols = self.cols.clone().into_iter().map(|s| rev(&s)).collect();
    }

    fn flip_h(&mut self) {
        self.cols = self.cols.clone().into_iter().rev().collect();
        self.rows = self.rows.clone().into_iter().map(|s| rev(&s)).collect();
    }

    fn find_sea_monsters(&mut self) -> usize {
        let mut count = 0;
        let mut i = 0usize;
        while count == 0 {
            for r in (2..self.rows.len()).rev() {
                for x in 0..self.rows.len() - 20 {
                    if BOT.is_match(&self.rows[r][x..x + 20])
                        && MID.is_match(&self.rows[r - 1][x..x + 20])
                        && TOP.is_match(&self.rows[r - 2][x..x + 20])
                    {
                        count += 1;
                    }
                }
            }
            if count == 0 {
                if i > 0 {
                    if i % 4 == 0 {
                        if i % 8 == 0 {
                            println!("{} flipping v", i);
                            self.flip_v()
                        } else {
                            println!("{} flipping h", i);
                            self.flip_h()
                        }
                    }
                }
                self.turn_left();
            }
            i += 1;
        }
        count
    }
}