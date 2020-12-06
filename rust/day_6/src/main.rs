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
    let mut group_id = 0;
    contents
        .split('\n')
        .fold(HashMap::<i32, HashSet<char>>::new(), |mut agg, line| {
            if line == "" {
                group_id += 1;
                return agg;
            }
            for c in line.chars() {
                agg.entry(group_id)
                    .or_insert(HashSet::<char>::new())
                    .insert(c);
            }
            agg
        })
        .iter()
        .map(|(_, value)| value.len())
        .sum()
}

fn part_2(contents: &String) -> i32 {
    let mut group_id = 0;
    contents
        .split('\n')
        .enumerate()
        .fold(HashMap::new(), |mut agg, (user_id, line)| {
            if line == "" {
                group_id += 1;
                return agg;
            }
            for c in line.chars() {
                agg.entry(group_id)
                    .or_insert(HashMap::new())
                    .entry(user_id)
                    .or_insert(HashSet::new())
                    .insert(c);
            }
            agg
        })
        .iter()
        .map(|(_, value)| {
            let sets = value
                .iter()
                .map(|(_, set)| set.clone())
                .collect::<Vec<HashSet<char>>>();
            let letters = sets.iter().fold(HashMap::new(), |mut agg, set| {
                for c in set.iter() {
                    *agg.entry(c).or_insert(0usize) += 1;
                }
                agg
            });
            letters.iter().fold(
                0,
                |agg, (_, n)| if *n == sets.len() { agg + 1 } else { agg },
            )
        })
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
        let data2 = "a\n\nb\nb".to_string();
        assert_eq!(super::part_2(&data2), 2);
    }
}
