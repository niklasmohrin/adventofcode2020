use monster_messages::RuleSet;
use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| String::from("input"));
    let content = fs::read_to_string(filename).expect("Couldn't read file");

    let mut iter = content.split("\n\n");
    let rules = iter.next().expect("Couldn't extract rules");
    let messages: Vec<_> = iter
        .next()
        .expect("Couldn't extract messages")
        .lines()
        .collect();

    let rule_set = RuleSet::from_str(rules).expect("Couldn't parse rules");

    // Part one
    let valid_messages = messages.iter().filter(|msg| rule_set.matches(msg)).count();
    println!("Valid messages without recursion mod: {}", valid_messages);

    // Part two
    let valid_messages = messages
        .iter()
        .filter(|msg| rule_set.matches_recursive(msg))
        .count();
    println!("Valid messages with    recursion mod: {}", valid_messages);
}
