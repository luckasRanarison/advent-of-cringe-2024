use advent_of_cringe::day_1::{part_one::calculate_distance, part_two::calculate_similarity};

const INPUT: &str = include_str!("../../input/day_1.txt");

fn main() {
    println!("--- Part one ---");
    println!("Distance: {}", calculate_distance(INPUT)); // 2057374
    println!("--- Part two ---");
    println!("Similarity: {}", calculate_similarity(INPUT)); // 23177084
}
