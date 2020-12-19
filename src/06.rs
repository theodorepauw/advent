use std::collections::HashSet;
const INPUT: &str = include_str!("../inputs/06.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let sum = INPUT.split("\r\n\r\n").fold(0, |sum, answered| {
        let distinct: HashSet<char> = answered
            .chars()
            .filter(|c| *c != '\r' && *c != '\n')
            .collect::<HashSet<char>>();
        sum + distinct.len()
    });

    println!("Day 6 Part 1: {}", sum);
}

fn part2() {
    // replaced mine with timvisee's lovely solution (and changed some variable names so that I could understand it)
    let sum = INPUT
        .split("\r\n\r\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.bytes().fold(0, |ans, q| ans | (1 << (q - b'a'))))
                .fold(std::u32::MAX, |ans, group| ans & group)
                .count_ones()
        })
        .sum::<u32>();

    println!("Day 6 Part 2: {}", sum);
}
