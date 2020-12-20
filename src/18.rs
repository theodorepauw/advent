const INPUT: &str = include_str!("../inputs/18.txt");

enum Version {
    One,
    Two,
}

fn main() {
    let (p1, p2) = INPUT
        .lines()
        .map(|s| (group(s, &Version::One), group(s, &Version::Two)))
        .fold((0, 0), |(p1, p2), (l1, l2)| (p1 + l1, p2 + l2));
    println!("Day 18 Part 1: {}", p1);
    println!("Day 18 Part 2: {}", p2);
}

fn group(s: &str, v: &Version) -> u64 {
    if !s.contains("(") {
        return match v {
            Version::One => solve_v1(s),
            Version::Two => solve_v2(s),
        };
    }

    let mut stack = vec![];
    let mut result = vec![];
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            result.push((stack.pop().expect(") unmatched"), i));
        }
    }
    let (start, end) = result.get(0).expect("no op");
    let expression = format!("{}", group(&s[start + 1..*end], v));
    let simplified = &[&s[..*start], &expression, &s[end + 1..]].join("");
    group(&simplified, v)
}

fn solve_v1(s: &str) -> u64 {
    s.split_whitespace()
        .fold((0, '+'), |(result, operator), s| {
            match (s.chars().next().unwrap(), operator) {
                (' ', _) => (result, operator),
                ('+', _) => (result, '+'),
                ('*', _) => (result, '*'),
                (_, '+') => (result + s.parse::<u64>().unwrap(), operator),
                (_, '*') => (result * s.parse::<u64>().unwrap(), operator),
                _ => panic!(format!("unrecognised str: {}!", s)),
            }
        })
        .0
}

fn solve_v2(s: &str) -> u64 {
    s.split('*').map(|expr| solve_v1(expr)).product()
}
