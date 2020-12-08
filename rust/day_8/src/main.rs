use common::get_file_content;
use std::{collections::HashSet, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

#[derive(Clone)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

impl Instruction {
    pub fn parse(line: &str) -> Instruction {
        let instruction: Vec<&str> = line.split(' ').collect();
        let n = instruction[1].parse::<i32>().unwrap();
        match instruction[0] {
            "acc" => Instruction::ACC(n),
            "jmp" => Instruction::JMP(n),
            "nop" => Instruction::NOP(n),
            _ => panic!("what??"),
        }
    }
}

fn part_1(contents: &String) -> i32 {
    let instructions = get_instructions(contents);
    match run_instructions(&instructions) {
        RunStatus::LOOP(n) => n,
        RunStatus::END(_) => panic!("Part 1 didn't loop!!!"),
    }
}

fn part_2(contents: &String) -> i32 {
    let instructions = get_instructions(contents);
    let mut test_instructions = instructions.clone();
    for (idx, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::ACC(_) => {}
            Instruction::JMP(n) => {
                test_instructions[idx] = Instruction::NOP(*n);
                match run_instructions(&test_instructions) {
                    RunStatus::END(n) => return n,
                    _ => test_instructions[idx] = Instruction::JMP(*n),
                }
            }
            Instruction::NOP(n) => {
                test_instructions[idx] = Instruction::JMP(*n);
                match run_instructions(&test_instructions) {
                    RunStatus::END(n) => return n,
                    _ => test_instructions[idx] = Instruction::NOP(*n),
                }
            }
        }
    }
    -1
}

fn get_instructions(s: &String) -> Vec<Instruction> {
    s.split('\n')
        .map(Instruction::parse)
        .collect::<Vec<Instruction>>()
}

enum RunStatus {
    LOOP(i32),
    END(i32),
}

fn run_instructions(instructions: &Vec<Instruction>) -> RunStatus {
    let mut seen = HashSet::new();
    let mut cur = 0i32;
    let mut acc = 0;
    loop {
        if (cur as usize) >= instructions.len() {
            return RunStatus::END(acc);
        }
        match instructions[(cur as usize)] {
            Instruction::ACC(n) => {
                cur += 1;
                if seen.insert(cur) {
                    acc += n;
                } else {
                    return RunStatus::LOOP(acc);
                }
            }
            Instruction::JMP(n) => {
                cur += n;
                if !seen.insert(cur) {
                    return RunStatus::LOOP(acc);
                }
            }
            Instruction::NOP(_) => {
                cur += 1;
                if !seen.insert(cur) {
                    return RunStatus::LOOP(acc);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .to_string();
        assert_eq!(super::part_1(&test_data), 5);
    }

    #[test]
    fn part_2() {
        let data = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .to_string();
        assert_eq!(super::part_2(&data), 8);
    }
}
