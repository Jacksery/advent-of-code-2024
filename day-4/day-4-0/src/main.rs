use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Read the input file and build the grid
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let word = "XMAS";
    let word_len = word.len() as isize;
    let directions = vec![
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Down-Right
        (-1, 0),  // Up
        (0, -1),  // Left
        (-1, -1), // Up-Left
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
    ];

    let rows = grid.len() as isize;
    let cols = if rows > 0 { grid[0].len() as isize } else { 0 };
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut match_found = true;
                for k in 0..word_len {
                    let x = i + dx * k;
                    let y = j + dy * k;
                    if x < 0 || x >= rows || y < 0 || y >= cols {
                        match_found = false;
                        break;
                    }
                    if grid[x as usize][y as usize] != word.chars().nth(k as usize).unwrap() {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    count += 1;
                }
            }
        }
    }

    println!("Number of times 'XMAS' appears: {}", count);
    Ok(())
}