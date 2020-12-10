use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = <isize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        let mut iter = s.splitn(2, " ");
        let cmd = iter.next().expect("Could not extract cmd.");
        let num = iter.next().expect("Could not extract num.").parse()?;
        Ok(match cmd {
            "acc" => Acc(num),
            "jmp" => Jmp(num),
            "nop" => Nop(num),
            _ => {
                eprintln!("Unknown instruction: {}", s);
                Nop(0)
            }
        })
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ProcessRunResult {
    Terminated,
    LoopDetected,
}

#[derive(Debug, Clone, Default)]
pub struct Process {
    pub accumulator: isize,
    pub program_counter: usize,
    program: Vec<Instruction>,
}

impl Process {
    pub fn from_program_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(File::open(filename)?);
        let program = reader
            .lines()
            .flatten()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .expect("Could not parse input file.");
        Ok(Self {
            accumulator: 0,
            program_counter: 0,
            program,
        })
    }

    pub fn run_until_loop(&mut self) -> ProcessRunResult {
        use Instruction::*;
        let mut counts = vec![0; self.program.len()];

        while self.program_counter != self.program.len() {
            let instruction = self.program[self.program_counter];
            if counts[self.program_counter] > 0 {
                eprintln!(
                    "Aborting: I am on an instruction that I have run before: {}: {:?}",
                    self.program_counter, instruction
                );
                return ProcessRunResult::LoopDetected;
            }
            counts[self.program_counter] += 1;

            let mut pc_step = 1;

            match instruction {
                Acc(x) => {
                    self.accumulator += x;
                }
                Jmp(x) => {
                    pc_step = x;
                }
                Nop(_) => {}
            };

            self.program_counter = (self.program_counter as isize + pc_step) as usize;
        }

        ProcessRunResult::Terminated
    }

    pub fn reset_state(&mut self) {
        self.accumulator = 0;
        self.program_counter = 0;
    }

    pub fn run_patched_program(&mut self) {
        use Instruction::*;

        for i in 0..self.program.len() {
            self.program[i] = match self.program[i] {
                Jmp(x) => Nop(x),
                Nop(x) => Jmp(x),
                _ => continue,
            };

            self.reset_state();
            let res = self.run_until_loop();

            self.program[i] = match self.program[i] {
                Jmp(x) => Nop(x),
                Nop(x) => Jmp(x),
                _ => unreachable!(),
            };

            if res == ProcessRunResult::Terminated {
                eprintln!("Found switched instruction at offset {}!", i);
                eprintln!("Acc is {}", self.accumulator);
                break;
            }
        }
    }
}
