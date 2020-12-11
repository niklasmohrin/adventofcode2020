use seating_system::{SeatingArea, Visibility};
use std::env;
use std::fs;

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "input".to_owned());
    let s = fs::read_to_string(filename).expect("Coulnd't read file.");
    let mut area: SeatingArea = s.parse().expect("Couldn't parse input.");

    // Part one
    // let visibility = Visibility::Adjacent;
    // Part two
    let visibility = Visibility::Far;

    let rounds = area.stabilize(visibility);
    println!(
        "{} seats are occupied after stabilizing which took {} rounds",
        area.occupied(),
        rounds
    );
}
