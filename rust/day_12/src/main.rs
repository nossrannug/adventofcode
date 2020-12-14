use common::get_file_content;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

/*
Action N means to move north by the given value.
Action S means to move south by the given value.
Action E means to move east by the given value.
Action W means to move west by the given value.
Action L means to turn left the given number of degrees.
Action R means to turn right the given number of degrees.
Action F means to move forward by the given value in the direction the ship is currently facing.
*/

#[derive(Debug)]
struct Ship {
    facing: char,
    coords: (i32, i32),
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            facing: 'E',
            coords: (0, 0),
        }
    }

    pub fn move_ship(&mut self, instruction: &String) {
        let n = instruction[1..].parse::<i32>().unwrap();
        match instruction.chars().nth(0).unwrap() {
            'N' => {
                self.coords.1 += n;
            }
            'S' => {
                self.coords.1 -= n;
            }
            'E' => {
                self.coords.0 += n;
            }
            'W' => {
                self.coords.0 -= n;
            }
            'L' => {
                self.change_direction(n, 'L');
            }
            'R' => {
                self.change_direction(n, 'R');
            }
            'F' => {
                self.coords.0 += self.get_direction().0 * n;
                self.coords.1 += self.get_direction().1 * n;
            }
            _ => panic!("ARGH!"),
        }
    }

    fn get_direction(&self) -> (i32, i32) {
        match self.facing {
            'N' => (0, 1),
            'S' => (0, -1),
            'E' => (1, 0),
            'W' => (-1, 0),
            _ => panic!("asdf"),
        }
    }

    fn change_direction(&mut self, degrees: i32, direction: char) {
        let directions = vec!['E', 'N', 'W', 'S'];
        let mut n = directions
            .iter()
            .position(|&item| item == self.facing)
            .unwrap() as i32;
        let turns = degrees / 90;
        if direction == 'L' {
            n += turns;
            if n > 3 {
                n -= 4;
            }
        } else if direction == 'R' {
            n -= turns;
            if n < 0 {
                n += 4;
            }
        }
        self.facing = directions[n as usize];
    }

    pub fn manhattan(&self) -> i32 {
        self.coords.0.abs() + self.coords.1.abs()
    }
}

fn part_1(contents: &String) -> i32 {
    let instructions = contents
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let mut ship = Ship::new();
    for instruction in instructions {
        ship.move_ship(&instruction);
    }
    ship.manhattan()
}

fn part_2(contents: &String) -> i32 {
    let instructions = contents
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let mut ship = Ship::new();
    for instruction in instructions {
        ship.move_ship(&instruction);
    }
    ship.manhattan()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "F10
N3
F7
R90
F11"
        .to_string();
        assert_eq!(super::part_1(&test_data), 25);
    }

    #[test]
    fn part_2() {
        let test_data = "F10
N3
F7
R90
F11"
        .to_string();
        assert_eq!(super::part_2(&test_data), -1);
    }
}
