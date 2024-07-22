use std::fs::read_to_string;

fn main() {
    // read input file
    let filename = String::from("input.txt");
    let binding = read_to_string(filename.clone()).unwrap();
    let input = binding.lines().collect::<Vec<&str>>();

    println!("Part 1 Solution: {:?}", part1(input.clone()));
    println!("Part 2 Solution: {:?}", part2(input.clone()));
}

fn part1(input: Vec<&str>) -> i64 {
    let mut sum = 0;

    let num_rows = input.len();
    let num_cols = input[0].len();

    // iterate through each row
    for row in 0..num_rows {
        let mut num = "".to_string();
        let mut surrounding: Vec<(usize, usize)> = Vec::new();

        // iterate through each column
        for col in 0..num_cols {
            let c = input[row].chars().nth(col).unwrap();
            if c.is_numeric() {
                num.push(c);

                // add valid surrounding indices to vector
                surrounding.append(&mut get_surrounding(row, col, num_rows, num_cols));

                // if the next character is a digit, continue
                if col < num_cols - 1 && input[row].chars().nth(col + 1).unwrap().is_digit(10) {
                    continue;
                }
                // otherwise, if all surrounding values are '.' or digits, clear
                else if surrounding.iter().all(|(r, c)| input[*r].chars().nth(*c).unwrap() == '.' 
                                                     || input[*r].chars().nth(*c).unwrap().is_digit(10)) {
                    num.clear();
                    surrounding.clear();
                    continue;
                }
                // else add to sum and clear
                else {
                    sum += num.parse::<i64>().unwrap();
                    num.clear();
                    surrounding.clear();
                    continue;
                } 
                
            }
        }
    }
    sum
}

fn part2(input: Vec<&str>) -> i64 {
    let num_rows = input.len();
    let num_cols = input[0].len();
    let mut sum = 0;

    // initialize vectors to store numbers, symbols, and their positions
    // nums_pos is a vector of tuples structured (number, vector of positions)
    // symbols_pos is a vector of positions for symbols
    let mut nums_pos: Vec<(i64, Vec<(usize, usize)>)> = Vec::new();
    let mut symbols_pos: Vec<(usize, usize)> = Vec::new();

    // iterate through each row
    for row in 0..num_rows {
        let mut num = "".to_string();
        let mut surrounding: Vec<(usize, usize)> = Vec::new();

        // iterate through each column
        for col in 0..num_cols {
            let c = input[row].chars().nth(col).unwrap();
            
            if c == '*' {
                symbols_pos.push((row,col));
            } else if c.is_numeric() {
                num.push(c);

                // add valid surrounding indices to vector
                surrounding.append(&mut get_surrounding(row, col, num_rows, num_cols));

                // if the next character is a digit, continue
                if col < num_cols - 1 && input[row].chars().nth(col + 1).unwrap().is_digit(10) {
                    continue;
                }
                // otherwise, if all surrounding values are not '*', reset
                else if surrounding.iter().all(|(r, c)| input[*r].chars().nth(*c).unwrap() != '*') {
                    num.clear();
                    surrounding.clear();
                    continue;
                }
                // else append  to nums_vec and clear
                else {
                    nums_pos.push((num.parse::<i64>().unwrap(), surrounding.clone()));
                    num.clear();
                    surrounding.clear();
                    continue;
                }
            }
        }
    }

    // for each symbol, if there are two numbers adjacent to it, multiply them and add to sum
    for (r, c) in symbols_pos {
        let mut nums = Vec::new();
        for (num, surrounding) in &nums_pos {
            if surrounding.contains(&(r, c)) {
                nums.push(*num);
            }
        }
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }

    sum
}

fn get_surrounding(row: usize, col: usize, num_rows: usize, num_cols: usize) -> Vec<(usize, usize)> {
    let mut surrounding: Vec<(usize, usize)> = Vec::new();
                // get grid of existent surrounding indices (including diagonals)
                if row > 0 {
                    surrounding.push((row - 1, col));
                }
                if row < num_rows - 1 {
                    surrounding.push((row + 1, col));
                }
                if col > 0 {
                    surrounding.push((row, col - 1));
                }
                if col < num_cols - 1 {
                    surrounding.push((row, col + 1));
                }
                if row > 0 && col > 0 {
                    surrounding.push((row - 1, col - 1));
                }
                if row > 0 && col < num_cols - 1 {
                    surrounding.push((row - 1, col + 1));
                }
                if row < num_rows - 1 && col > 0 {
                    surrounding.push((row + 1, col - 1));
                }
                if row < num_rows - 1 && col < num_cols - 1 {
                    surrounding.push((row + 1, col + 1));
                }
    surrounding
}