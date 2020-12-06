use common::get_file_content;
use std::convert::TryInto;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().expect("Failed to get args").to_string())
        .expect("Failed to get file content");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn get_seat_id(line: &str) -> i32 {
    let row = line[0..7].chars().rev().enumerate().fold(0, |acc, (i, c)| {
        let n = if c == 'F' { 0 } else { 1 };
        acc + (2i32.pow(i.try_into().unwrap()) * n)
    });
    let col = line[7..10]
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| {
            let n = if c == 'L' { 0 } else { 1 };
            acc + (2i32.pow(i.try_into().unwrap()) * n)
        });
    row * 8 + col
}

fn part_1(contents: &String) -> i32 {
    contents
        .split('\n')
        .map(|line| get_seat_id(line))
        .max()
        .unwrap()
}

fn part_2(contents: &String) -> i32 {
    let ids = contents
        .split('\n')
        .map(|line| get_seat_id(line))
        .collect::<Vec<i32>>();
    find_seat(ids)
}

fn find_seat(values: Vec<i32>) -> i32 {
    let mut items = values.clone();
    items.sort();
    for i in 0..items.len() - 1 {
        if items[i] == items[i + 1] - 2 {
            return items[i] + 1;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::part_1(&"FBFBBFFRLR".to_string()), 357);
        assert_eq!(super::part_1(&"BFFFBBFRRR".to_string()), 567);
        assert_eq!(super::part_1(&"FFFBBBFRRR".to_string()), 119);
        assert_eq!(super::part_1(&"BBFFBBFRLL".to_string()), 820);
        assert_eq!(
            super::part_1(&"BBFFBBFRLL\nBBFFBBFRLL\nFFFBBBFRRR".to_string()),
            820
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(super::find_seat(vec![321, 358, 23, 356, 999]), 357);
    }
}
