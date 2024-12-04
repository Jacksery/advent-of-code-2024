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

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            // Down-Right diagonal
            let dr = [grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]];
            // Up-Right diagonal
            let ur = [grid[i + 1][j - 1], grid[i][j], grid[i - 1][j + 1]];

            let dr_str: String = dr.iter().collect();
            let ur_str: String = ur.iter().collect();

            let valid_dr = dr_str == "MAS" || dr_str == "SAM";
            let valid_ur = ur_str == "MAS" || ur_str == "SAM";

            if valid_dr && valid_ur {
                count += 1;
            }
        }
    }

    println!("Number of times 'X-MAS' appears: {}", count);
    Ok(())
}
