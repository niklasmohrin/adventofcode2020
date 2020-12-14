use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaskBit {
    Set,
    Unset,
    Leave,
}

impl TryFrom<char> for MaskBit {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(MaskBit::Set),
            '0' => Ok(MaskBit::Unset),
            'X' => Ok(MaskBit::Leave),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Reg(u64);
pub const REG_SIZE: usize = 36;

impl Reg {
    pub fn new(val: u64) -> Self {
        let mut this: Self = Default::default();
        this.set(val);
        this
    }

    fn size_mask() -> u64 {
        (0..REG_SIZE).fold(0, |acc, _| (acc << 1) | 1)
    }

    pub fn set(&mut self, val: u64) {
        debug_assert_eq!(val & !Self::size_mask(), 0);
        self.0 = val & Self::size_mask();
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    SetMask([MaskBit; REG_SIZE]),
    SetMemory { address: usize, value: u64 },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" = ");
        let left = iter.next().ok_or(())?;
        let right = iter.next().ok_or(())?;

        if left == "mask" {
            let val = right
                .chars()
                .map(MaskBit::try_from)
                .collect::<Result<Vec<_>, _>>()?;
            let val: [MaskBit; REG_SIZE] = val.as_slice().try_into().map_err(|_| ())?;

            Ok(Instruction::SetMask(val))
        } else if &left[0..3] == "mem" {
            if left.as_bytes()[3] == b'[' && left.as_bytes()[left.len() - 1] == b']' {
                let address: usize = left[4..left.len() - 1].parse().map_err(|_| ())?;
                let value: u64 = right.parse().map_err(|_| ())?;
                Ok(Instruction::SetMemory { address, value })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitializationProgram {
    instructions: Vec<Instruction>,
    mask: [MaskBit; REG_SIZE],
}

impl InitializationProgram {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            mask: [MaskBit::Leave; REG_SIZE],
        }
    }

    pub fn run_v1(&mut self) -> Vec<Reg> {
        let mut memory = Vec::<Reg>::new();
        for instruction in self.instructions.iter().copied() {
            match instruction {
                Instruction::SetMask(val) => self.mask = val,
                Instruction::SetMemory { address, value } => {
                    let value = self.apply_mask_to(value);
                    if address >= memory.len() {
                        memory.resize(address + 1, Default::default());
                    }
                    memory[address].set(value);
                }
            }
        }
        memory
    }

    pub fn run_v2(&mut self) -> HashMap<u64, u64> {
        let mut memory = HashMap::new();

        for instruction in self.instructions.iter().copied() {
            match instruction {
                Instruction::SetMask(val) => self.mask = val,
                Instruction::SetMemory { address, value } => {
                    let addresses = self.addresses_from_mask(address);

                    for actual_address in &addresses {
                        let _ = memory.insert(*actual_address, value);
                    }
                }
            }
        }

        memory
    }

    fn addresses_from_mask(&self, original_address: usize) -> Vec<u64> {
        let mut addresses = vec![original_address as u64 | self.bits_of(MaskBit::Set)];
        for i in 0..REG_SIZE {
            // Leave now means "Floating"
            if self.mask[i] == MaskBit::Leave {
                let mut new_addresses = Vec::new();
                for a in addresses {
                    let bit = 1 << (REG_SIZE - i - 1);
                    new_addresses.push(a & !bit);
                    new_addresses.push(a | bit);
                }

                addresses = new_addresses;
            }
        }

        addresses
    }

    fn bits_of(&self, target: MaskBit) -> u64 {
        self.mask
            .iter()
            .fold(0, |acc, &cur| (acc << 1) | ((cur == target) as u64))
    }

    fn apply_mask_to(&self, value: u64) -> u64 {
        let set_mask = self.bits_of(MaskBit::Set);
        let unset_mask = self.bits_of(MaskBit::Unset);

        (value | set_mask) & !unset_mask
    }
}

impl FromStr for InitializationProgram {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self::new();
        this.instructions = s
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(this)
    }
}
