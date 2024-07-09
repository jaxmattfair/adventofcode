use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // instantiate filename
    let filename = String::from("input.txt");

    // println!("Part 1 Solution: {:?}", part1(filename.clone()));
    println!("Part 2 Solution: {:?}", part2(filename.clone()));
}

// part 1 of problem 1
fn part1(filename: String) -> i64 {
    let mut ans: i64 = 0;
    if let Ok(lines) = read_lines(filename.clone()) {
        for line in lines.flatten() {
            let mut l = String::from(line);
            l.retain(|c| c.is_numeric());
            ans += format!("{}{}", l.chars().next().unwrap(), l.chars().last().unwrap()).parse::<i64>().unwrap();
        }
    }
    ans
}

// part 2 of problem 1
fn part2(filename: String) -> i64 {
    // create mapping for values
    let mut rust_map = HashMap::new();
    rust_map.insert("one", "1");
    rust_map.insert("two", "2");
    rust_map.insert("three", "3");
    rust_map.insert("four", "4");
    rust_map.insert("five", "5");
    rust_map.insert("six", "6");
    rust_map.insert("seven", "7");
    rust_map.insert("eight", "8");
    rust_map.insert("nine", "9");

    // sum values
    let mut ans: i64 = 0;
    if let Ok(lines) = read_lines(filename.clone()) {
        for line in lines.flatten() {
            let mut l = String::from(line);

            print!("{:?}: ", l);
            // println!("Line before: {:?}", l);
            for (word, num) in &rust_map {
                l = l.replace(word, num);
            }
            // println!("Line after: {:?}", l);

            l.retain(|c| c.is_numeric());
            let num = format!("{}{}", l.chars().next().unwrap(), 
                                      l.chars().last().unwrap())
                        .parse::<i64>().unwrap();
            println!("{:?}", num);
            ans += num;
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