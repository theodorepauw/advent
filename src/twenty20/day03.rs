use std::io::{self, Write};
const INPUT: &str = include_str!("./inputs/03.txt");

pub fn solve() -> crate::util::Result<()> {
    let map = Map::from(INPUT);
    const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]; // (right, down)

    let mut toboggans: Vec<Toboggan> = SLOPES
        .iter()
        .map(|(right, down)| Toboggan::with_slope(*right, *down))
        .collect();

    toboggans.iter_mut().for_each(|t| {
        (1..map.len())
            .step_by(t.slope.down)
            .for_each(|_| t.traverse(&map));
    });
    let (p1, p2) = (
        toboggans[1].trees_found,
        toboggans.iter().map(|t| t.trees_found).product::<usize>(),
    );

    writeln!(io::stdout(), "Day 03 Part 1: {}\nDay 03 Part 2: {}", p1, p2)?;
    Ok(())
}
enum Square {
    Open,
    Tree,
}
struct Map(Vec<Vec<Square>>);
struct Slope {
    right: usize,
    down: usize,
}
struct Toboggan {
    x: usize,
    y: usize,
    trees_found: usize,
    slope: Slope,
}

impl Toboggan {
    fn with_slope(right: usize, down: usize) -> Self {
        Toboggan {
            x: 0,
            y: 0,
            trees_found: 0,
            slope: Slope { right, down },
        }
    }

    fn traverse(&mut self, map: &Map) {
        self.x += self.slope.right;
        self.y += self.slope.down;
        if let Square::Tree = map.lookup(self.x, self.y) {
            self.trees_found += 1;
        }
    }
}

impl Map {
    fn from(s: &str) -> Self {
        Map(s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Square::Open,
                        '#' => Square::Tree,
                        _ => panic!("Unrecognised input character: {}", c),
                    })
                    .collect()
            })
            .collect())
    }

    fn lookup(&self, x: usize, y: usize) -> &Square {
        &self.0[y][x % self.width()]
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }
}
