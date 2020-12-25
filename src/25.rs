const INPUT: &str = include_str!("../inputs/25.txt");
const INIT_SN: usize = 7;
fn main() {
    let public_keys: Vec<_> = INPUT
        .lines()
        .map(|s| s.parse::<usize>().expect("parse err"))
        .collect();
    let card_loop = calc_loop(public_keys[0]);
    let p1 = (0..card_loop).fold(1, |val, _| transform(val, public_keys[1]));
    println!("Day 25 Part 1: {}", p1);
}

fn transform(val: usize, sn: usize) -> usize {
    (val * sn) % 20201227
}

fn calc_loop(public_key: usize) -> usize {
    let mut loop_size = 1;
    let mut val = 1;
    loop {
        val = transform(val, INIT_SN);
        if val == public_key {
            return loop_size;
        } else {
            loop_size += 1;
        }
    }
}