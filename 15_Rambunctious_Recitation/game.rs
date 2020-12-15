use std::collections::HashMap;
use std::env;
use std::num::NonZeroUsize;
use std::str::FromStr;

const MY_INPUT: &'static str = "14,1,17,0,3,20";
const TURN_IN_QUESTION: usize = 30000000;
// const TURN_IN_QUESTION: usize = 2020;

fn main() {
    let starting_numbers = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| MY_INPUT.to_owned())
        .split(',')
        .map(usize::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Couldn't parse input");
    let mut last_number = 0;
    let mut last_spoken = HashMap::<usize, (Option<NonZeroUsize>, NonZeroUsize)>::new();

    for turn in 1..=TURN_IN_QUESTION {
        let number_spoken: usize = starting_numbers.get(turn - 1).copied().unwrap_or_else(|| {
            if let Some((Some(before_last), last)) = last_spoken.get(&last_number) {
                last.get() - before_last.get()
            } else {
                0
            }
        });

        last_spoken
            .entry(number_spoken)
            .and_modify(|entry| {
                *entry = (Some(entry.1), NonZeroUsize::new(turn).unwrap());
            })
            .or_insert((None, NonZeroUsize::new(turn).unwrap()));

        last_number = number_spoken;
    }

    println!("{}", last_number);
}
