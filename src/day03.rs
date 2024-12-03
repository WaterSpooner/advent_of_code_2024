use regex::Regex;

pub fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).map(|c| {
        let (_, [num1, num2,]) = c.extract();
        num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap()
    })
    .sum::<i32>()
    .to_string()
}

pub fn part2(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(don't\(\))|(do\(\))").unwrap();
    let mut valid = true;
    let mut total: i32 = 0;
    for captures in re.captures_iter(input){
        if let Some(mul1) = captures.get(1){
            total += if valid {captures.get(2).unwrap().as_str().parse::<i32>().unwrap() * mul1.as_str().parse::<i32>().unwrap()} else {0}
        } else if let Some(_) = captures.get(3) {
            valid = false;
        } else if let Some(_) = captures.get(4) {
            valid = true;
        }
    }
    total.to_string()
}