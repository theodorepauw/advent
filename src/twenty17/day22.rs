use std::collections::HashMap;
use std::io::{self, Write};
const INPUT: &str = include_str!("./inputs/22.txt");
const DX: &[i32] = &[0, 1, 0, -1]; // up -> right -> down -> left
const DY: &[i32] = &[1, 0, -1, 0]; // up -> right -> down -> left
type Coord = (i32, i32);

pub fn solve() -> crate::util::Result<()> {
    let (mut infected_p1, mut infected_p2): (HashMap<Coord, bool>, HashMap<Coord, u8>) =
        (HashMap::new(), HashMap::new());
    let (mut x_p1, mut y_p1) = INPUT.chars().fold((0, 0), |(mut x, mut y), c| {
        match c {
            '\n' => {
                x = 0;
                y -= 1;
            }
            _ => {
                if c == '#' {
                    infected_p1.insert((x, y), true);
                    infected_p2.insert((x, y), 2);
                }
                x += 1;
            }
        }
        (x, y)
    });

    x_p1 >>= 1;
    y_p1 >>= 1;
    let (mut x_p2, mut y_p2) = (x_p1, y_p1);

    let (mut d_p1, mut d_p2) = (0usize, 0usize);
    let (mut p1, mut p2) = (0usize, 0usize);

    for bursts in 0..10000000 {
        // Part 1
        if bursts < 10_000 {
            let is_infected = infected_p1.entry((x_p1, y_p1)).or_default();
            if *is_infected {
                d_p1 = (d_p1 + 1) & 3; // wrapping add (&3 == %4)
            } else {
                p1 += 1;
                d_p1 = (d_p1 + 3) & 3; // wrapping sub
            }
            *is_infected ^= true;
            x_p1 += DX[d_p1];
            y_p1 += DY[d_p1];
        }

        // Part 2:
        let status = infected_p2.entry((x_p2, y_p2)).or_default();
        if *status == 0 {
            // clean
            d_p2 = (d_p2 + 3) & 3;
        } else if *status == 1 {
            // weakened
            p2 += 1;
        } else if *status == 2 {
            // infected
            d_p2 = (d_p2 + 1) & 3;
        } else if *status == 3 {
            // flagged
            d_p2 = (d_p2 + 2) & 3;
        }

        *status = (*status + 1) & 3;
        x_p2 += DX[d_p2];
        y_p2 += DY[d_p2];
    }

    writeln!(
        io::stdout(),
        "Day 22 Part 1: {}\nDay 22 Part 2: {}\n\n",
        p1,
        p2,
    )?;
    Ok(())
}
