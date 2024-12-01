use std::fs::read_to_string;

fn main(){
    day01()
}

fn day01(){
    use advent_of_code_2024::day01::*;
    if let Ok(input) = &read_to_string("inputs/day01.txt"){
        println!("day01 part 1: {}",part1(input));
        println!("day01 part 2: {}",part2(input));
    }
}