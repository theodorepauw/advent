use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/19.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = INPUT.splitn(2, "\n\n");
    let mut rules: HashMap<&str, (String, String)> = HashMap::new();
    let (ruleset, messages) = (
        input.next().ok_or("no rules")?,
        input.next().ok_or("no messages")?,
    );
    for r in ruleset.lines() {
        let mut split = r.splitn(2, ": ");
        let id = split.next().ok_or("no : in rule")?;
        let text = split.next().ok_or("no text after :")?;
        rules.insert(id, (text.replace("\"", ""), String::new()));
    }

    let rule_0 = Regex::new(&format!("^{}$", simplify("0", &mut rules)))?;
    let p1 = messages.lines().filter(|m| rule_0.is_match(&m)).count();
    rules.get_mut("0").ok_or("no rule 0")?.1 = String::new();
    let rule_31 = rules.get("31").ok_or("no rule 31")?.1.clone();
    let rule_42 = rules.get("42").ok_or("no rule 42")?.1.clone();
    rules.get_mut("8").ok_or("no rule 0")?.1 = format!("{}+", rule_42);
    rules.get_mut("11").ok_or("no rule 0")?.1 = format!("{}+(?P<thirtyone>{}+)", rule_42, rule_31);

    let rule_0 = Regex::new(&format!("^{}$", simplify("0", &mut rules)))?;
    let p2 = messages
        .lines()
        .filter(|m| {
            rule_0
                .captures(&m)
                .and_then(|c| c.name("thirtyone"))
                .map_or(0, |x| x.start())
                << 1
                > m.len() // 42 must take up the bulk of m
        })
        .count();

    println!("Day 19 Part 1: {}\nDay 19 Part 2: {}", p1, p2);
    Ok(())
}

fn simplify(id: &str, rules: &mut HashMap<&str, (String, String)>) -> String {
    if rules.get(id).expect("no rule for id").1.len() == 0 {
        let (text, _) = rules.get(id).expect("no rule for id").clone();
        let mut result = String::new();
        if text == "a" || text == "b" {
            result.push_str(&text);
        } else {
            result.push('(');
            for x in text.split(" | ") {
                for t in x.split_whitespace().filter(|&f| f != "a" && f != "b") {
                    result.push_str(&simplify(t, rules));
                }
                result.push('|');
            }
            result.pop();
            result.push(')');
        }
        rules.get_mut(id).expect("rule disappeared").1 = result;
    }
    rules.get(id).expect("no rule simplification").1.clone()
}