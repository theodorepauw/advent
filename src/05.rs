const INPUT: &str = include_str!("../inputs/05.txt");

fn main() {
    let mut seat_ids: Vec<u16> =  INPUT.lines().map(|s| {
        let binary = s.chars().map(|c| match c {
            'F'|'L' => '0',
            'B'|'R' => '1',
            _ => panic!("Invalid character!"),
        }).collect::<String>();
        let row = u16::from_str_radix(&binary[..7], 2).unwrap();
        let col = u16::from_str_radix(&binary[7..], 2).unwrap();
        row * 8 + col
    })
    .collect();

    seat_ids.sort_unstable();
    let p1 = &seat_ids[seat_ids.len()-1..];
    println!("Day 5 Part 1: {:?}", p1);
    for i in 1..seat_ids.len() {
        if seat_ids[i]-seat_ids[i-1] > 1 {
            println!("Day 5 Part 2: {}", seat_ids[i]-1);
            break
        }
    }
}
