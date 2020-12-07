use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "input".to_owned());
    let rules = fs::read_to_string(filename).unwrap();
    let rules: HashMap<_, _> = rules
        .trim_end()
        .split("\n")
        .map(|s| {
            let mut iter = s.splitn(2, " contain ");
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .map(|(k, v)| {
            (
                k.trim_end_matches(" bag")
                    .trim_end_matches(" bags")
                    .trim_end_matches("."),
                match v {
                    "no other bags." => HashMap::new(),
                    _ => v
                        .trim_end_matches(".")
                        .split(", ")
                        .map(|s| {
                            let mut iter = s.splitn(2, " ");
                            let count = iter.next().unwrap().parse::<usize>().unwrap();
                            let color = iter
                                .next()
                                .unwrap()
                                .trim_end_matches(" bag")
                                .trim_end_matches(" bags");
                            (color, count)
                        })
                        .collect(),
                },
            )
        })
        .collect();

    part_one(&rules);
    part_two(&rules);
}

fn part_one<'s>(rules: &HashMap<&'s str, HashMap<&'s str, usize>>) {
    let mut can_contain_gold = HashMap::<&str, bool>::new();

    fn check<'a>(
        color: &'a str,
        rules: &HashMap<&'a str, HashMap<&'a str, usize>>,
        cache: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if let Some(x) = cache.get(color) {
            return *x;
        }

        let mut sub_colors = rules.get(color).unwrap().keys();
        let x = sub_colors.any(|c| check(c, rules, cache));
        cache.insert(color, x);
        x
    }

    can_contain_gold.insert("shiny gold", true);
    for color in rules.keys().copied() {
        let _ = check(color, rules, &mut can_contain_gold);
    }

    eprintln!(
        "{} colors of bags can contain a shiny golden bag.",
        can_contain_gold.values().filter(|&&b| b).count() - 1
    );
}

fn part_two<'s>(rules: &HashMap<&'s str, HashMap<&'s str, usize>>) {
    let mut contains_bags = HashMap::<&str, usize>::new();

    fn count_bags<'a>(
        color: &'a str,
        rules: &HashMap<&'a str, HashMap<&'a str, usize>>,
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if let Some(n) = cache.get(color) {
            return *n;
        }

        let sub_counts = rules.get(color).unwrap().iter();
        let sum = sub_counts
            .map(|(cl, ct)| ct * count_bags(cl, rules, cache))
            .sum::<usize>()
            + 1;
        cache.insert(color, sum);
        sum
    }

    eprintln!(
        "A shiny gold bag contains {} bags.",
        count_bags("shiny gold", rules, &mut contains_bags) - 1
    );
}
