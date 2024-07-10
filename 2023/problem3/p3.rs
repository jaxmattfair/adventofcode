use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // instantiate filename
    let filename = String::from("test.txt");
    
    if let Ok(lines) = read_lines(filename.clone()) {
        for line in lines.flatten() {
            let mut l = String::from(line);
            println!("{l}");
        }
    }

    
    // println!("Part 1 Solution: {:?}", part1(filename.clone()));
    // println!("Part 2 Solution: {:?}", part2(filename.clone()));
}

// fn part1(filename: String) -> i64 {-1}

// fn part2(filename: String) -> i64 {-1}

// utility for reading each line of file iteratively
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}