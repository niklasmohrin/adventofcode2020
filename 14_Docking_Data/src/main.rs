use docking_data::{InitializationProgram, Reg};
use std::str::FromStr;

fn main() {
    let mut program =
        InitializationProgram::from_str(include_str!("../input")).expect("Couldn't parse input");

    // Part one
    let memory = program.run_v1();
    println!(
        "Part 1 - Sum of memory: {}",
        memory.iter().map(Reg::get).sum::<u64>()
    );

    // Part two
    let memory = program.run_v2();
    println!("Part 2 - Sum of memory: {}", memory.values().sum::<u64>());
}
