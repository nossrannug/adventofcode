use common::get_file_content;
use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

#[derive(Debug)]
struct E {
    mask: String,
    memory: HashMap<i64, i64>,
}

impl E {
    pub fn write_to_mem(&mut self, loc: i64, v: i64) {
        self.memory.insert(
            loc,
            E::get_number(&E::apply_mask(&E::get_bits(v), &self.mask)),
        );
    }

    pub fn write_to_mem_v2(&mut self, loc: i64, v: i64) {
        self.memory.insert(loc, v);
    }

    pub fn sum_memory(&self) -> i64 {
        let mut total = 0i64;
        for (_, v) in &self.memory {
            total += *v;
        }
        total
    }

    fn get_mem_locs(loc: i64, mask: &String) -> Vec<i64> {
        let mut bit_chars = E::get_bits(loc).chars().collect::<Vec<char>>();
        let mask_chars = mask.chars().collect::<Vec<char>>();
        for (idx, c) in mask_chars.iter().enumerate() {
            if *c != '0' {
                bit_chars[idx] = *c
            }
        }
        let mut all_masks: Vec<Vec<char>> = Vec::new();
        for c in bit_chars {
            if c == 'X' {
                let mut next: Vec<Vec<char>> = Vec::new();
                if all_masks.len() == 0 {
                    next.push(vec!['0']);
                    next.push(vec!['1']);
                } else {
                    for m in all_masks {
                        let mut n1 = m.clone();
                        let mut n2 = m.clone();
                        n1.push('0');
                        n2.push('1');
                        next.push(n1);
                        next.push(n2);
                    }
                }
                all_masks = next;
            } else {
                let mut next: Vec<Vec<char>> = Vec::new();
                if all_masks.len() == 0 {
                    next.push(vec![c]);
                } else {
                    for a in all_masks {
                        let mut n = a.clone();
                        n.push(c);
                        next.push(n);
                    }
                }
                all_masks = next;
            }
        }
        let result = all_masks
            .iter()
            .map(|m| E::get_number(&m.iter().collect::<String>()))
            .collect();

        result
    }

    fn get_bits(v: i64) -> String {
        format!("{:036b}", v)
    }

    fn apply_mask(bits: &String, mask: &String) -> String {
        let mut bit_chars = bits.chars().collect::<Vec<char>>();
        let mask_chars = mask.chars().collect::<Vec<char>>();
        for (idx, c) in mask_chars.iter().enumerate() {
            if *c != 'X' {
                bit_chars[idx] = *c;
            }
        }
        bit_chars.iter().collect()
    }

    fn get_number(bits: &String) -> i64 {
        i64::from_str_radix(bits, 2).unwrap()
    }
}

fn part_1(contents: &String) -> i64 {
    let mut e = E {
        mask: "".to_string(),
        memory: HashMap::new(),
    };
    let lines = contents.split('\n').collect::<Vec<&str>>();
    for line in lines {
        let values = line.split(" = ").collect::<Vec<&str>>();
        if line.starts_with("mask = ") {
            e.mask = line[7..].to_string();
        } else {
            e.write_to_mem(
                values[0][4..values[0].len() - 1].parse::<i64>().unwrap(),
                values[1].parse::<i64>().unwrap(),
            );
        }
    }
    e.sum_memory()
}

fn part_2(contents: &String) -> i64 {
    let mut e = E {
        mask: "".to_string(),
        memory: HashMap::new(),
    };
    let lines = contents.split('\n').collect::<Vec<&str>>();
    for line in lines {
        let values = line.split(" = ").collect::<Vec<&str>>();
        if line.starts_with("mask = ") {
            e.mask = line[7..].to_string();
        } else {
            let mem_locs = E::get_mem_locs(
                values[0][4..values[0].len() - 1].parse::<i64>().unwrap(),
                &e.mask,
            );
            for loc in mem_locs {
                e.write_to_mem_v2(loc, values[1].parse::<i64>().unwrap());
            }
        }
    }
    e.sum_memory()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .to_string();
        assert_eq!(super::part_1(&test_data), 165);
    }

    #[test]
    fn get_bits() {
        let test_data = 10;
        assert_eq!(
            super::E::get_bits(test_data),
            "000000000000000000000000000000001010"
        );
    }

    #[test]
    fn apply_mask() {
        let bits = "000000000000000000000000000000001010".to_string();
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXX1XXXXXXXXXXX".to_string();
        assert_eq!(
            super::E::apply_mask(&bits, &mask),
            "000000000000000000000000100000001010"
        );
    }

    #[test]
    fn get_number() {
        let bits = "000000000000000000000000000000001010".to_string();
        assert_eq!(super::E::get_number(&bits), 10);
    }

    #[test]
    fn get_mem_locs() {
        let mask = "000000000000000000000000000000X1001X".to_string();
        assert_eq!(super::E::get_mem_locs(42, &mask), vec![26, 27, 58, 59]);
    }

    #[test]
    fn part_2() {
        let test_data = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .to_string();
        assert_eq!(super::part_2(&test_data), 208);
    }
}
