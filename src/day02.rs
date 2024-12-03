fn parse_input(input: &str) -> Vec<Vec<i32>>{
    input.split("\n")
    .map(|line: &str| {
        line.split_whitespace()
        .filter_map(|str_num| str_num.parse().ok())
        .collect()
    }).collect()
}

pub fn part1(input: &str) -> String {
    let nums = parse_input(input);
    valid(&nums).to_string()
    }

pub fn valid(nums: &Vec<Vec<i32>>) -> i32{
    nums.iter().filter(|line| {
        let diffs = line.windows(2).map(|w| w[1] - w[0]);
        diffs.clone().all(|x| x > 0 && x <= 3) || diffs.clone().all(|x| x < 0 && x >= -3)
    }).count() as i32
}


pub fn part2(input: &str) -> String {
    let mut count = 0;
    let nums = parse_input(input);
    for line in nums {
        let mut permuations : Vec<Vec<i32>> = vec![line.clone()];
        for i in 0..line.len(){
            let mut subset = line.clone();
            subset.remove(i);
            permuations.push(subset);
        }
        count += if valid(&permuations)!= 0 {1} else {0}

    }
    count.to_string()
}