use regex::Regex;

const INPUT: &str = include_str!("../inputs/02.txt");

struct Line<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
    let passwords: Vec<Line> = INPUT
        .trim()
        .split("\n")
        .map(|line| {
            let m = re.captures(line).unwrap();
            Line {
                min: m[1].parse().expect("Couldn't parse minimum (1st integer)."),
                max: m[2].parse().expect("Couldn't parse maximum (2nd integer)."),
                letter: m[3].chars().next().unwrap(),
                password: m.get(4).unwrap().as_str(),
            }
        }).collect();
        part1(&passwords);
        part2(&passwords);
}

fn part1(passwords: &[Line]) {
    let correct = passwords.iter().filter(|line| {
        let count = line.password.chars().filter(|c| *c == line.letter).count();
        line.min <= count && count <= line.max
    })
    .count();
    println!("Day 2 Part 1: {}", correct);
    
}

fn part2(passwords: &[Line]) {
    let correct = passwords.iter().filter(|line| {
        let x = line.password.chars().nth(line.min-1).unwrap();
        let y = line.password.chars().nth(line.max - 1).unwrap();
        (x==line.letter) ^ (y==line.letter)
    }).count();
    println!("Day 2 Part 2: {}", correct);
}