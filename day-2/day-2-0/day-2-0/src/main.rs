use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut counter = 0;

    // read in input.txt line by line
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if is_safe(&line.unwrap()) {
                // increment the counter
                counter += 1;
            }
        }
    }

    // return number of safe lines
    // e.g:
    // 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
    // 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    println!("Number of safe lines: {}", counter);
    return;
}

fn is_safe(line: &str) -> bool {
    // split the line into a vector of integers
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // check if levels are safe
    let mut increasing = true;
    let mut decreasing = true;
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }
    if !increasing && !decreasing {
        return false;
    }
    // if all the levels are safe, return true
    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
