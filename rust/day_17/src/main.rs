use common::get_file_content;
use std::{collections::HashSet, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32, i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube(i32, i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube4d(i32, i32, i32, i32);

fn process_3d(grid: &Grid) -> Grid {
    let surrounding = vec![
        (1, 1, 1),
        (1, 0, 1),
        (0, 1, 1),
        (0, 0, 1),
        (-1, 1, 1),
        (1, -1, 1),
        (-1, -1, 1),
        (-1, 0, 1),
        (0, -1, 1),
        (1, 1, -1),
        (1, 0, -1),
        (0, 1, -1),
        (0, 0, -1),
        (-1, 1, -1),
        (1, -1, -1),
        (-1, -1, -1),
        (-1, 0, -1),
        (0, -1, -1),
        (1, 1, 0),
        (1, 0, 0),
        (0, 1, 0),
        (-1, 1, 0),
        (1, -1, 0),
        (-1, -1, 0),
        (-1, 0, 0),
        (0, -1, 0),
    ];

    let mut inactive_cubes: HashSet<Cube> = HashSet::new();
    let mut new_grid = Grid::new();
    for cube in &grid.cubes {
        let mut count = 0usize;
        for (x, y, z) in &surrounding {
            let value = Cube(cube.0 + x, cube.1 + y, cube.2 + z);
            if let Some(_) = grid.cubes.get(&value) {
                count += 1;
            } else {
                inactive_cubes.insert(value);
            }
        }
        if count == 2 || count == 3 {
            new_grid.cubes.insert(cube.clone());
        }
    }

    for cube in &inactive_cubes {
        let mut count = 0usize;
        for (x, y, z) in &surrounding {
            let value = Cube(cube.0 + x, cube.1 + y, cube.2 + z);
            if let Some(_) = grid.cubes.get(&value) {
                count += 1;
            }
            if count > 3 {
                break;
            }
        }
        if count == 3 {
            new_grid.cubes.insert(cube.clone());
        }
    }

    new_grid
}

fn generate_4d() -> Vec<(i32, i32, i32, i32)> {
    let p = vec![-1, 0, 1];
    let mut v = Vec::new();
    for x in &p {
        for y in &p {
            for z in &p {
                for w in &p {
                    if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                        continue;
                    }
                    v.push((*x, *y, *z, *w));
                }
            }
        }
    }
    v
}

fn process_4d(grid: &Grid4d) -> Grid4d {
    let surrounding = generate_4d();

    let mut inactive_cubes: HashSet<Cube4d> = HashSet::new();
    let mut new_grid = Grid4d::new();
    for cube in &grid.cubes {
        let mut count = 0usize;
        for (x, y, z, w) in &surrounding {
            let value = Cube4d(cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w);
            if let Some(_) = grid.cubes.get(&value) {
                count += 1;
            } else {
                inactive_cubes.insert(value);
            }
        }
        if count == 2 || count == 3 {
            new_grid.cubes.insert(cube.clone());
        }
    }

    for cube in &inactive_cubes {
        let mut count = 0usize;
        for (x, y, z, w) in &surrounding {
            let value = Cube4d(cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w);
            if let Some(_) = grid.cubes.get(&value) {
                count += 1;
            }
            if count > 3 {
                break;
            }
        }
        if count == 3 {
            new_grid.cubes.insert(cube.clone());
        }
    }

    new_grid
}

#[derive(Debug, Clone)]
struct Grid {
    cubes: HashSet<Cube>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cubes: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid4d {
    cubes: HashSet<Cube4d>,
}

impl Grid4d {
    pub fn new() -> Grid4d {
        Grid4d {
            cubes: HashSet::new(),
        }
    }
}

fn part_1(contents: &String) -> usize {
    let mut grid = Grid::new();
    contents.split('\n').enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c == '#' {
                grid.cubes.insert(Cube(x as i32, y as i32, 0i32));
            }
        });
    });
    for _ in 0..6 {
        grid = process_3d(&grid);
    }
    grid.cubes.len()
}

fn part_2(contents: &String) -> usize {
    let mut grid = Grid4d::new();
    contents.split('\n').enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c == '#' {
                grid.cubes.insert(Cube4d(x as i32, y as i32, 0i32, 0i32));
            }
        });
    });
    for _ in 0..6 {
        grid = process_4d(&grid);
    }
    grid.cubes.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = ".#.
..#
###"
        .to_string();
        assert_eq!(super::part_1(&test_data), 112);
    }

    #[test]
    fn part_2() {
        let test_data = ".#.
..#
###"
        .to_string();
        assert_eq!(super::part_2(&test_data), 848);
    }
}
