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
    println!("Day 19 Part 1: {}", p1);
    let rule_31 = rules.get("31").ok_or("no rule 31")?.1.clone();
    let rule_42 = rules.get("42").ok_or("no rule 42")?.1.clone();

    rules.get_mut("8").ok_or("no rule 8")?.1.push('+');
    let rule_11 = rules.get_mut("11").ok_or("no rule 11")?;
    rule_11.1 = format!("(({})+({})*({})+)", rule_42, rule_11.1, rule_31);
    println!("{}", rules.get("11").unwrap().1);

    let rule_0 = Regex::new(&format!("^{}$", simplify("0", &mut rules)))?;
    let p2 = messages.lines().filter(|m| rule_0.is_match(&m)).count();
    println!("Day 19 Part 2: {}", p2);
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

        let (_, simplification) = rules.get_mut(id).expect("rule disappeared");
        *simplification = result;
    }

    rules.get(id).expect("no rule simplification").1.clone()
}
