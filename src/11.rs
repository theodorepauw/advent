use std::{collections::HashMap, convert::TryInto};

const INPUT: &str = include_str!("../inputs/11.txt");

/* This one is VERY slow and naive. I may optimise/rewrite it later. */

type Direction = (i8, i8);
const DELTAS: &[Direction] = &[(-1, -1),(-1, 0),(-1, 1),(0, -1),(0, 1),(1, -1),(1, 0),(1, 1),];
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i8,
    y: i8,
}

#[derive(Clone, Debug)]
struct Seats {
    visible: HashMap<Direction, HashMap<Point, Point>>,
    occupied: HashMap<Point, bool>,
    max_x: i8,
    max_y: i8,
}

fn main() {
    let mut seats = Seats::from(INPUT);
    seats.chart_visbible();

    part1(seats.clone());
    part2(seats);
}

// part1 and part2 are very similar. smartest thing would probably be to implement an optional distance param for count_visible
// -> count_adjacent could then become count_visible(point, Some(1)) or something similar

fn part1(mut seats: Seats) {
    loop {
        let flips: Vec<Point> = seats
            .occupied
            .keys()
            .map(|p| {
                (
                    p,
                    seats.is_occupied(p).expect("invalid seat!"),
                    seats.count_adjacent(&p),
                )
            })
            .filter(|(_, &occ, adjacent)| (occ && *adjacent > 3) || (!occ && *adjacent == 0))
            .map(|(p, _, _)| *p)
            .collect();
        seats.flip_seats(&flips);
        if flips.is_empty() {
            break;
        }
    }
    println!("Day 11 Part 1: {}", seats.count_occupied());
}

fn part2(mut seats: Seats) {
    loop {
        let flips: Vec<Point> = seats
            .occupied
            .keys()
            .map(|p| {
                (
                    p,
                    seats.is_occupied(p).expect("invalid seat!"),
                    seats.count_visible(&p),
                )
            })
            .filter(|(_, &occ, visible)| (occ && *visible > 4) || (!occ && *visible == 0))
            .map(|(p, _, _)| *p)
            .collect();
        seats.flip_seats(&flips);
        if flips.is_empty() {
            break;
        }
    }
    println!("Day 11 Part 2: {}", seats.count_occupied());
}
impl Seats {
    fn from(s: &str) -> Self {
        let mut cols = 0;
        let mut rows = 0;
        let mut seats: HashMap<Point, bool> = HashMap::new();
        for (y, row) in s.lines().enumerate() {
            for (x, block) in row.chars().enumerate() {
                match block {
                    'L' => seats.insert(Point::from(x, y), false),
                    '#' => seats.insert(Point::from(x, y), true),
                    '.' => None,
                    _ => panic!("unrecognised input!"),
                };
                cols = x;
            }
            rows = y;
        }

        Seats {
            visible: DELTAS.iter().map(|&d| (d, HashMap::new())).collect(),
            occupied: seats,
            max_x: cols.try_into().expect("too many cols"),
            max_y: rows.try_into().expect("too many rows"),
        }
    }

    fn count_occupied(&self) -> usize {
        self.occupied.values().filter(|&occ| *occ).count()
    }

    fn is_occupied(&self, p: &Point) -> Option<&bool> {
        self.occupied.get(p).map(|occ| occ)
    }

    fn count_adjacent(&self, p: &Point) -> usize {
        DELTAS
            .iter()
            .map(|d| p.shift(d))
            .filter(|adj| *self.is_occupied(adj).unwrap_or(&false))
            .count()
    }

    fn chart_visbible(&mut self) { // A naive attempt at memoization
        for &point in self.occupied.keys() {
            for d in DELTAS {
                let mut cursor = point;

                while !self
                    .visible
                    .get(d)
                    .expect("invalid dir")
                    .contains_key(&point)
                    && !(cursor.x < 0
                        || cursor.x > self.max_x
                        || cursor.y < 0
                        || cursor.y > self.max_y)
                {
                    cursor = cursor.shift(d);
                    if self.occupied.contains_key(&cursor) {
                        self.visible
                            .get_mut(d)
                            .expect("invalid dir")
                            .insert(point, cursor);
                        self.visible
                            .get_mut(&(-d.0, -d.1))
                            .expect("invalid dir")
                            .insert(cursor, point);
                    }
                }
            }
        }
    }

    fn count_visible(&self, point: &Point) -> usize {
        self.visible
            .iter()
            .filter(|(_, seen)| {
                (seen
                    .get(point)
                    .and_then(|seat| self.occupied.get(seat).and_then(|&occ| Some(occ))))
                .unwrap_or(false)
            })
            .count()
    }

    fn flip_seats(&mut self, seats: &[Point]) {
        seats.iter().for_each(|seat| {
            let is_occupied = self.occupied.get_mut(seat).expect("invalid seat");
            *is_occupied = !*is_occupied;
        });
    }
}

impl Point {
    fn from(x: usize, y: usize) -> Self {
        Point {
            x: x as i8,
            y: y as i8,
        }
    }

    fn shift(&self, d: &Direction) -> Self {
        Point {
            x: self.x + d.0,
            y: self.y + d.1,
        }
    }
}
