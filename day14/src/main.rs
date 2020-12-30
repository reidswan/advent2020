use common::load_vec;
use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input: Vec<Command> = load_vec("input/day14.txt");
    part1(&input);
    part2(&input);
}

fn run_with_masking_type(input: &[Command], masking_type: MaskingType) -> u128 {
    let mut machine = Machine::default();

    for command in input.iter() {
        machine.run_command(command, masking_type);
    }

    machine.sum_memory()
}

fn part1(input: &[Command]) {
    println!(
        "Part 1: {}",
        run_with_masking_type(input, MaskingType::Value)
    );
}

fn part2(input: &[Command]) {
    println!(
        "Part 2: {}",
        run_with_masking_type(input, MaskingType::Memory)
    );
}

#[derive(Copy, Clone)]
enum MaskingType {
    Value,
    Memory,
}

#[derive(Clone, Debug, Default)]
struct Machine {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl Machine {
    fn run_command(&mut self, command: &Command, masking_type: MaskingType) {
        match command {
            Command::SetMask(mask) => self.set_mask(mask.clone()),
            Command::SetMem { location, value } => self.set_mem(*location, *value, masking_type),
        }
    }

    fn set_mask(&mut self, mask: Mask) {
        self.mask = mask
    }

    fn set_mem(&mut self, location: u64, value: u64, masking_type: MaskingType) {
        match masking_type {
            MaskingType::Value => {
                self.memory.insert(location, self.mask.mask(value));
            }
            MaskingType::Memory => {
                for location in self.mask.masked_set(location) {
                    self.memory.insert(location, value);
                }
            }
        };
    }

    fn sum_memory(&self) -> u128 {
        self.memory.values().map(|&i| i as u128).sum()
    }
}

#[derive(Clone, Debug)]
enum Command {
    SetMask(Mask),
    SetMem { location: u64, value: u64 },
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"mask = ([10X]{36})").unwrap();
            static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        }

        if MASK_RE.is_match(s) {
            let caps = MASK_RE.captures(s).unwrap();
            Ok(Command::SetMask(Mask::from_str(
                caps.get(1).unwrap().as_str(),
            )?))
        } else if MEM_RE.is_match(s) {
            let caps = MEM_RE.captures(s).unwrap();
            let location = u64::from_str(caps.get(1).unwrap().as_str()).unwrap();
            let value = u64::from_str(caps.get(2).unwrap().as_str()).unwrap();
            Ok(Command::SetMem { location, value })
        } else {
            Err(format!("Supplied string is not a valid command: {}", s))
        }
    }
}

#[derive(Clone, Default, Debug)]
struct Mask {
    zero_mask: u64,
    one_mask: u64,
    floating_locations: Vec<usize>,
}

impl Mask {
    fn new(zero_mask: u64, one_mask: u64, floating_locations: Vec<usize>) -> Self {
        Mask {
            zero_mask,
            one_mask,
            floating_locations,
        }
    }

    fn set_bit_at(num: u64, location: usize) -> u64 {
        num | (1 << location)
    }

    fn clear_bit_at(num: u64, location: usize) -> u64 {
        num & !(1 << location)
    }

    fn mask(&self, num: u64) -> u64 {
        Self::mask_with(num, self.zero_mask, self.one_mask)
    }

    fn mask_with(num: u64, zero_mask: u64, one_mask: u64) -> u64 {
        (num & zero_mask) | one_mask
    }

    fn masked_set(&self, num: u64) -> Vec<u64> {
        let start_num = num | self.one_mask;
        let mut masked_vals = vec![start_num];
        for loc in &self.floating_locations {
            masked_vals = masked_vals
                .into_iter()
                .flat_map(|i| vec![Self::set_bit_at(i, *loc), Self::clear_bit_at(i, *loc)])
                .collect()
        }

        masked_vals
    }
}

impl FromStr for Mask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.len() == 36 {
            return Err("Expected a 36-char bit mask".into());
        }
        if !s.chars().all(|c| c == 'X' || c == '1' || c == '0') {
            return Err("Expected only characters 'X', '0' and '1'".into());
        }
        let one_s: String = s.chars().map(|c| if c == '1' { c } else { '0' }).collect();
        let zero_s: String = s.chars().map(|c| if c == '0' { c } else { '1' }).collect();
        let x_locations: Vec<usize> = s
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| if c == 'X' { Some(35 - idx) } else { None })
            .collect();
        Ok(Mask::new(
            u64::from_str_radix(&zero_s, 2).unwrap(),
            u64::from_str_radix(&one_s, 2).unwrap(),
            x_locations,
        ))
    }
}
