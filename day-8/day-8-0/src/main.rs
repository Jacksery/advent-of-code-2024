use std::collections::{HashMap, HashSet};

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // Group towers by frequency
    let mut towers_by_freq = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                towers_by_freq
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    // Dimensions of the input grid
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;

    // Calculate antinodes
    let mut antinodes = HashSet::new();
    for positions in towers_by_freq.values() {
        for (i, &(x1, y1)) in positions.iter().enumerate() {
            for &(x2, y2) in &positions[i + 1..] {
                // Antinode positions
                let antinode1 = (2 * x2 - x1, 2 * y2 - y1);
                let antinode2 = (2 * x1 - x2, 2 * y1 - y2);

                // Check bounds and insert
                for &(x, y) in &[antinode1, antinode2] {
                    if (0..width).contains(&x) && (0..height).contains(&y) {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
    }

    // Output the result
    println!("Total unique antinode locations: {}", antinodes.len());
}
