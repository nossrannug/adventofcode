use common::get_file_content;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> i64 {
    let mut numbers = get_numbers(contents);
    numbers.sort();
    // Count the number of gaps of 1 and gaps of 3
    let mut one = 0;
    let mut three = 1;
    let mut current = 0;
    for n in numbers {
        // There are only gaps of 1 and 3, no 2
        if n - current == 1 {
            one += 1;
        } else if n - current == 3 {
            three += 1;
        }
        current = n;
    }
    // Multiply the number of 1 and 3 gaps to get the answer
    one * three
}

fn part_2(contents: &String) -> i64 {
    let mut numbers = get_numbers(contents);
    // add the first 0 and the last (max + 3)
    numbers.push(0);
    numbers.push(numbers.iter().max().unwrap()+3);
    // sort
    numbers.sort();
    // Create array to count how many combinations can be created at each adapter
    // This allows us to keep track of how many paths there are without recalculating
    // each time
    let mut counter = numbers.iter().map(|_| 0).collect::<Vec<i64>>();
    // First adapter only has one combination
    counter[0] = 1;

    for a in 0..numbers.len() {
        for b in a+1..numbers.len() {
            // If the next adapter we look at is over 3 from the current we break
            if numbers[a] + 3 < numbers[b] {
                break;
            }
            // If we can get to this adapter from the current adapter
            // we can sum how many paths there are to this adapter + paths to the current
            counter[b] = counter[a] + counter[b];
        }
    }
    // Last item in the vector is the number of possible combinations of all
    // adapters in the bag
    counter[counter.len()-1]
}

fn get_numbers(contents: &String) -> Vec<i64> {
    contents.split('\n').map(|line| line.parse::<i64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let test_data = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            .to_string();
        assert_eq!(super::part_1(&test_data), 220);
    }

    #[test]
    fn part_2() {
        let test_data = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            .to_string();
        assert_eq!(super::part_2(&test_data), 19208);
    }
}
