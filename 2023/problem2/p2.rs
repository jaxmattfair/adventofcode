use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // instantiate filename
    let filename = String::from("input.txt");

    println!("Part 1 Solution: {:?}", part1(filename.clone()));
    println!("Part 2 Solution: {:?}", part2(filename.clone()));
}

// part 1 of problem 2
fn part1(filename: String) -> i64 {
    let mut ans: i64 = 0;
    if let Ok(lines) = read_lines(filename.clone()) {

        for line in lines.flatten() {
            let l = String::from(line);
            
            // colors will keep track of most number of each color
            // The vector is in R G B form
            let mut colors: Vec<i64> = Vec::from([0, 0, 0]);

            // game ID processing
            let mut parts = l.split(":");
            let game_id = parts.nth(0).unwrap().split(" ").nth(1).unwrap().parse::<i64>().unwrap();

            // game block processing
            for blockpull in parts.next().unwrap().split(";") {
                for numcol in blockpull.split(",") {
                    let mut numcol_parts = numcol.split(" ");
                    let num = numcol_parts.nth(1).unwrap().parse::<i64>().unwrap();
                    let col = numcol_parts.nth(0).unwrap().parse::<String>().unwrap();  
                    match col.as_str() {
                        "red" => if num > colors[0] {colors[0] = num},
                        "green" => if num > colors[1] {colors[1] = num},
                        "blue" => if num > colors[2] {colors[2] = num},
                        _ => println!("{col} is not RBG"),
                    }
                }
            }

            if colors[0] <= 12 && colors[1] <= 13 && colors[2] <= 14 {
                ans += game_id;
            }

        }
    }
    ans
}

// part 2 of problem 2
fn part2(filename: String) -> i64 {
    let mut ans: i64 = 0;
    if let Ok(lines) = read_lines(filename.clone()) {

        for line in lines.flatten() {
            let l = String::from(line);
            
            // colors will keep track of most number of each color
            // The vector is in R G B form
            let mut colors: Vec<i64> = Vec::from([0, 0, 0]);

            // game ID processing
            let mut parts = l.split(":");
            let _game_id = parts.nth(0).unwrap().split(" ").nth(1).unwrap().parse::<i64>().unwrap();

            // game block processing
            for blockpull in parts.next().unwrap().split(";") {
                for numcol in blockpull.split(",") {
                    let mut numcol_parts = numcol.split(" ");
                    let num = numcol_parts.nth(1).unwrap().parse::<i64>().unwrap();
                    let col = numcol_parts.nth(0).unwrap().parse::<String>().unwrap();  
                    match col.as_str() {
                        "red" => if num > colors[0] {colors[0] = num},
                        "green" => if num > colors[1] {colors[1] = num},
                        "blue" => if num > colors[2] {colors[2] = num},
                        _ => println!("{col} is not RBG"),
                    }
                }
            }
            ans += colors[0] * colors[1] * colors[2];
        }
    }
    ans
}

// utility for reading each line of file iteratively
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}