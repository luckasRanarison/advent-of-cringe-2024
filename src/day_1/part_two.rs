use std::collections::HashMap;

pub fn calculate_similarity(input: &str) -> usize {
    let mut result = 0;
    let mut right_count = HashMap::<usize, usize>::new();
    let mut left_count = HashMap::<usize, usize>::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let left = iter.next().unwrap().parse::<usize>().unwrap();
        let right = iter.next().unwrap().parse::<usize>().unwrap();

        *left_count.entry(left).or_default() += 1;
        *right_count.entry(right).or_default() += 1;
    }

    for (number, right_count) in right_count {
        if let Some(&left_count) = left_count.get(&number) {
            result += number * right_count * left_count;
        }
    }

    result
}
