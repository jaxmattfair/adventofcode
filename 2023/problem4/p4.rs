use std::fs::read_to_string;

fn main() {
    // read input file
    let filename = String::from("input.txt");
    let binding = read_to_string(filename.clone()).unwrap();
    let input = binding.lines().collect::<Vec<&str>>();

    println!("Part 1 Solution: {:?}", part1(input.clone()));
    // println!("Part 2 Solution: {:?}", part2(input.clone()));
}

fn part1(lines: Vec<&str>) -> i64 {
    let mut score = 0;
    for line in lines {
        let sides = line.split(":")
                        .nth(1)
                        .unwrap()
                        .split("|")
                        .collect::<Vec<&str>>()
                        .iter().map(|x| 
                                        x.trim()
                                         .split(" ")
                                         .collect::<Vec<&str>>())
                                .map(|x| 
                                        x.iter()
                                         .filter(|&y| y != &"")
                                         .map(|y| y.parse::<i64>().unwrap())
                                         .collect::<Vec<i64>>())
                                .collect::<Vec<Vec<i64>>>();
        score += sides[0].iter()
            .flat_map(|&x| sides[1].iter().map(move |&y| (x, y)))
            .filter(|&(x, y)| x == y)
            .enumerate()
            .fold(0, |score, (index, _)| {
                if index == 0 {
                    score + 1
                } else {
                    score * 2
                }
            });
    }
    score
}

// fn part2(lines: Vec<&str>) -> i64 {
//     0
// }