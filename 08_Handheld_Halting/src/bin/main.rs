use aocasm::{Process, ProcessRunResult};
use std::env;

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "input".to_owned());
    let mut process = Process::from_program_file(&filename).expect("Couldn't open input.");

    // Part one
    // let res = process.run_until_loop();
    // assert_eq!(res, ProcessRunResult::LoopDetected);
    // println!("After loop detection, acc is {}", process.accumulator);
    // eprintln!("Process: {:?}", process);

    // Part two
    process.run_patched_program();
}
