use adventofcode2020::file_utils;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Computer {
    pointer: i64,
    instructions: Vec<Instruction>,
    accumulator: i64,
}

#[derive(Debug, Clone)]
struct Instruction {
    operation: String,
    value: i64,
}

impl Computer {
    fn new(instructions: &Vec<Instruction>) -> Computer {
        return Computer {
            pointer: 0i64,
            accumulator: 0i64,
            instructions: instructions.iter().cloned().collect(),
        };
    }

    fn next_instruction(&self) -> Option<&Instruction> {
        if self.pointer < self.instructions.len() as i64 {
            return Some(&self.instructions[self.pointer as usize]);
        }
        None
    }

    fn has_completed(&self) -> bool {
        self.pointer == self.instructions.len() as i64
    }

    fn run(&mut self) {
        let mut seen = HashSet::new();
        while let Some(instruction) = self.next_instruction() {
            if seen.contains(&self.pointer) {
                break;
            }
            seen.insert(self.pointer);

            match instruction.operation.as_str() {
                "acc" => {
                    self.accumulator += instruction.value;
                    self.pointer += 1;
                }
                "jmp" => {
                    self.pointer += instruction.value;
                }
                _ => self.pointer += 1,
            }
        }
    }
}

fn main() {
    let instructions = file_utils::lines("inputs/day8.txt")
        .iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            Instruction {
                operation: split[0].to_string(),
                value: str::parse::<i64>(&split[1].replace("+", "")).unwrap(),
            }
        })
        .collect();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &Vec<Instruction>) {
    let mut computer = Computer::new(instructions);
    computer.run();

    println!("part 1 {:?}", computer.accumulator);
}

fn part2(instructions: &Vec<Instruction>) {
    let mut result = false;
    let mut index = 0;
    let mut last_accumulator = 0;
    while !result {
        let new_instructions = &instructions
            .iter()
            .enumerate()
            .map(|(id, instruction)| {
                let mut operation = instruction.operation.as_str();
                if id == index {
                    operation = match operation {
                        "jmp" => "nop",
                        "nop" => "jmp",
                        _ => operation,
                    }
                }
                return Instruction {
                    operation: operation.to_string(),
                    value: instruction.value,
                };
            })
            .collect();

        let mut computer = Computer::new(new_instructions);
        computer.run();
        result = computer.has_completed();

        last_accumulator = computer.accumulator;
        index += 1;
    }
    println!("part 2 {:?}", last_accumulator);
}
