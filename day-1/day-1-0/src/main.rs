use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read in input.txt
    if let Ok(lines) = read_lines("./input.txt") {
        let mut col1 = Vec::new();
        let mut col2 = Vec::new();

        for line in lines.flatten() {
            let columns: Vec<&str> = line.split_whitespace().collect();
            if columns.len() >= 2 {
                col1.push(columns[0].parse::<i32>().unwrap());
                col2.push(columns[1].parse::<i32>().unwrap());
            }
        }

        // Sort the columns
        col1.sort();
        col2.sort();

        // Calculate the sum of differences
        let sum_of_differences: i32 = col1
            .iter()
            .zip(col2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        println!("Sum of differences: {}", sum_of_differences);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
