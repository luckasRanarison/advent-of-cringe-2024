use std::{cmp::Reverse, collections::BinaryHeap};

pub fn calculate_distance(input: &str) -> usize {
    let mut result = 0;
    let mut left_pq = BinaryHeap::new();
    let mut right_pq = BinaryHeap::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let left = iter.next().unwrap().parse::<usize>().unwrap();
        let right = iter.next().unwrap().parse::<usize>().unwrap();

        left_pq.push(Reverse(left));
        right_pq.push(Reverse(right));
    }

    while let Some((left, right)) = left_pq.pop().zip(right_pq.pop()) {
        result += left.0.abs_diff(right.0);
    }

    result
}
