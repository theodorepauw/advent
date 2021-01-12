const INPUT: &str = include_str!("./inputs/20.txt");
use crate::util::{self, SquareVec};
use std::io::{self, Write};
use std::{
    collections::HashMap,
    iter::{once, Chain, Once},
};

const PATTERN: &[&str] = &[
    "..................#.",
    "#....##....##....###",
    ".#..#..#..#..#..#...",
];
lazy_static::lazy_static! {
    static ref MONSTER: Vec<Vec<usize>> = PATTERN.iter().map(|s| s.chars().enumerate().filter(|(_, c)| *c=='#').map(|(i, _)| i).collect()).collect();
}

type Edge = String;
type Tile = Vec<char>;
type Image = Vec<Tile>;

pub fn solve() -> util::Result<()> {
    let mut edges: HashMap<Edge, Vec<usize>> = HashMap::new();
    let mut tiles: HashMap<usize, Tile> = HashMap::new();
    for s in INPUT.split("\n\n") {
        let mut split = s.splitn(2, '\n');
        let id = split
            .next()
            .ok_or("no id")?
            .get(5..9)
            .ok_or("tile id fmt wrong")?
            .parse::<usize>()?;
        let tile: Tile = split
            .next()
            .ok_or("no img data")?
            .chars()
            .filter(|&p| p == '#' || p == '.')
            .collect();
        for e in tile.edges() {
            edges.entry(e).or_default().push(id);
        }
        tiles.insert(id, tile);
    }

    let count_parents =
        |ids: Option<&Vec<usize>>| -> usize { ids.map(|ids| ids.len()).unwrap_or(0) };
    let is_shared = |e: &Edge, map: &HashMap<Edge, Vec<usize>>| -> bool {
        count_parents(map.get(e)) + count_parents(map.get(&rev(&e))) == 2
    };

    let corner_id = tiles
        .iter()
        .find(|(_, t)| t.edges().filter(|e| is_shared(e, &edges)).count() == 2)
        .map(|(id, _)| *id)
        .ok_or("no corner tile found")?;

    let n_tiles = tiles.len();
    let mut corner = tiles.remove(&corner_id).ok_or("no top left tile")?;
    let d = util::sqrt(n_tiles);
    while is_shared(&corner.left(), &edges) || is_shared(&corner.top(), &edges) {
        corner.rotate_left();
    }

    let mut img = Image::with_capacity(n_tiles);
    let mut ids = Vec::with_capacity(n_tiles);
    ids.push(corner_id);
    img.push(corner);

    let get_other_id = |edge: &Edge, edges: &HashMap<Edge, Vec<usize>>, prev_id| -> Option<usize> {
        edges
            .get(edge)
            .iter()
            .chain(edges.get(&rev(edge)).iter())
            .flat_map(|v| v.iter().cloned())
            .find(|&id| id != prev_id)
    };

    for i in 1..n_tiles {
        if i % d != 0 {
            let (prev_id, edge) = (ids[i - 1], img[i - 1].right()); // find tile with left edge == previous right edge
            let curr_id = get_other_id(&edge, &edges, prev_id).ok_or("no L-R edge")?;
            let mut matching_tile = tiles.remove(&curr_id).ok_or("no tile with L edge")?;
            loop {
                let x = matching_tile.left();
                if x == edge {
                    break;
                } else if rev(&x) == edge {
                    matching_tile.flip_vertical();
                    break;
                }
                matching_tile.rotate_left();
            }

            img.push(matching_tile);
            ids.push(curr_id);
        } else {
            let (prev_id, edge) = (ids[i - d], img[i - d].bot()); // find tile with top edge == tile above's bottom edge
            let curr_id = get_other_id(&edge, &edges, prev_id).ok_or("no T-B edge")?;
            let mut matching_tile = tiles.remove(&curr_id).ok_or("no tile with T edge")?;
            loop {
                let x = matching_tile.top();
                if x == edge {
                    break;
                } else if rev(&x) == edge {
                    matching_tile.flip_horizontal();
                    break;
                }
                matching_tile.rotate_left();
            }
            img.push(matching_tile);
            ids.push(curr_id);
        }
    }
    let p1 = ids[0] * ids[11] * ids[132] * ids[143];
    let tile_d = util::sqrt(img[0].len());
    let mut waters = Tile::new(); // assemble all tiles into one big tile
    for block in img.chunks(d) {
        for r in 1..tile_d - 1 {
            for t in block {
                let start = r * tile_d;
                waters.extend(t[start + 1..start + tile_d - 1].iter());
            }
        }
    }

    let (mut count, mut i, waters_d) = (0usize, 0usize, util::sqrt(waters.len()));
    while count == 0 {
        for row in 0..waters_d - MONSTER.len() {
            for col in 0..waters_d - PATTERN[0].len() {
                if MONSTER.iter().enumerate().all(|(row_offset, spots)| {
                    spots.iter().all(|&col_offset| {
                        waters[(row + row_offset) * waters_d + col + col_offset] == '#'
                    })
                }) {
                    count += 1;
                }
            }
        }

        i += 1;
        if count == 0 {
            if i % 4 == 0 {
                if i % 8 == 0 {
                    waters.flip_vertical();
                } else {
                    waters.flip_horizontal();
                }
            }
            waters.rotate_left();
        }
    }

    let p2 = waters.iter().filter(|&&c| c == '#').count()
        - (count * MONSTER.iter().map(|p| p.len()).sum::<usize>());
    writeln!(
        io::stdout(),
        "Day 20 Part 1: {}\nDay 20 Part 2: {}",
        p1,
        p2,
    )?;
    Ok(())
}

trait Square<'a>: SquareVec<'a, char> {
    fn top(&self) -> Edge {
        self.top_edge().collect()
    }
    fn bot(&self) -> Edge {
        self.bottom_edge().collect()
    }
    fn left(&self) -> Edge {
        self.left_edge().collect()
    }
    fn right(&self) -> Edge {
        self.right_edge().collect()
    }

    fn edges(&self) -> Chain<Chain<Chain<Once<Edge>, Once<Edge>>, Once<Edge>>, Once<Edge>> {
        once(self.top())
            .chain(once(self.left()))
            .chain(once(self.bot()))
            .chain(once(self.right()))
    }
}

impl Square<'_> for Tile {}

fn rev(s: &str) -> Edge {
    s.chars().rev().collect()
}
