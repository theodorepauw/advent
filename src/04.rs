use std::i64;

const INPUT: &str = include_str!("../inputs/04.txt");
const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let mut valid = 0;
    let year_valid = |year: &str, min: &str, max: &str| -> bool { year.len()==4 && year >= min && year <= max };
    let height_valid = |height: &str| -> bool { 
        let unit = &height[height.len()-2..];
        let h = &height[..height.len()-2];
        (unit =="cm" && h >= "150" && h <= "193") || (unit == "in" && h >= "59" && h <= "76")
    };
    let eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let hair_color_valid = |code: &str| -> bool { match code.strip_prefix("#") {
        Some(clr) if clr.len()==6 && i64::from_str_radix(clr, 16).is_ok() => true,
        _ => false,
        }
    };
    for credentials in INPUT.split("\r\n\r\n") {
        let mut fields = 0;
        for mut key_value_pair in credentials.split_whitespace().map(|s| s.split(':')) {
            let key = key_value_pair.next().unwrap();
            let value = key_value_pair.next().unwrap();
            
            fields += match key {
                "byr" if year_valid(value, "1920", "2002") => 1,
                "iyr" if year_valid(value, "2010", "2020") => 1,
                "eyr" if year_valid(value, "2020", "2030") => 1,
                "hgt" if value.len()>3 && height_valid(value) => 1,
                "hcl" if hair_color_valid(value) => 1,
                "ecl" if eye_colors.contains(&value) => 1,
                "pid" if value.len()==9 && i64::from_str_radix(value, 10).is_ok() => 1,                
                _ => 0,
            };
        }

        if fields == REQ_FIELDS.len() { valid += 1 };        
    }
    
    println!("Day 4 Part 2: {}", valid);
}