use common::get_file_content;
use std::{collections::{VecDeque}, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> i64 {
    process(25, &get_numbers(contents))
}

fn part_2(contents: &String) -> i64 {
    process_2(25, &get_numbers(contents))
}

fn process(n: usize, numbers: &Vec<i64>) -> i64 {
    let mut q: VecDeque<i64> = VecDeque::with_capacity(n);
    for number in numbers.iter() {
        if q.len() < n {
            q.push_back(*number);
            continue;
        }

        if !has_sum(*number, &q) {
            return *number;
        }
        q.pop_front();
        q.push_back(*number);
    }
    -1
}

fn process_2(n: usize, numbers: &Vec<i64>) -> i64 {
    let number = process(n, numbers);
    let (a,b) = get_slice(number, numbers);
    let slice = &numbers[a..=b];
    slice.iter().min().unwrap() + slice.iter().max().unwrap()
}

fn get_slice(x:i64, numbers: &Vec<i64>) -> (usize, usize) {
    for a in 0..numbers.len()-1 {
        let mut sum = numbers[a];
        for b in a+1..numbers.len() {
            sum += numbers[b];
            if sum == x {
                return (a,b);
            }
            if sum > x {
                break;
            }
        }
    }
    (0,0)
}

fn has_sum(x: i64, v: &VecDeque<i64>) -> bool {
    for a in 0..v.len()-1 {
        for b in a+1..v.len() {
            if v[a] + v[b] == x {
                return true;
            }
        }
    }
    false
}

fn get_numbers(contents: &String) -> Vec<i64> {
    contents.split('\n').map(|line| line.parse::<i64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
            .to_string();
        assert_eq!(super::process(5, &super::get_numbers(&test_data)), 127);
    }

    #[test]
    fn part_2() {
        let data = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
            .to_string();
            assert_eq!(super::process_2(5, &super::get_numbers(&data)), 62);
    }
}
