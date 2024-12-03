use std::fs::read_to_string;

macro_rules! run_day {
    ($day:ident) => {
        fn $day() {
            use advent_of_code_2024::$day::*;
            if let Ok(input) = &read_to_string(concat!("inputs/", stringify!($day), ".txt")) {
                println!("{} part 1: {}", stringify!($day), part1(input));
                println!("{} part 2: {}", stringify!($day), part2(input));
                println!("=======================================================")
            }
        }
    };
}

run_day!(day01);
run_day!(day02);
run_day!(day03);

fn main() {
    day01();
    day02();
    day03();
}
