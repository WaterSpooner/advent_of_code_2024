fn to_grid(input: &str) -> Vec<Vec<char>>{
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: &str) -> String {
    let grid = to_grid(input);
    let word_chars: Vec<char> = "XMAS".to_string().chars().collect();
    let word_len = word_chars.len();
    let columns = grid[0].len() as isize;
    let rows = grid.len() as isize;
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut count = 0;

    for row in 0..rows {
        for col in 0..columns {
            for &(row_offset, col_offset) in &directions {
                let mut valid = true;
                for i in 0..word_len {
                    let r = row + i as isize * row_offset;
                    let c = col + i as isize * col_offset;
                    if !(0..rows).contains(&r) || !(0..columns).contains(&c) || grid[r as usize][c as usize] != word_chars[i]{
                        valid = false;
                        break;
                    }
                }
                if valid {count +=1;}
            }
        }
    }
   count.to_string()  
}

pub fn part2(input: &str) -> String {
    let grid = to_grid(input);
    let mut count = 0;
    let columns = grid[0].len();
    let rows = grid.len();
    for row in 1..(rows-1){
        for col in 1..(columns-1){
            if grid[row as usize][col as usize] == 'A' {
                let diag_1 = vec![grid[row - 1][col - 1], grid[row + 1][col + 1]];
                let diag_2 = vec![grid[row - 1][col + 1], grid[row + 1][col - 1]];
                if (diag_1 == ['S', 'M'] || diag_1 == ['M','S']) && (diag_2 == ['S', 'M'] || diag_2 == ['M','S']) {count+=1;}

            }
        } 
    }
    count.to_string()
}