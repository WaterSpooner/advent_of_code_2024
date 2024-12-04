use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>,Vec<i32>){
    input.lines()
    .map(|line: &str| {
        let nums: Vec<i32> = line.split_whitespace()
        .filter_map(|str_num| str_num.parse().ok())
        .collect();
        (nums[0],nums[1])
    })
    .unzip()
}

pub fn part1(input: &str) -> String {
    let (left, right) = parse_input(input);
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    left_sorted.sort();
    right_sorted.sort();
    left_sorted.iter()
                .zip(right_sorted)
                .map(|(x,y)| (x-y).abs())
                .sum::<i32>()
                .to_string()
}

pub fn part2(input: &str) -> String {
    let (left, right) = parse_input(input);
    let mut freq = HashMap::new();
    right.iter().for_each(|x| *freq.entry(x).or_insert(0) += 1);
    left.iter().map(|y| *freq.entry(y).or_insert(0) * y).sum::<i32>().to_string()
}