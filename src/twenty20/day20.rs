const INPUT: &str = include_str!("./inputs/20.txt");
use crate::util;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Write};
use std::{collections::HashMap, iter};

lazy_static! {
    static ref TOP: Regex = Regex::new(r"..................#.").expect("top err");
    static ref MID: Regex = Regex::new(r"#....##....##....###").expect("mid err");
    static ref BOT: Regex = Regex::new(r".#..#..#..#..#..#...").expect("bot err");
}

pub fn solve() -> util::Result<()> {
    let mut edges: HashMap<String, Vec<usize>> = HashMap::new();
    let mut tiles: HashMap<usize, Tile> = INPUT
        .split("\n\n")
        .map(|s| s.parse::<Tile>().map(|t| (t.id, t)))
        .collect::<util::Result<_>>()?;

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
        .ok_or("part 2 is whack")?
        .id;
    let mut top_left = tiles.remove(&tl_id).expect("no ref tile"); // 1st corner found is made top left.
    while !(edges.get(&top_left.grid.rows[0]).map_or(0, |e| e.len())
        + edges
            .get(&rev(&top_left.grid.rows[0]))
            .map_or(0, |e| e.len())
        == 1
        && edges.get(&top_left.grid.cols[0]).map_or(0, |e| e.len())
            + edges
                .get(&rev(&top_left.grid.cols[0]))
                .map_or(0, |e| e.len())
            == 1)
    {
        writeln!(io::stdout(), "{}\n", top_left.grid.rows.join("\n"))?;
        top_left.grid.turn_left();
    }
    img.push(top_left);

    for x in 1..144 {
        if x % 12 == 0 {
            let e = &img[x - 12].grid.rows[9]; // thus try to match bottom side of anchored tile
            println!("{} {:?}{:?}", x, edges.get(e), edges.get(&rev(e)));
            let id: usize = *edges
                .get(e)
                .iter()
                .chain(edges.get(&rev(e)).iter())
                .flat_map(|v| v.iter())
                .find(|&&id| id != img[x - 12].id)
                .ok_or("no id")?;

            let mut next_tile = tiles.remove(&id).expect("no ref tile");
            while &next_tile.grid.rows[0] != e {
                println!("matching {}", next_tile.id);
                if &rev(&next_tile.grid.rows[0]) == e {
                    println!("flipping h");
                    next_tile.grid.flip_h();
                } else {
                    next_tile.grid.turn_left()
                }
            }
            img.push(next_tile);
        } else {
            let e = &img[x - 1].grid.cols[9]; // thus try to match right side of anchored tile
            writeln!(
                io::stdout(),
                "{} {:?}{:?}",
                x,
                edges.get(e),
                edges.get(&rev(e))
            )?;
            let id: usize = *edges
                .get(e)
                .iter()
                .chain(edges.get(&rev(e)).iter())
                .flat_map(|v| v.iter())
                .find(|&&id| id != img[x - 1].id)
                .ok_or("no id")?;

            let mut next_tile = tiles.remove(&id).expect("no ref tile");
            while &next_tile.grid.cols[0] != e {
                writeln!(io::stdout(), "matching {}", next_tile.id)?;
                if &rev(&next_tile.grid.cols[0]) == e {
                    println!("flipping v");
                    next_tile.grid.flip_v()
                } else {
                    next_tile.grid.turn_left()
                }
            }
            img.push(next_tile);
        }
    }
    let p1 = img[0].id * img[11].id * img[132].id * img[143].id;

    let mut img = Image::from(&img);
    img.rows_and_cols();
    let p2 = img.count_roughness();
    writeln!(io::stdout(), "Day 20 Part 1: {}\nDay 20 Part 2: {}", p1, p2)?;
    Ok(())
}

struct Tile {
    id: usize,
    grid: Grid,
}

struct Grid {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Grid {
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

impl std::str::FromStr for Tile {
    type Err = util::Error;
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

        Ok(Tile {
            id,
            grid: Grid { rows, cols },
        })
    }
}

impl Tile {
    fn edges(&self) -> impl Iterator<Item = &String> {
        let (t, b, l, r) = (
            iter::once(&self.grid.rows[0]),
            iter::once(&self.grid.rows[9]),
            iter::once(&self.grid.cols[0]),
            iter::once(&self.grid.cols[9]),
        );
        t.chain(b).chain(l).chain(r)
    }
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

struct Image {
    waters: String,
    grid: Grid,
}

impl Image {
    fn from(tiles: &[Tile]) -> Self {
        let mut waters = String::new();
        for block in tiles.chunks(12) {
            for r in 1..9 {
                for t in block {
                    waters.push_str(&t.grid.rows[r][1..9]);
                }
                waters.push('\n');
            }
        }

        Image {
            waters,
            grid: Grid { rows: vec![], cols: vec![] },
        }
    }

    fn count_roughness(&mut self) -> usize {
        self.grid
            .rows
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
        self.grid.rows = rows;
        self.grid.cols = cols;
    }

    fn find_sea_monsters(&mut self) -> usize {
        let mut count = 0;
        let mut i = 0usize;
        while count == 0 {
            for r in (2..self.grid.rows.len()).rev() {
                for x in 0..self.grid.rows.len() - 20 {
                    if BOT.is_match(&self.grid.rows[r][x..x + 20])
                        && MID.is_match(&self.grid.rows[r - 1][x..x + 20])
                        && TOP.is_match(&self.grid.rows[r - 2][x..x + 20])
                    {
                        count += 1;
                    }
                }
            }
            if count == 0 {
                if i > 0 && i % 4 == 0 {
                    if i % 8 == 0 {
                        println!("{} flipping v", i);
                        self.grid.flip_v()
                    } else {
                        println!("{} flipping h", i);
                        self.grid.flip_h()
                    }
                }
                self.grid.turn_left();
            }
            i += 1;
        }
        count
    }
}