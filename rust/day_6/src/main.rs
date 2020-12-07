use common::get_file_content;
use std::{
    collections::{HashMap, HashSet},
    env,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> usize {
    contents
        .split("\n\n")
        .map(|lines| {
            lines
                .split('\n')
                .fold(HashSet::new(), |mut agg, line| {
                    agg.extend(line.chars());
                    agg
                })
                .len()
        })
        .sum()
}

fn part_2(contents: &String) -> usize {
    contents
        .split("\n\n")
        .enumerate()
        .map(|(_, lines)| {
            lines
                .split('\n')
                .enumerate()
                .fold(HashSet::new(), |agg, (i, line)| {
                    if i == 0 {
                        line.chars().collect()
                    } else {
                        agg.intersection(&line.chars().collect())
                            .map(|c| c.clone())
                            .collect::<HashSet<char>>()
                    }
                })
        })
        .map(|set| set.len())
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "abc

a
b
c

ab
ac

a
a
a
a

b"
        .to_string();
        assert_eq!(super::part_1(&test_data), 11);
    }

    #[test]
    fn part_2() {
        let data = "abc

a
b
c

ab
ac

a
a
a
a

b"
        .to_string();
        assert_eq!(super::part_2(&data), 6);
        // let data2 = "a\n\nb\nb".to_string();
        // assert_eq!(super::part_2(&data2), 2);
    }
}
