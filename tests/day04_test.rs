use advent_of_code_2024::day04::*;

const INPUT: &str = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";


#[test]
fn part1_test() {
    assert_eq!(part1(INPUT), "18");
}


#[test]
fn part2_test() {
    assert_eq!(part2(INPUT), "9");
}