use std::io::{self, Write};
const INPUT: &str = include_str!("./inputs/04.txt");
const REQ_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn solve() -> crate::util::Result<()> {
    let year_valid = |year: &str, min: &str, max: &str| -> bool {
        year.len() == 4 && year >= min && year <= max
    };
    let height_valid = |height: &str| -> bool {
        let unit = &height[height.len() - 2..];
        let h = &height[..height.len() - 2];
        (unit == "cm" && ("150"..="193").contains(&h))
            || (unit == "in" && ("59"..="76").contains(&h))
    };
    let eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let hair_color_valid = |code: &str| matches!(code.strip_prefix('#'), Some(clr) if clr.len() == 6 && i64::from_str_radix(clr, 16).is_ok());

    let (p1, p2) = INPUT.split("\n\n").fold((0, 0), |(p1, p2), creds| {
        let (a, b) = creds
            .split_whitespace()
            .map(|s| (&s[0..3], &s[4..]))
            .map(|(field, value)| match field {
                "byr" => (true, year_valid(value, "1920", "2002")),
                "iyr" => (true, year_valid(value, "2010", "2020")),
                "eyr" => (true, year_valid(value, "2020", "2030")),
                "hgt" => (true, value.len() > 3 && height_valid(value)),
                "hcl" => (true, hair_color_valid(value)),
                "ecl" => (true, eye_colors.contains(&value)),
                "pid" => (true, value.len() == 9 && value.parse::<i64>().is_ok()),
                _ => (false, false),
            })
            .fold((0, 0), |(p1_valid, p2_valid), (a, b)| {
                (p1_valid + usize::from(a), p2_valid + usize::from(b))
            });

        (
            p1 + i32::from(a == REQ_FIELDS.len()),
            p2 + i32::from(b == REQ_FIELDS.len()),
        )
    });

    writeln!(io::stdout(), "Day 04 Part 1: {}\nDay 04 Part 2: {}", p1, p2)?;
    Ok(())
}
