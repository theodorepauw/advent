const INPUT: &str = include_str!("../inputs/12.txt");

fn main() {
    let map: Vec<(&str, i32)> = INPUT
        .split_whitespace()
        .map(|s| (&s[0..1], s[1..].parse::<i32>().expect("invalid num")))
        .collect();
    part1(&map);
    part2((10, 1), &map);
}

fn part1(map: &[(&str, i32)]) {
    const ROTATIONS: &[(i32, i32)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)]; // E -> N -> W -> S

    let (x, y, _) = map.iter().fold((0, 0, 0), |(x, y, r), (action, d)| { // r = 90° anti-clockwise rotations with East = 0
        match *action {
            "N" => (x, y + d, r),
            "S" => (x, y - d, r),
            "E" => (x + d, y, r),
            "W" => (x - d, y, r),
            "L" => (x, y, (r + d / 90).rem_euclid(4)), // Rust's % is not modulo but rather remainder
            "R" => (x, y, (r - d / 90).rem_euclid(4)),
            "F" => (
                x + d * ROTATIONS[r as usize].0,
                y + d * ROTATIONS[r as usize].1,
                r,
            ),
            _ => panic!("invalid action"),
        }
    });

    println!("Day 12 Part 1: {}", x.abs() + y.abs());
}

fn part2(waypoint: (i32, i32), map: &[(&str, i32)]) {
    let rotate_left = |x: i32, y: i32, deg: i32| -> (i32, i32) { // rotate 90° left: (x, y) -> (-y, x)
        (0..(deg as usize / 90)).fold((x, y), |(x, y), _| (-y, x)) 
    };
    let rotate_right = |x: i32, y: i32, deg: i32| -> (i32, i32) { // rotate 90° right: (x, y) -> (y, -x)
        (0..(deg as usize / 90)).fold((x, y), |(x, y), _| (y, -x))
    };

    let ((x, y), (_, _)) = map.iter().fold(
        ((0, 0), waypoint),
        |((ship_x, ship_y), (waypoint_x, waypoint_y)), (action, d)| match *action {
            "N" => ((ship_x, ship_y), (waypoint_x, waypoint_y + d)),
            "S" => ((ship_x, ship_y), (waypoint_x, waypoint_y - d)),
            "E" => ((ship_x, ship_y), (waypoint_x + d, waypoint_y)),
            "W" => ((ship_x, ship_y), (waypoint_x - d, waypoint_y)),
            "L" => ((ship_x, ship_y), rotate_left(waypoint_x, waypoint_y, *d)),
            "R" => ((ship_x, ship_y), rotate_right(waypoint_x, waypoint_y, *d)),
            "F" => (
                (ship_x + d * waypoint_x, ship_y + d * waypoint_y),
                (waypoint_x, waypoint_y),
            ),
            _ => panic!("invalid action"),
        },
    );

    println!("Day 12 Part 2: {}", x.abs() + y.abs());
}
