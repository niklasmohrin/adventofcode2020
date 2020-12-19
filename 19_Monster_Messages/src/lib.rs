use std::collections::HashMap;
use std::error::Error;
use std::num::NonZeroUsize;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleRule {
    Literal(u8),
    Composite(Vec<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub id: usize,
    pub variants: Vec<SingleRule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSet {
    pub rules: HashMap<usize, Rule>,
}

impl RuleSet {
    pub fn matches(&self, msg: &str) -> bool {
        // Small shortcut with the NonZeroUsize creation here, expecting non-empty messages
        self.matches_rule(msg.as_bytes(), 0) == NonZeroUsize::new(msg.len())
    }

    pub fn matches_rule(&self, msg: &[u8], id: usize) -> Option<NonZeroUsize> {
        let rule = self.rules.get(&id);
        if rule.is_none() {
            eprintln!("Tried to get rule out of bounds: {}", id);
            return None;
        }
        let rule = rule.unwrap();

        for variant in &rule.variants {
            let res = self.matches_single_rule(msg, variant);
            if res.is_some() {
                return res;
            }
        }

        None
    }

    pub fn matches_single_rule(&self, msg: &[u8], rule: &SingleRule) -> Option<NonZeroUsize> {
        match rule {
            SingleRule::Literal(c) => NonZeroUsize::new((msg.len() > 0 && msg[0] == *c) as usize),
            SingleRule::Composite(subrules) => {
                let mut matched_up_to = 0;
                for &subrule in subrules {
                    let progress = self.matches_rule(&msg[matched_up_to..], subrule)?;
                    matched_up_to += progress.get();
                }

                NonZeroUsize::new(matched_up_to)
            }
        }
    }

    pub fn matches_recursive(&self, msg: &str) -> bool {
        // 8 -> 42 | 42 8
        // 11 -> 42 31 | 42 11 31
        //
        // Since 0: 8 11
        // The result can be any number of 42-matches up front and any (smaller) number of
        // 31-matches in the back, therefore I can just try out any number of 42 and see if the
        // rest fits into some smaller amount of 31s

        let msg = msg.as_bytes();
        let mut offset = 0;
        // At least two matches are needed
        for _ in 0..2 {
            match self.matches_rule(&msg[offset..], 42) {
                Some(n) => offset += n.get(),
                None => return false,
            }
        }

        let mut num42s = 2;
        loop {
            let mut local_offset = offset;
            // The number of 31s can be at most the number of 42s minus 1 (otherwise the leading
            // rule 8 wouldn't match)
            for _num31s in 1..num42s {
                match self.matches_rule(&msg[local_offset..], 31) {
                    Some(n) => local_offset += n.get(),
                    None => break,
                };
            }

            if local_offset > offset && local_offset == msg.len() {
                return true;
            }

            num42s += 1;
            match self.matches_rule(&msg[offset..], 42) {
                Some(n) => offset += n.get(),
                None => return false,
            }
        }
    }
}

impl FromStr for RuleSet {
    type Err = <Rule as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .lines()
            .map(Rule::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let rules = rules.into_iter().map(|r| (r.id, r)).collect();
        Ok(Self { rules })
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(": ");
        let id = iter.next().ok_or("Couldn't split off id")?.parse()?;
        let variants = iter.next().ok_or("Couldn't split off rule variants")?;

        let variants = variants
            .split(" | ")
            .map(SingleRule::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { id, variants })
    }
}

impl FromStr for SingleRule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 3 && s.as_bytes()[0] == b'"' && s.as_bytes()[2] == b'"' {
            Ok(SingleRule::Literal(s.as_bytes()[1]))
        } else {
            Ok(SingleRule::Composite(
                s.split(" ")
                    .map(usize::from_str)
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
}
