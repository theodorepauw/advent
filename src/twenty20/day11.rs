use std::collections::HashMap;
use std::io::{self, Write};

const INPUT: &str = include_str!("./inputs/11.txt");
const DX: &[i8] = &[-1, -1, -1, 0, 0, 1, 1, 1];
const DY: &[i8] = &[-1, 0, 1, -1, 1, -1, 0, 1];
type Seat = (i8, i8);
type Direction = (i8, i8);

#[derive(Clone)]
struct WaitingArea {
    visible: HashMap<Seat, HashMap<Direction, Seat>>, // each seat stores a key (direction) and corresponding value (seat in that direction)
    occupied: HashMap<Seat, bool>,
    max_x: i8,
    max_y: i8,
}

pub fn solve() -> crate::util::Result<()> {
    let seats = WaitingArea::from(INPUT);
    let (p1, p2) = (run(seats.clone(), 3, true), run(seats, 4, false));
    writeln!(io::stdout(), "Day 11 Part 1: {}\nDay 11 Part 2: {}", p1, p2)?;
    Ok(())
}

fn run(mut area: WaitingArea, threshold: usize, only_adjacent: bool) -> usize {
    loop {
        let flips: Vec<(i8, i8)> = area
            .occupied
            .keys()
            .map(|p| {
                (
                    p,
                    area.occupied.get(p).expect("invalid seat!"),
                    area.count_visible(&p, only_adjacent),
                )
            })
            .filter(|(_, &occ, visible)| (occ && *visible > threshold) || (!occ && *visible == 0))
            .map(|(p, _, _)| *p)
            .collect();
        area.flip_seats(&flips);
        if flips.is_empty() {
            return area.count_occupied();
        }
    }
}

impl WaitingArea {
    fn from(s: &str) -> Self {
        let mut occupied: HashMap<(i8, i8), bool> = HashMap::new();
        let (max_x, max_y) = s.trim().chars().fold((0i8, 0i8), |(mut x, mut y), block| {
            if block == '\n' {
                y += 1;
                x = 0;
            } else {
                x += 1;
                if block != '.' {
                    occupied.insert((x, y), block == '#');
                }
            }
            (x, y)
        });

        let mut visible = DX
            .iter()
            .zip(DY.iter())
            .map(|(&x, &y)| ((x, y), HashMap::new()))
            .collect::<HashMap<_, _>>();

        for &point in occupied.keys() {
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let ((mut x, mut y), d) = (point, (dx, dy));

                while !(visible.get(&d).expect("invalid dir").contains_key(&point)
                    || x < 0
                    || x > max_x
                    || y < 0
                    || y > max_y)
                {
                    x += dx;
                    y += dy;
                    if occupied.contains_key(&(x, y)) {
                        visible
                            .get_mut(&d)
                            .expect("invalid dir")
                            .insert(point, (x, y));
                        visible
                            .get_mut(&(-dx, -dy))
                            .expect("invalid dir")
                            .insert((x, y), point);
                    }
                }
            }
        }

        WaitingArea {
            visible,
            occupied,
            max_x,
            max_y: max_y + 1, // last \n is trimmed off
        }
    }

    fn count_occupied(&self) -> usize {
        self.occupied.values().filter(|&occ| *occ).count()
    }

    fn count_visible(&self, point: &(i8, i8), only_adjacent: bool) -> usize {
        self.visible
            .iter()
            .filter(|((dx, dy), seen)| {
                seen.get(point).map_or_else(
                    || false,
                    |&(x, y)| {
                        *self.occupied.get(&(x, y)).unwrap_or(&false)
                            && (!only_adjacent || (x - point.0 == *dx && y - point.1 == *dy))
                    },
                )
            })
            .count()
    }

    fn flip_seats(&mut self, seats: &[(i8, i8)]) {
        seats.iter().for_each(|seat| {
            *self.occupied.get_mut(seat).expect("invalid seat") ^= true;
        })
    }
}
