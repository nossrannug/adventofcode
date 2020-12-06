extern crate time;
use common::get_file_content;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().expect("Failed to get args").to_string())
        .expect("Failed to get file content");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn get_numbers(s: &String) -> Vec<i32> {
    s.split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn part_1(contents: &String) -> i32 {
    let numbers = get_numbers(contents);
    for a in 0..numbers.len() - 2 {
        for b in a + 1..numbers.len() {
            if numbers[a] + numbers[b] == 2020 {
                return numbers[a] * numbers[b];
            }
        }
    }
    -1
}

fn part_2(contents: &String) -> i32 {
    let numbers = get_numbers(&contents);
    for a in 0..numbers.len() - 2 {
        for b in a + 1..numbers.len() - 1 {
            for c in b + 1..numbers.len() {
                if numbers[a] + numbers[b] + numbers[c] == 2020 {
                    return numbers[a] * numbers[b] * numbers[c];
                }
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_1() {
        assert_eq!(super::part_1(&"100\n200\n300\n1920".to_string()), 192000);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(super::part_2(&"100\n200\n300\n1620".to_string()), 48600000);
    }
}
