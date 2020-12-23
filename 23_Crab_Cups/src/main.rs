use std::collections::VecDeque;
use std::env;
use std::iter;
use std::str::FromStr;

const MY_INPUT: &str = "135468729";

/// Stupid simulation with placing labels in ring buffer
fn part1() {
    let mut labels = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(MY_INPUT))
        .split_terminator("")
        .skip(1)
        .map(usize::from_str)
        .collect::<Result<VecDeque<_>, _>>()
        .expect("Couldn't parse input");

    for _ in 0..100 {
        let picked_up: Vec<_> = labels.drain(1..=3).collect();

        let mut wanted = ((labels[0] as isize - 1 - 1).rem_euclid(9) + 1) as usize;
        while picked_up.contains(&wanted) {
            wanted = ((wanted as isize - 1 - 1).rem_euclid(9) + 1) as usize;
        }
        let insert_index = labels.iter().position(|&el| el == wanted).unwrap() + 1;
        for x in picked_up.into_iter().rev() {
            labels.insert(insert_index, x);
        }

        labels.rotate_left(1);
    }

    labels.rotate_left(labels.iter().position(|&el| el == 1).unwrap());
    let res = labels
        .iter()
        .skip(1)
        .fold(String::from(""), |acc, cur| format!("{}{}", acc, cur));
    println!("End arrangement: {}", res);
}

/// Friendship with stupid part 1 solution is over,
/// fast "linked list" is my best friend now.
fn part2() {
    const LEN: usize = 1_000_000;

    let starting_labels = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(MY_INPUT))
        .split_terminator("")
        .skip(1)
        .map(usize::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Couldn't parse input");

    // all labels (0..LEN) are in a circular linked list, but instead of having an actual linked
    // list, this vector just has the next value of label i at next[i]
    // Note: labels are all offset by one to have indexing not be so painful
    let mut next: Vec<usize> = (0..LEN).map(|x| (x + 1).rem_euclid(LEN)).collect();

    let mut prev = LEN - 1;
    starting_labels
        .iter()
        .copied()
        .map(|x| x - 1)
        .chain(iter::once(starting_labels.len()))
        .for_each(|n| {
            next[prev] = n;
            prev = n;
        });

    let mut current = starting_labels[0] - 1;
    for _ in 0..10_000_000 {
        let p1 = next[current];
        let p2 = next[p1];
        let p3 = next[p2];
        next[current] = next[p3];

        let mut wanted = current.checked_sub(1).unwrap_or(LEN - 1);
        while [p1, p2, p3].contains(&wanted) {
            wanted = wanted.checked_sub(1).unwrap_or(LEN - 1);
        }

        next[p3] = next[wanted];
        next[wanted] = p1;

        current = next[current];
    }

    // Remember: values in the vector are all one lower than the actual label, so add one to both
    let f1 = next[0];
    let f2 = next[f1];
    println!(
        "Product of two numbers after 1 is {}",
        dbg!(f1 + 1) * dbg!(f2 + 1)
    );
}

fn main() {
    part1();
    part2();
}
