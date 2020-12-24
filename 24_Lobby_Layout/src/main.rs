#![feature(pattern)]
#![feature(array_methods)]
#![feature(split_inclusive)]

use derive_more::{Add, AddAssign, Sum};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;

#[derive(Add, AddAssign, Sum, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy, Debug)]
struct Vector(isize, isize);

impl Vector {
    fn from_hexagon_str(s: &str) -> Self {
        s.split_inclusive(|c| c == 'e' || c == 'w')
            .map(Vector::from)
            .sum()
    }

    fn from(s: &str) -> Self {
        match s {
            "e" => Vector(1, 0),
            "w" => Vector(-1, 0),
            "ne" => Vector(0, 1),
            "sw" => Vector(0, -1),
            "nw" => Vector(-1, 1),
            "se" => Vector(1, -1),
            x => panic!("Cannot parse: {}", x),
        }
    }

    fn neighbor_directions() -> [Self; 6] {
        [
            Vector(1, 0),
            Vector(-1, 0),
            Vector(0, 1),
            Vector(0, -1),
            Vector(-1, 1),
            Vector(1, -1),
        ]
    }

    fn neighbors(self) -> [Self; 6] {
        let mut res = Self::neighbor_directions();
        res.iter_mut().for_each(|n| {
            *n += self;
        });
        res
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let points: Vec<_> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(Vector::from_hexagon_str)
        .collect();
    let mut black_tiles: BTreeSet<Vector> = BTreeSet::new();
    for p in &points {
        if black_tiles.take(p).is_none() {
            black_tiles.insert(*p);
        }
    }

    println!("Initial number of black tiles: {}", black_tiles.len());

    for _ in 0..100 {
        let to_remove: Vec<Vector> = black_tiles
            .iter()
            .copied()
            .filter(|p| {
                !matches!(
                    p.neighbors()
                        .iter()
                        .filter(|n| black_tiles.contains(n))
                        .count(),
                    1 | 2
                )
            })
            .collect();
        let mut all_neighbors = BTreeMap::new();
        for &t in black_tiles.iter() {
            for &n in t.neighbors().iter() {
                *all_neighbors.entry(n).or_insert(0usize) += 1;
            }
        }
        let to_add: Vec<Vector> = all_neighbors
            .into_iter()
            .filter(|(_, c)| *c == 2)
            .map(|(n, _)| n)
            .collect();

        for p in to_remove {
            black_tiles.remove(&p);
        }
        for p in to_add {
            black_tiles.insert(p);
        }
    }

    println!("Final number of black tiles: {}", black_tiles.len());
}
