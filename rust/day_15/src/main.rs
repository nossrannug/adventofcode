use common::get_file_content;
use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

#[derive(Debug)]
struct Game {
    turn: usize,
    last_number: i32,
    numbers: HashMap<i32, usize>,
}

impl Game {
    pub fn new(numbers: Vec<i32>) -> Game {
        Game {
            turn: numbers.len(),
            last_number: numbers[numbers.len()-1],
            numbers: numbers[0..numbers.len()-1]
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut map, (idx, n)| {
                    map.insert(*n, idx+1);
                    map
                }),
        }
    }
    
    pub fn take_turn(&mut self) -> i32 {
        match self.numbers.insert(self.last_number, self.turn) {
            Some(n) => {
                self.last_number = self.turn as i32 - n as i32;
            }
            None => {
                self.last_number = 0;
            }
        }
        self.turn += 1;
        self.last_number
    }

    pub fn get_nth_number(&mut self, n: usize) -> i32 {
        while self.turn < n {
            self.take_turn();
        }
        self.last_number
    }
}

fn part_1(contents: &String) -> i32 {
    let numbers = contents.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut game = Game::new(numbers);
    game.get_nth_number(2020)
}

fn part_2(contents: &String) -> i32 {
    let numbers = contents.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut game = Game::new(numbers);
    game.get_nth_number(30000000)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "0,3,6"
            .to_string();
        assert_eq!(super::part_1(&test_data), 436);
    }

    #[test]
    fn part_1_1() {
        let test_data = "2,3,1"
            .to_string();
        assert_eq!(super::part_1(&test_data), 78);
    }

    #[test]
    fn part_2() {
        let test_data = "0,3,6"
            .to_string();
        assert_eq!(super::part_2(&test_data), 175594);
    }
}
