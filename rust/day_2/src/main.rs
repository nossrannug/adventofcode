use common::get_file_content;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().expect("Failed to get args").to_string())
        .expect("Failed to get file content");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

struct Validator {}

impl Validator {
    pub fn valid_1(input: &str) -> i32 {
        let groups: Vec<&str> = input.split(' ').collect();
        let numbers: Vec<i32> = groups[0]
            .split('-')
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let letter: char = groups[1].chars().next().unwrap();
        let count: i32 = groups[2]
            .chars()
            .map(|c| if c == letter { 1 } else { 0 })
            .sum();
        if count >= numbers[0] && count <= numbers[1] {
            return 1;
        }
        0
    }

    pub fn valid_2(input: &str) -> i32 {
        let groups: Vec<&str> = input.split(' ').collect();
        let numbers: Vec<usize> = groups[0]
            .split('-')
            .map(|number| number.parse::<usize>().unwrap() - 1)
            .collect::<Vec<usize>>();
        let letter: char = groups[1].chars().next().unwrap();

        let mut found = false;
        for (i, c) in groups[2].chars().enumerate() {
            if i == numbers[0] && c == letter {
                found = !found;
            } else if i == numbers[1] && c == letter {
                found = !found;
            }
        }
        if found {
            1
        } else {
            0
        }
    }
}

fn part_1(contents: &String) -> i32 {
    contents
        .split('\n')
        .map(|line| Validator::valid_1(line))
        .sum()
}

fn part_2(contents: &String) -> i32 {
    contents
        .split('\n')
        .map(|line| Validator::valid_2(line))
        .sum()
}
