use std::str::FromStr;
#[macro_use]
extern crate lazy_static;
use common::load_vec;
use regex::Regex;
use std::iter::repeat;

fn main() {
    let input: Vec<Instruction> = load_vec("input/day8.txt");
    part1(&input);
    part2(&input);
}

fn part1(input: &[Instruction]) {
    let mut machine = Machine::new();

    match machine.run_script(input) {
        ExitCondition::InvalidJump => panic!(
            "instruction_ptr is at {} which is beyond {}",
            machine.instruction_ptr,
            input.len()
        ),
        ExitCondition::InfiniteLoop => println!("Part 1: {}", machine.accumulator),
        _ => unreachable!(),
    }
}

fn part2(input: &[Instruction]) {
    let mut instruction_executed_count: Vec<usize> = repeat(0).take(input.len()).collect();
    let mut machine = Machine::new();

    while machine.instruction_ptr < input.len()
        && instruction_executed_count[machine.instruction_ptr] < 3
    {
        let current_instruction = &input[machine.instruction_ptr];
        instruction_executed_count[machine.instruction_ptr] += 1;
        if let Err(s) = machine.run(current_instruction) {
            panic!("{}", s);
        }
    }

    if machine.instruction_ptr >= input.len() {
        panic!(
            "instruction_ptr is at {} which is beyond {}",
            machine.instruction_ptr,
            input.len()
        )
    }

    let instructions_executed = instruction_executed_count
        .into_iter()
        .enumerate()
        .filter(|&(idx, cnt)| {
            cnt >= 1 && matches!(input[idx].0, InstructionType::Jmp | InstructionType::Nop)
        })
        .map(|(idx, _)| idx);

    let mut script = ModifiableScript::new(&input);
    for instruction in instructions_executed {
        if let (machine, ExitCondition::EndOfScript) = script.flip_instruction_and_run(instruction)
        {
            println!("Part 2: {}", machine.accumulator);
            return;
        }
    }

    panic!("No result found for Part 2")
}

struct ModifiableScript {
    instructions: Vec<Instruction>,
}

impl ModifiableScript {
    fn new(src: &[Instruction]) -> Self {
        let instructions = src.to_vec();

        ModifiableScript { instructions }
    }

    fn flip_instruction_and_run(&mut self, idx: usize) -> (Machine, ExitCondition) {
        let mut machine = Machine::new();
        let original_instr = self.instructions[idx];
        self.instructions[idx] = Self::flip_instruction(original_instr);
        let result = machine.run_script(&self.instructions);
        self.instructions[idx] = original_instr;
        (machine, result)
    }

    fn flip_instruction(instr: Instruction) -> Instruction {
        let mut modified = instr.clone();
        modified.0 = match instr.0 {
            InstructionType::Jmp => InstructionType::Nop,
            InstructionType::Nop => InstructionType::Jmp,
            _ => unreachable!(),
        };

        modified
    }
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    instruction_ptr: usize,
    accumulator: isize,
}

impl Machine {
    fn new() -> Self {
        Machine {
            instruction_ptr: 0,
            accumulator: 0,
        }
    }

    fn run(&mut self, instruction: &Instruction) -> Result<(), String> {
        use InstructionType::*;
        let move_amt = match instruction {
            Instruction(Acc, val) => {
                self.accumulator += val;
                1
            }
            Instruction(Jmp, val) => *val,
            Instruction(Nop, _) => 1,
        };

        self.move_ptr_by(move_amt)
    }

    fn move_ptr_by(&mut self, amount: isize) -> Result<(), String> {
        if amount >= 0 || self.instruction_ptr as isize >= -1 * amount {
            self.instruction_ptr = (self.instruction_ptr as isize + amount) as usize;
            Ok(())
        } else {
            Err(format!(
                "At {}: attempted an invalid jump by {}",
                self.instruction_ptr, amount
            ))
        }
    }

    fn run_script(&mut self, script: &[Instruction]) -> ExitCondition {
        let mut instruction_executed: Vec<bool> = repeat(false).take(script.len()).collect();

        while self.instruction_ptr < script.len() && !instruction_executed[self.instruction_ptr] {
            let current_instruction = &script[self.instruction_ptr];
            instruction_executed[self.instruction_ptr] = true;
            if let Err(s) = self.run(current_instruction) {
                eprintln!("{}", s);
                return ExitCondition::InvalidJump;
            }
        }
        if self.instruction_ptr > script.len() {
            ExitCondition::InvalidJump
        } else if self.instruction_ptr == script.len() {
            ExitCondition::EndOfScript
        } else {
            ExitCondition::InfiniteLoop
        }
    }
}

enum ExitCondition {
    EndOfScript,
    InfiniteLoop,
    InvalidJump,
}

#[derive(Debug, Clone, Copy)]
enum InstructionType {
    Jmp,
    Acc,
    Nop,
}

#[derive(Debug, Clone, Copy)]
struct Instruction(InstructionType, isize);

impl FromStr for InstructionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InstructionType::*;
        Ok(match s {
            "acc" => Acc,
            "nop" => Nop,
            "jmp" => Jmp,
            _ => return Err(format!("Unrecognized instruction: {}", s)),
        })
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex = Regex::new(r"(nop|acc|jmp) ([+\-]\d+)").unwrap();
        }

        let caps = INSTRUCTION_RE
            .captures(s)
            .ok_or(format!("{} did not match expected format", s))?;
        let arg = &caps[2].trim_start_matches("+");

        Ok(Instruction(
            InstructionType::from_str(&caps[1])?,
            isize::from_str(arg).map_err(|e| format!("{} is not a valid isize: {}", arg, e))?,
        ))
    }
}
