use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/09.txt");
fn main() {
    let data: Vec<usize> = INPUT
        .lines()
        .map(|s: &str| s.parse::<usize>().expect("Couldn't parse number!"))
        .collect();

    let invalid_number = part1(&data).expect("No answer for Part 1 found");
    println!("Day 9 Part 1: {}", invalid_number);
    let encryption_weakness = part2(&data, invalid_number);
    println!("Day 9 Part 2: {}", encryption_weakness);    
}

fn part1(data: &[usize]) -> Option<usize> {
    let mut i = 0;
    let mut sums: Vec<(usize, HashSet<usize>)> = vec![];
    for d in data {
        if i > 24 && !sums[i - 25..i].iter().any(|(_, s)| s.contains(&d)) {
            return Some(*d);
        }
        sums.push((*d, HashSet::new()));

        for j in i..i + 25 {
            if j >= data.len()-1 {
                break;
            }
            if data[i] != data[j] {
                sums[i].1.insert(data[i] + data[j]);
            }
        }

        i += 1;
    };
    None
}

fn part2(data: &[usize], invalid_number: usize) -> usize {
    let (mut x1, mut x2) = (0, 1); // 2 cursors
    let mut contiguous_sum = data[x1] + data[x2]; // must consist of at least 2 numbers

    while contiguous_sum < invalid_number {
        x2 += 1;
        contiguous_sum += data[x2];
        while contiguous_sum > invalid_number {
            contiguous_sum -= data[x1];
            x1 += 1;
        }
    }

    data[x1..x2+1].iter().min().expect("no minimum!") + data[x1..x2+1].iter().max().expect("no maximum!")
}