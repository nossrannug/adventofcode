use common::get_file_content;
use std::{collections::{HashSet}, env};
use std::hash::{
    Hash,
    BuildHasher,
};

/// Takes an arbitrary element from a `HashSet`, or None if empty
pub fn hashset_take_arbitrary<K, S> (
    set: &mut HashSet<K, S>,
) -> Option<K>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    let key_ref = {
        if let Some(key_ref) = set.iter().next() {
            /* must hide the origin of this borrow ... */
            unsafe { &*(key_ref as *const _) }
        } else {
            return None
        }
    };
    /* ... so that we may be able to mutably borrow the set here
       despite key_ref existence */
    set.take(key_ref)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().unwrap().to_string()).expect("Failed to open file");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> i64 {
    let mut numbers = get_numbers(contents);
    numbers.sort();
    let mut one = 0;
    let mut three = 1;
    let mut current = 0;
    for n in numbers {
        if n - current == 1 {
            one += 1;
        } else if n - current == 3 {
            three += 1;
        }
        current = n;
    }
    one * three
}
fn part_2(contents: &String) -> usize {
    1
    // let mut numbers = get_numbers(contents);
    // numbers.push(numbers.iter().max().unwrap()+3);
    // numbers.push(0);
    // numbers.sort();
    // let mut found = HashSet::new();
    // found.insert(numbers.clone());
    // let mut queue = vec![numbers.clone()];
    // loop {
    //     if queue.len() == 0 {
    //         break;
    //     }
    //     let process = queue.pop().unwrap();
    //     for i in 1..process.len()-1 {
    //         if process[i+1] - process[i-1] <= 3 {
    //             let mut v = process.clone();
    //             v.remove(i);
    //             if found.insert(v.clone()) {
    //                 queue.push(v.clone());
    //                 println!("q: {}", queue.len());
    //                 println!("f: {}", found.len());
    //             }
    //         }
    //     }
    // }
    // found.len()
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

    #[test]
    fn part_2_1() {
        let test_data = "1\n2\n3"
            .to_string();
        assert_eq!(super::part_2(&test_data), 4);
    }

}
