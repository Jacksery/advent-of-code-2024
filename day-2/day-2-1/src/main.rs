use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut counter = 0;

    // read in input.txt line by line
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if is_safe(&line) {
                    // increment the counter
                    counter += 1;
                }
            }
        }
    }

    // return number of safe lines
    println!("Number of safe lines: {}", counter);
}

fn is_safe(line: &str) -> bool {
    let levels: Vec<i32> = match line.split_whitespace().map(|x| x.parse()).collect() {
        Ok(levels) => levels,
        Err(_) => return false,
    };

    if check_levels(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        if check_levels(
            &levels
                .iter()
                .enumerate()
                .filter_map(|(j, &x)| if i == j { None } else { Some(x) })
                .collect(),
        ) {
            return true;
        }
    }

    false
}

fn check_levels(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return true;
    }

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

    increasing || decreasing
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
