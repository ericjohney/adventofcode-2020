use adventofcode2020::file_utils;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Instruction {
    id: i64,
    operation: String,
    value: i64,
}

impl Hash for Instruction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Instruction {}

fn main() {
    let instructions = file_utils::lines("inputs/day8.txt")
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let split = l.split(" ").collect::<Vec<_>>();
            Instruction {
                id: i as i64,
                operation: split[0].to_string(),
                value: str::parse::<i64>(&split[1].replace("+", "")).unwrap(),
            }
        })
        .collect();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &Vec<Instruction>) {
    let mut accumulator = 0i64;
    let mut pointer = 0i64;
    let mut seen = HashSet::new();
    while pointer < instructions.len() as i64 {
        let instruction = &instructions[pointer as usize];
        if seen.contains(instruction) {
            break;
        }

        match instruction.operation.as_str() {
            "acc" => {
                accumulator += instruction.value;
                pointer += 1;
            }
            "jmp" => {
                pointer += instruction.value;
            }
            _ => pointer += 1,
        }
        seen.insert(instruction);
    }
    println!("part 1 {:?}", accumulator);
}

fn part2(instructions: &Vec<Instruction>) {
    fn run_computer(instructions: &Vec<Instruction>) -> bool {
        let mut accumulator = 0i64;
        let mut pointer = 0i64;
        let mut seen = HashSet::new();
        while pointer < instructions.len() as i64 {
            let instruction = &instructions[pointer as usize];
            if seen.contains(instruction) {
                return false;
            }

            let operation = instruction.operation.as_str();
            match operation {
                "acc" => {
                    accumulator += instruction.value;
                    pointer += 1;
                }
                "jmp" => {
                    pointer += instruction.value;
                }
                _ => pointer += 1,
            }
            seen.insert(instruction);
        }
        println!("part 2 {}", accumulator);
        return true;
    }

    let mut result = false;
    let mut index = 0;
    while !result {
        result = run_computer(
            &instructions
                .iter()
                .map(|instruction| {
                    let mut operation = instruction.operation.as_str();
                    if instruction.id == index {
                        operation = match operation {
                            "jmp" => "nop",
                            "nop" => "jmp",
                            _ => operation,
                        }
                    }
                    return Instruction {
                        id: instruction.id,
                        operation: operation.to_string(),
                        value: instruction.value,
                    };
                })
                .collect(),
        );
        index += 1;
    }
}
