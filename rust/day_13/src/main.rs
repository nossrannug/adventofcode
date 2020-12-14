use common::get_file_content;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> i32 {
    let lines = contents.split('\n').collect::<Vec<&str>>();
    let depart = lines[0].parse::<i32>().unwrap();
    let busses = lines[1].split(',').fold(Vec::new(), |mut agg, item| {
        match item.parse::<i32>() {
            Ok(n) => agg.push(n),
            _ => {}
        }
        agg
    });
    println!("{:?}", busses);
    let mut m = depart;
    let mut m_id = -1;
    for b in busses {
        let t = b - (depart % b);
        if t < m {
            m = t;
            m_id = b;
        }
    }
    m_id * m
}

fn part_2(contents: &String) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "939
7,13,x,x,59,x,31,19"
            .to_string();
        assert_eq!(super::part_1(&test_data), 295);
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
