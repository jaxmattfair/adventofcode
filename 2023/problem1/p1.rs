use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // instantiate filename
    let filename = String::from("input.txt");

    println!("Part 1 Solution: {:?}", part1(filename.clone()));
    println!("Part 2 Solution: {:?}", part2(filename.clone()));
}

// part 1 of problem 1
fn part1(filename: String) -> i64 {
    let mut ans: i64 = 0;
    if let Ok(lines) = read_lines(filename.clone()) {
        for line in lines.flatten() {
            let mut l = String::from(line);
            l.retain(|c| c.is_numeric());
            ans += format!("{}{}", l.chars().next().unwrap(), 
                                   l.chars().last().unwrap())
                    .parse::<i64>().unwrap();
        }
    }
    ans
}

// part 2 of problem 1
fn part2(filename: String) -> i64 {
    // create mapping for values
    // the mapping has wrapping letters for cases like eighthree => 83
    // single-letter wrapping is all that's needed due to no further overlap between
    // words one through nine
    let mut rust_map = HashMap::new();
    rust_map.insert("one", "o1w");
    rust_map.insert("two", "t2o");
    rust_map.insert("three", "t3e");
    rust_map.insert("four", "f4r");
    rust_map.insert("five", "f5e");
    rust_map.insert("six", "s6x");
    rust_map.insert("seven", "s7n");
    rust_map.insert("eight", "e8t");
    rust_map.insert("nine", "n9e");

    let mut ans: i64 = 0;
    if let Ok(lines) = read_lines(filename.clone()) {
        for line in lines.flatten() {
            let mut l = String::from(line);
            
            // make numeric replacements
            for (word, num) in &rust_map {
                l = l.replace(word, num);
            }

            l.retain(|c| c.is_numeric());
            ans += format!("{}{}", l.chars().next().unwrap(), 
                                   l.chars().last().unwrap())
                    .parse::<i64>().unwrap();
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