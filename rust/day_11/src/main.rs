use common::get_file_content;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> i32 {
    let mut seats: Vec<Vec<char>> = contents
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    seats = process(seats, true, 4);
    count_occupied(&seats)
}

fn part_2(contents: &String) -> i32 {
    let mut seats: Vec<Vec<char>> = contents
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    seats = process(seats, false, 5);
    count_occupied(&seats)
}
fn process(mut seats: Vec<Vec<char>>, adj: bool, limit: i32) -> Vec<Vec<char>> {
    loop {
        let mut update = false;

        let mut new_seats = seats.clone();
        for i in 0..new_seats.len() {
            for j in 0..new_seats[i].len() {
                match seats[i][j] {
                    'L' => {
                        if get_seat_count((i, j), &seats, adj) == 0 {
                            new_seats[i][j] = '#';
                            update = true;
                        }
                    }
                    '#' => {
                        if get_seat_count((i, j), &seats, adj) >= limit {
                            new_seats[i][j] = 'L';
                            update = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        seats = new_seats;

        if !update {
            break;
        }
    }
    seats
}

fn get_seat_count((x, y): (usize, usize), seats: &Vec<Vec<char>>, adj: bool) -> i32 {
    let checks = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for (a, b) in checks.iter() {
        let mut loops = 1;
        loop {
            let i: i32 = (a * loops) + (x as i32);
            let j: i32 = (b * loops) + (y as i32);
            if i < 0 || i >= (seats.len() as i32) {
                break;
            }
            if j < 0 || j >= (seats[0].len() as i32) {
                break;
            }
            if seats[(i as usize)][(j as usize)] == '#' {
                count += 1;
                break;
            }
            if seats[(i as usize)][(j as usize)] == 'L' {
                break;
            }
            if adj {
                break;
            }
            loops += 1;
        }
    }
    count
}

fn count_occupied(seats: &Vec<Vec<char>>) -> i32 {
    seats
        .iter()
        .map(|row| {
            return row
                .iter()
                .map(|seat| if *seat == '#' { 1 } else { 0 })
                .sum::<i32>();
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .to_string();
        assert_eq!(super::part_1(&test_data), 37);
    }

    #[test]
    fn part_2() {
        let test_data = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .to_string();
        assert_eq!(super::part_2(&test_data), 26);
    }
}
