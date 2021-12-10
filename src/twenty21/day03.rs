const INPUT: &str = include_str!("./inputs/03.txt");
use std::{io::{self, Write}, usize};

pub fn solve() -> crate::util::Result<()> {

    let numbers: Vec<usize> = INPUT.lines().map(|s| usize::from_str_radix(s, 2).expect("not a binary number")).collect();
    let binary_length: usize = INPUT.find('\n').expect("invalid input: no lines");

    let most_common_bit = | list: &[usize], power: usize | {
        let ones: usize = list.iter().map(|&n| (n>>power)&1).sum();
        if ones + 1 > (list.len()+1>>1) { 1 } else { 0usize }
    };

    let (gamma, epsilon) = (0..binary_length).rev().fold((0, 0), |(g, e), i| {
        let bit = most_common_bit(&numbers, i);
        (g | bit<<i, e | (1-bit)<<i)
    });


    let p1 = gamma * epsilon;

    let mut oxygen_generator_rating = numbers.clone();
    let mut co2_scrubber_rating = numbers.clone();
    let mut power = binary_length;

    while oxygen_generator_rating.len() > 1 {
        power = power.checked_sub(1).expect("no more options for ogr");
        let bit = most_common_bit(&oxygen_generator_rating, power);
        oxygen_generator_rating.retain(|&number| (number>>power)&1==bit);
    }

    let mut power = binary_length;
    while co2_scrubber_rating.len() > 1 {
        power = power.checked_sub(1).expect("no more options for ogr");
        let bit = 1^most_common_bit(&co2_scrubber_rating, power);
        co2_scrubber_rating.retain(|&number| (number>>power)&1 == bit);
    }

    let p2 = oxygen_generator_rating[0] * co2_scrubber_rating[0];

    writeln!(io::stdout(), "Day 03 Part 1: {}\nDay 03 Part 2: {}", p1, p2)?;
    Ok(())
}