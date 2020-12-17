use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Rule {
    name: String,
    ranges: [RangeInclusive<usize>; 2],
}

impl Rule {
    pub fn check(&self, num: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&num))
    }
}

impl TryFrom<String> for Rule {
    type Error = Box<dyn Error>;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let split_index = line.find(": ").ok_or("split index not found")?;
        let both_rules = &line[split_index + 2..];

        let mut iter = both_rules.split(" or ");
        let first = iter.next().ok_or("first rule not found")?;
        let second = iter.next().ok_or("second rule not found")?;

        let parse_rule = |s: &str| -> Result<_, Box<dyn Error>> {
            let mut iter = s.split("-");
            let start = iter.next().ok_or("start not found")?.parse::<usize>()?;
            let end = iter.next().ok_or("end not found")?.parse::<usize>()?;
            Ok(start..=end)
        };

        let ranges = [parse_rule(first)?, parse_rule(second)?];
        let mut name = line;
        name.truncate(split_index);

        Ok(Self { name, ranges })
    }
}

#[derive(Debug, Clone)]
pub struct TicketTranslator {
    rules: Vec<Rule>,
    own_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
}

impl TicketTranslator {
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut lines = BufReader::new(File::open(filename)?).lines();
        eprintln!("Opened file");

        let mut rules = Vec::new();
        while let Some(Ok(line)) = lines.next() {
            if line == "your ticket:" {
                break;
            } else if !line.is_empty() {
                rules.push(Rule::try_from(line)?);
            }
        }

        eprintln!("{} rules read", rules.len());

        let own_ticket = lines
            .next()
            .ok_or("own ticket not found")??
            .split(",")
            .map(usize::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        eprintln!("Own ticket scanned");

        // Skip empty and header line
        let _ = lines.next();
        let _ = lines.next();

        let mut other_tickets = Vec::new();
        while let Some(Ok(line)) = lines.next() {
            let ticket = line
                .split(",")
                .map(usize::from_str)
                .collect::<Result<Vec<_>, _>>()?;
            other_tickets.push(ticket);
        }

        eprintln!("{} other tickets scanned", other_tickets.len());

        Ok(Self {
            rules,
            own_ticket,
            other_tickets,
        })
    }

    pub fn error_rate(&self) -> usize {
        self.other_tickets
            .iter()
            .map(|ticket| self.find_invalid_num(&ticket))
            .flatten()
            .sum()
    }

    fn find_invalid_num(&self, ticket: &[usize]) -> Option<usize> {
        ticket
            .iter()
            .copied()
            .find(|num| !self.rules.iter().any(|rule| rule.check(*num)))
    }

    fn ticket_is_valid(&self, ticket: &[usize]) -> bool {
        self.find_invalid_num(ticket).is_none()
    }

    pub fn translate<'s>(&'s self) -> Vec<&'s Rule> {
        let valid_tickets: Vec<&[usize]> = self
            .other_tickets
            .iter()
            .filter(|ticket| self.ticket_is_valid(&ticket))
            .map(Vec::as_slice)
            .collect();

        let mut possible_columns: Vec<HashSet<usize>> = vec![HashSet::new(); self.rules.len()];

        for (i, rule) in self.rules.iter().enumerate() {
            for cur_index in 0..self.own_ticket.len() {
                let all_tickets_match = valid_tickets
                    .iter()
                    .all(|ticket| rule.check(ticket[cur_index]));
                if all_tickets_match {
                    possible_columns[i].insert(cur_index);
                }
            }
        }

        let mut progress = true;
        let mut found_column: Vec<Option<usize>> = vec![None; self.rules.len()];
        while progress {
            progress = false;
            for i in 0..possible_columns.len() {
                if possible_columns[i].len() == 1 {
                    progress = true;
                    let field = possible_columns[i].drain().next().unwrap();
                    found_column[i] = Some(field);
                    for set in possible_columns.iter_mut() {
                        set.remove(&field);
                    }
                }
            }
        }

        let mut rule_of_column = vec![None; self.own_ticket.len()];

        found_column
            .into_iter()
            .map(Option::unwrap)
            .enumerate()
            .for_each(|(rule_index, column_index)| {
                rule_of_column[column_index] = Some(rule_index);
            });

        rule_of_column
            .into_iter()
            .map(Option::unwrap)
            .map(|i| &self.rules[i])
            .collect()
    }

    pub fn departure_numbers(&self) -> Vec<usize> {
        let rule_of_column = self.translate();

        self.own_ticket
            .iter()
            .enumerate()
            .filter(|(i, _)| rule_of_column[*i].name.starts_with("departure"))
            .map(|(_, v)| *v)
            .collect()
    }
}
