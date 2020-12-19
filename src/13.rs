const INPUT: &str = include_str!("../inputs/13.txt");

fn main() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let timestamp = lines[0].parse::<usize>().unwrap();
    let buses: &str = lines[1];

    part1(buses, timestamp);
    part2(buses);
}

fn part1(buses: &str, timestamp: usize) {
    let (id, wait) = buses
        .split_terminator(',')
        .filter(|&s| s != "x")
        .map(|s| {
            let id = s.parse::<usize>().unwrap();
            (id, id - timestamp % id)
        })
        .min_by_key(|(_, wait)| *wait)
        .unwrap();
    println!("Day 13 Part 1: {}", id * wait);
}

fn part2(buses: &str) {
    let buses: Vec<(usize, usize)> = buses
        .split_terminator(',')
        .enumerate()
        .filter(|&(_, bus)| bus != "x")
        .map(|(offset, id)| (id.parse::<usize>().unwrap(), offset))
        .collect();

    let (timestamp, _) = buses.iter().skip(1).fold(
        (buses[0].0, buses[0].0),
        |(mut time, step), (id, offset)| {
            while (time + offset) % id != 0 {
                time += step;
            }
            (time, step * id) // unncessary to do the whole lcm rigmarole for the steps & bus ids because ids are prime
        },
    );

    println!("Day 13 Part 2: {}", timestamp);
}
