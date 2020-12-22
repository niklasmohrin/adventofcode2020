#![feature(str_split_once)]
#![feature(deque_range)]

use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::ptr;
use std::str::FromStr;

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "input".to_owned());
    let content = fs::read_to_string(filename).expect("Couldn't read file");
    let (player1, player2) = content.split_once("\n\n").unwrap();

    let mut player1 = player1
        .lines()
        .skip(1)
        .map(usize::from_str)
        .collect::<Result<VecDeque<_>, _>>()
        .expect("Couldn't parse cards");
    let mut player2 = player2
        .lines()
        .skip(1)
        .map(usize::from_str)
        .collect::<Result<VecDeque<_>, _>>()
        .expect("Couldn't parse cards");

    // Part 1
    // let winner = normal_combat(&mut player1, &mut player2);

    // Part 2
    let winner = recursive_combat(&mut player1, &mut player2);

    let score: usize = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum();
    println!("winning score is {}", score);
}

#[allow(dead_code)]
fn normal_combat<'p>(
    player1: &'p mut VecDeque<usize>,
    player2: &'p mut VecDeque<usize>,
) -> &'p mut VecDeque<usize> {
    loop {
        if player2.is_empty() {
            return player1;
        } else if player1.is_empty() {
            return player2;
        } else {
            let p1_card = player1.pop_front().unwrap();
            let p2_card = player2.pop_front().unwrap();

            if p1_card > p2_card {
                player1.push_back(p1_card);
                player1.push_back(p2_card);
            } else {
                player2.push_back(p2_card);
                player2.push_back(p1_card);
            }
        }
    }
}

#[allow(dead_code)]
fn recursive_combat<'p>(
    player1: &'p mut VecDeque<usize>,
    player2: &'p mut VecDeque<usize>,
) -> &'p mut VecDeque<usize> {
    let mut prev_rounds: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

    loop {
        if player1.is_empty() {
            return player2;
        } else if player2.is_empty() {
            return player1;
        }

        // Infinite game prevention rule
        let round_id = (player1.clone(), player2.clone());
        if prev_rounds.contains(&round_id) {
            return player1;
        }
        prev_rounds.insert(round_id);

        // Unwrap is okay, because length was checked above
        let p1_card = player1.pop_front().unwrap();
        let p2_card = player2.pop_front().unwrap();

        let player1_wins = if player1.len() >= p1_card && player2.len() >= p2_card {
            // Copy the next x cards from the deck for the sub round, where x is the value of the
            // card drawn in this round by that player
            let mut p1_sub_deck = player1.range(..p1_card).copied().collect();
            let mut p2_sub_deck = player2.range(..p2_card).copied().collect();

            // Recurse
            let sub_winner = &*recursive_combat(&mut p1_sub_deck, &mut p2_sub_deck);

            // Compare references instead of values, to check which player won
            ptr::eq(sub_winner, &p1_sub_deck)
        } else {
            // Cannot recurse, bigger card wins
            p1_card > p2_card
        };

        if player1_wins {
            player1.push_back(p1_card);
            player1.push_back(p2_card);
        } else {
            player2.push_back(p2_card);
            player2.push_back(p1_card);
        }
    }
}
