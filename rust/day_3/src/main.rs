use common::get_file_content;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().expect("Failed to get args").to_string())
        .expect("Failed to get file content");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn get_chars(s: &String) -> Vec<Vec<char>> {
    s.split('\n').map(|line| line.chars().collect()).collect()
}

fn part_1(contents: &String) -> i32 {
    let ok: Vec<Vec<char>> = get_chars(contents);
    let width = ok[0].len();
    let mut count = 0;
    for i in 0..ok.len() {
        let y = (i * 3) % width;
        if ok[i][y] == '#' {
            count += 1;
        }
    }
    count
}

fn part_2(contents: &String) -> u64 {
    let ok = get_chars(contents);
    let mut count = 1u64;
    let slopes = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];
    for slope in slopes {
        count *= count_trees(&ok, slope)
    }
    count
}

struct Slope {
    x: usize,
    y: usize,
}

fn count_trees(map: &Vec<Vec<char>>, slope: Slope) -> u64 {
    let width = map[0].len();
    let mut count = 0;
    let steps: usize = map.len() / slope.y;
    for i in 0..steps {
        let x = (i * slope.x) % width;
        let y = i * slope.y;
        if map[y][x] == '#' {
            count += 1;
        }
    }
    count
}
