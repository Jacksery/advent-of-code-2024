use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read in input.txt
    if let Ok(lines) = read_lines("./input.txt") {
        // Split into two columns that are separated by whitespace
        let mut col1 = Vec::new();
        let mut col2 = Vec::new();

        for line in lines.flatten() {
            let columns: Vec<&str> = line.split_whitespace().collect();
            if columns.len() >= 2 {
                col1.push(columns[0].parse::<i32>().unwrap());
                col2.push(columns[1].parse::<i32>().unwrap());
            }
        }

        // Create a HashMap to count occurrences of each number in col2
        let mut counts = HashMap::new();
        for &num in &col2 {
            *counts.entry(num).or_insert(0) += 1;
        }

        // Calculate the sum of the number of occurrences of each left column number in the right column
        let mut sum = 0;
        for &num in &col1 {
            if let Some(&count) = counts.get(&num) {
                sum += num * count;
            }
        }

        println!("Sum of occurrences: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
