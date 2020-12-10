use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "nop" => OpCode::Nop,
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            _ => return Err(()),
        })
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    opcode: OpCode,
    operand: isize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let opcode_string = parts.get(0).map_or(Err(()), Ok)?;
        let operand_string: &str = parts.get(1).map_or(Err(()), Ok)?;
        Ok(Instruction {
            opcode: OpCode::from_str(opcode_string)?,
            operand: operand_string.parse::<isize>().map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day8)]
fn parse_input_day8(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|line| Instruction::from_str(line).map_or(None, Some))
        .collect::<Vec<Instruction>>()
}

enum TerminationReason {
    InvalidAddress,
    EndOfInstructions,
    InfiniteLoop,
}

struct VM {
    instructions: Vec<Instruction>,
    executed: HashSet<usize>,
    address: usize,
    accumulator: isize,
}

impl VM {
    fn from_instructions(instructions: &Vec<Instruction>) -> Self {
        VM {
            instructions: instructions.clone(),
            executed: HashSet::new(),
            address: 0,
            accumulator: 0,
        }
    }

    fn reset(&mut self) {
        self.executed.clear();
        self.address = 0;
        self.accumulator = 0;
    }

    fn run(&mut self) -> TerminationReason {
        while let Some(instruction) = self.instructions.get(self.address) {
            if self.executed.contains(&self.address) {
                return TerminationReason::InfiniteLoop;
            }
            self.executed.insert(self.address);

            match instruction.opcode {
                OpCode::Nop => {}
                OpCode::Acc => {
                    self.accumulator += instruction.operand;
                }
                OpCode::Jmp => {
                    self.address = (self.address as isize + instruction.operand - 1) as usize;
                }
            }

            self.address += 1;
        }

        if self.address == self.instructions.len() {
            TerminationReason::EndOfInstructions
        } else {
            TerminationReason::InvalidAddress
        }
    }
}

#[aoc(day8, part1)]
fn part1(input: &Vec<Instruction>) -> isize {
    let mut vm = VM::from_instructions(input);
    vm.run();
    vm.accumulator
}

#[aoc(day8, part2)]
fn part2(input: &Vec<Instruction>) -> Option<isize> {
    let mut vm = VM::from_instructions(input);

    for i in 0..vm.instructions.len() {
        let instruction = vm.instructions.get_mut(i).unwrap();
        let original_opcode = instruction.opcode;
        match original_opcode {
            OpCode::Jmp => instruction.opcode = OpCode::Nop,
            OpCode::Nop => instruction.opcode = OpCode::Jmp,
            _ => {}
        }

        let reason = vm.run();
        match reason {
            TerminationReason::EndOfInstructions => return Some(vm.accumulator),
            _ => vm.reset(),
        }

        let instruction = vm.instructions.get_mut(i).unwrap();
        instruction.opcode = original_opcode;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input_day8(INPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&mut parse_input_day8(INPUT)), Some(8));
    }
}
