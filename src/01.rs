const INPUT: &str = include_str!("../inputs/01.txt");

fn main() {
    let mut numbers: Vec<i64> = INPUT
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    numbers.sort();
    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &[i64]) {
    let sol = numbers.iter().enumerate().find_map(|(i, n)| {
        numbers[i..]
            .binary_search(&(2020 - n))
            .and_then(|j| Ok(n * numbers[i + j]))
            .ok()
    });

    println!("Day 1 Part 1: {}", sol.expect("part 1: no solution found"));
}

fn part2(numbers: &[i64]) {
    let sol = numbers.iter().enumerate().find_map(|(i, n)| {
        numbers[i..].iter().enumerate().find_map(|(j, m)| {
            numbers[i + j..]
                .binary_search(&(2020 - n - m))
                .and_then(|k| Ok(n * m * numbers[i + j + k]))
                .ok()
        })
    });

    println!("Day 1 Part 2: {}", sol.expect("part 2: no solution found"));
}
