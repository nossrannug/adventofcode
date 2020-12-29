use common::get_file_content;
use std::{collections::HashSet, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> i32 {
    let groups = contents.split("\n\n").collect::<Vec<&str>>();
    let map = groups[0].split('\n').fold(HashSet::new(), |mut agg, s| {
        let items = s.split(": ").collect::<Vec<&str>>();
        let numbers = items[1].split(' ').fold(Vec::new(), |mut v, i| {
            let nums = i.split('-').collect::<Vec<&str>>();
            if nums.len() == 2 {
                v.push(nums[0].parse::<i32>().unwrap());
                v.push(nums[1].parse::<i32>().unwrap());
            }
            v
        });
        for i in numbers[0]..=numbers[1] {
            agg.insert(i);
        }
        for i in numbers[2]..=numbers[3] {
            agg.insert(i);
        }
        agg
    });
    groups[2]
        .split('\n')
        .skip(1)
        .flat_map(|line| line.split(',').map(|n| n.parse::<i32>().unwrap()))
        .fold(Vec::new(), |mut v, n| {
            if !map.get(&n).is_some() {
                v.push(n);
            }
            v
        })
        .iter()
        .sum()
}

fn part_2(contents: &String) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .to_string();
        assert_eq!(super::part_1(&test_data), 71);
    }

    #[test]
    fn part_2() {
        let test_data = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            .to_string();
        assert_eq!(super::part_2(&test_data), 175594);
    }
}
