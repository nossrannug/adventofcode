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
    let mut valid_bags = HashSet::new();
    let bag_map = contents
        .split('\n')
        .filter(|line| !line.ends_with("no other bags."))
        .enumerate()
        .fold(HashMap::new(), |mut agg, (_, line)| {
            let info = line.split(" contain ").collect::<Vec<&str>>();
            let main_bag = info[0].split(' ').collect::<Vec<&str>>()[0..2]
                .iter()
                .map(|s| *s)
                .collect::<String>();
            let bags = info[1]
                .split(", ")
                .map(|bag| {
                    bag.split(' ').collect::<Vec<&str>>()[1..3]
                        .iter()
                        .map(|s| *s)
                        .collect::<String>()
                })
                .collect::<Vec<String>>();
            for bag in bags {
                agg.entry(bag)
                    .or_insert(Vec::new())
                    .push(main_bag.to_string());
            }
            agg
        });
    let mut seen = HashSet::new();
    let mut queue = vec!["shinygold".to_string()];
    loop {
        if queue.len() == 0 {
            break;
        }
        let bag = queue.pop().unwrap();
        if seen.insert(bag.clone()) {
            match bag_map.get(&bag) {
                Some(set) => {
                    queue.extend(set.iter().map(|b| b.clone()));
                    valid_bags.extend(set.iter().map(|b| b.clone()));
                }
                None => {}
            };
        }
    }
    valid_bags.len()
}

fn count_bags_needed(bag_map: &HashMap<String, Vec<(i32, String)>>, bag: &String) -> u64 {
    let mut sum = 0u64;
    match bag_map.get(bag) {
        Some(v) => {
            for (c, b) in v.iter() {
                sum += (*c as u64) + (*c as u64) * count_bags_needed(bag_map, b);
            }
        }
        _ => sum = 0,
    }
    sum
}

fn part_2(contents: &String) -> u64 {
    let bag_map = contents
        .split('\n')
        .filter(|line| !line.ends_with("no other bags."))
        .enumerate()
        .fold(HashMap::new(), |mut agg, (_, line)| {
            let info = line.split(" contain ").collect::<Vec<&str>>();
            let main_bag = info[0].split(' ').collect::<Vec<&str>>()[0..2]
                .iter()
                .map(|s| *s)
                .collect::<String>();
            let bags = info[1]
                .split(", ")
                .map(|bag| {
                    let bag_info = &bag.split(' ').collect::<Vec<&str>>();
                    (
                        bag_info[0].parse::<i32>().unwrap(),
                        bag_info[1..3]
                            .iter()
                            .map(|s| *s)
                            .collect::<String>()
                            .to_owned(),
                    )
                })
                .collect::<Vec<_>>();
            for bag in bags {
                agg.entry(main_bag.clone()).or_insert(Vec::new()).push(bag);
            }
            agg
        });
    count_bags_needed(&bag_map, &"shinygold".to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .to_string();
        assert_eq!(super::part_1(&test_data), 4);
    }

    #[test]
    fn part_2() {
        let data = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .to_string();
        assert_eq!(super::part_2(&data), 126);
    }

    #[test]
    fn part_2_1() {
        let data = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags."
            .to_string();
        assert_eq!(super::part_2(&data), 6);
    }

    #[test]
    fn part_2_2() {
        let test_data = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .to_string();
        assert_eq!(super::part_2(&test_data), 32);
    }
}
