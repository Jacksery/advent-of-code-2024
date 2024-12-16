use std::collections::{HashMap, HashSet};

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // Group towers by frequency
    let mut towers_by_freq: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                towers_by_freq
                    .entry(c)
                    .or_default()
                    .push((x as i32, y as i32));
            }
        }
    }

    // Dimensions of the input grid
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;

    // Set to store unique antinode positions
    let mut antinodes = HashSet::new();

    // Calculate antinodes
    for positions in towers_by_freq.values() {
        // Only consider frequencies with at least two antennas
        if positions.len() >= 2 {
            // Add the antennas themselves as antinodes
            for &(x, y) in positions {
                antinodes.insert((x, y));
            }

            // For each pair of antennas, find all positions colinear with them
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    // Calculate direction vector
                    let dx = x2 - x1;
                    let dy = y2 - y1;

                    // Compute the greatest common divisor to step through grid points
                    let gcd = gcd(dx.abs(), dy.abs());

                    // Normalize direction vector
                    let step_x = dx / gcd;
                    let step_y = dy / gcd;

                    // Traverse in both directions along the line
                    let mut x = x1;
                    let mut y = y1;
                    // Extend in the positive direction
                    while x >= 0 && x < width && y >= 0 && y < height {
                        antinodes.insert((x, y));
                        x += step_x;
                        y += step_y;
                    }

                    // Reset to starting point
                    x = x1;
                    y = y1;
                    // Extend in the negative direction
                    while x >= 0 && x < width && y >= 0 && y < height {
                        antinodes.insert((x, y));
                        x -= step_x;
                        y -= step_y;
                    }
                }
            }
        }
    }

    // Output the result
    println!("Total unique antinode locations: {}", antinodes.len());
}

// Function to compute the Greatest Common Divisor
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}
