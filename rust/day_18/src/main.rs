use common::get_file_content;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn parse_string(s: &str) -> i64 {
    let re = Regex::new(r"\(([^\(\)]*)\)").unwrap();
    let mut text = s.clone().to_string();
    while re.is_match(&text.clone()) {
        for cap in re.captures_iter(&text.clone()) {
            let n = parse_string(&cap[1]);
            text = text
                .replacen(&cap[0], n.to_string().as_str(), 1)
                .to_string();
        }
    }
    let mut n = 0i64;
    let mut operator = "+";
    for c in text.trim().split(' ') {
        if let Ok(v) = c.parse::<i64>() {
            match operator {
                "+" => n += v,
                "*" => n *= v,
                _ => panic!("ARGH!!!"),
            }
        } else {
            operator = c;
        }
    }
    n
}

fn parse_string_2(s: &str) -> i64 {
    let re = Regex::new(r"\(([^\(\)]*)\)").unwrap();
    let mut text = s.clone().to_string();
    while re.is_match(&text.clone()) {
        for cap in re.captures_iter(&text.clone()) {
            let n = parse_string_2(&cap[1]);
            text = text
                .replacen(&cap[0], n.to_string().as_str(), 1)
                .to_string();
        }
    }

    text.split(" * ").fold(1, |agg, t| {
        let mut n = 0i64;
        let mut operator = "+";
        for c in t.trim().split(' ') {
            if let Ok(v) = c.parse::<i64>() {
                match operator {
                    "+" => n += v,
                    _ => panic!("ARGH!!!"),
                }
            } else {
                operator = c;
            }
        }
        agg * n
    })
}

fn part_1(contents: &String) -> i64 {
    contents.split('\n').map(|line| parse_string(line)).sum()
}

fn part_2(contents: &String) -> i64 {
    contents.split('\n').map(|line| parse_string_2(line)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_string() {
        assert_eq!(super::parse_string("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(super::parse_string("2 * 3 + (4 * 5)"), 26);
        assert_eq!(super::parse_string("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            super::parse_string("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
        assert_eq!(
            super::parse_string("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(super::parse_string("(5 * (5 * 2 + 4 * 4 * 5 + 2) * 9 + (2 + 6 * 4 * 2 + 3) * 6) * 4 + ((3 * 7 + 3) + 7) * 8 * 6 * 2"), 29395104);
    }

    #[test]
    fn test_parse_string_2() {
        assert_eq!(super::parse_string_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(super::parse_string_2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(super::parse_string_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            super::parse_string_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
        assert_eq!(
            super::parse_string_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
    }

    #[test]
    fn part_1() {
        let test_data = "2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
            .to_string();
        assert_eq!(super::part_1(&test_data), 26 + 437 + 13632 + 12240);
    }

    #[test]
    fn part_2() {
        let test_data = "2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string();
        assert_eq!(super::part_2(&test_data), 46 + 1445 + 23340 + 669060);
    }
}
